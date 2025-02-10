# Use Rust nightly as base image
FROM rustlang/rust:nightly AS builder

# Set working directory
WORKDIR /app

# Install required Rust toolchain components
RUN rustup install nightly \
    && rustup default nightly \
    && rustup target add x86_64-unknown-linux-musl

# Install required dependencies
RUN apt update && apt install -y --no-install-recommends \
    musl-tools curl ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt install -y nodejs npm \
    && npm i -g yarn \
    && rm -rf /var/lib/apt/lists/*

# Copy dependencies separately for caching
COPY Cargo.toml Cargo.lock Rocket.toml RustConfig package.json yarn.lock ./

# Install Rust dependencies
RUN cargo fetch

# Install Node.js dependencies
RUN yarn install --frozen-lockfile

# Copy source code
COPY . .

# Build Node.js project
RUN yarn build

# Build Rust project
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Create a minimal runtime image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Set working directory
WORKDIR /app

# Copy compiled Rust binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen .

# Copy built frontend assets if needed
COPY --from=builder /app/public ./public

# Create a non-root user and set permissions
RUN addgroup --system appgroup && adduser --system --group appuser \
    && chown -R appuser:appgroup /app

# Use non-root user
USER appuser

# Expose application port
EXPOSE 8000

# Start application
CMD ["./Xen"]
