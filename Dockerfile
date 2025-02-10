# Stage 1: Build
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Install necessary tools
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    musl-tools \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set Rust to stable and add the musl target
RUN rustup install stable && rustup default stable
RUN rustup target add x86_64-unknown-linux-musl

# Install Node.js and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get install -y nodejs npm \
    && npm install -g yarn

# Copy Rust and Node.js dependencies
COPY Cargo.toml Cargo.lock Rocket.toml RustConfig package.json ./

# Ensure we have a valid target in Cargo.toml
RUN grep -q '\[\[bin\]\]' Cargo.toml || grep -q '\[lib\]' Cargo.toml || (echo 'Error: Cargo.toml has no valid targets' && exit 1)

# Fetch Rust dependencies
RUN cargo fetch || true  # Ignore fetch errors if Cargo.lock is missing

# Install Yarn dependencies
RUN yarn install --frozen-lockfile

# Copy remaining source code
COPY . .

# Build frontend
RUN yarn build

# Build Rust binary with musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Stage 2: Create Minimal Runtime Image
FROM alpine:latest

WORKDIR /app

# Install required runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the compiled Rust binary from the builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen ./Xen

# Set executable permissions
RUN chmod +x ./Xen

# Switch to non-root user for security
USER 1000

# Run the application
CMD ["./Xen"]
