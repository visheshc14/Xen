# Use Rust for Rocket
FROM rust:bullseye AS builder

WORKDIR /app

# Install dependencies
RUN apt update && apt install -y curl gcc g++ musl-tools pkg-config libssl-dev

# Set Rust toolchain
RUN rustup target add x86_64-unknown-linux-musl

# Install Node.js & Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y nodejs npm
RUN npm install -g yarn

# Copy necessary files
COPY Cargo.toml Rocket.toml package.json ./

# Fetch Rust dependencies
RUN cargo fetch

# Install Yarn dependencies
RUN yarn install

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
RUN apt update && apt install -y libssl-dev ca-certificates

# Copy built binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen /app/Xen

# Use non-root user
USER 1000

# Expose application port
EXPOSE 8000

# Run the compiled binary
CMD ["/app/Xen"]
