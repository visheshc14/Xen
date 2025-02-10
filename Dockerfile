# Use Rust with nightly support
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Install required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    musl-tools \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust nightly toolchain & target
RUN rustup install nightly && rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

# Install Node.js and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y nodejs npm && \
    npm i -g yarn

# Copy only dependency files first (improves caching)
COPY Cargo.toml Cargo.lock Rocket.toml RustConfig package.json yarn.lock ./

# Install Yarn dependencies
RUN yarn install --frozen-lockfile

# Copy the rest of the project
COPY . .

# Build frontend assets
RUN yarn build

# Build Rust project
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use minimal runtime image
FROM debian:bullseye-slim AS runtime

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen /usr/local/bin/Xen

# Use non-root user
USER 1000

# Expose Rocket default port
EXPOSE 8000

# Start application
CMD ["/usr/local/bin/Xen"]
