# Use Rust for Rocket (builder stage)
FROM rust:bullseye AS builder

WORKDIR /app

# Install system dependencies
RUN apt update && apt install -y \
    curl \
    gcc \
    g++ \
    musl-tools \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set Rust toolchain for musl target
RUN rustup target add x86_64-unknown-linux-musl

# Install Node.js & Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt install -y nodejs \
    && npm install -g yarn

# Copy dependency files
COPY Cargo.toml Cargo.lock Rocket.toml package.json yarn.lock ./

# Fetch Rust dependencies (cache layer)
RUN cargo fetch

# Install Yarn dependencies (cache layer)
RUN yarn install --frozen-lockfile

# Copy all project files
COPY . .

# Build frontend
RUN yarn build

# Compile Rust project
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use minimal runtime image
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt update && apt install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen /app/Xen

# Use non-root user
RUN useradd -m appuser
USER appuser

# Expose application port
EXPOSE 8000

# Run the compiled binary
CMD ["/app/Xen"]
