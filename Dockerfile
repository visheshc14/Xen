# Use a stable Rust version instead of nightly
FROM rust:1.65 AS builder

WORKDIR /app

# Add Rust target
RUN rustup target add x86_64-unknown-linux-musl

# Install Node.js and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - \
    && apt-get update \
    && apt-get install -y nodejs npm \
    && npm install -g yarn

# Copy necessary files
COPY Cargo.toml Cargo.lock Rocket.toml package.json ./

# Update dependencies to ensure compatibility
RUN cargo update

# Install Yarn dependencies
RUN yarn install --frozen-lockfile

# Copy the remaining files
COPY . .

# Build Yarn project
RUN yarn build

# Build Rust project in release mode with MUSL target for a static binary
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Use a minimal runtime image
FROM alpine:latest
WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the Rust binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/Xen .

# Change to non-root user for security
USER 1000

# Run the application
CMD ["./Xen"]
