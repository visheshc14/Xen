# Use official Rust image as base
FROM rust:1.65 as builder

# Set working directory
WORKDIR /app

# Install necessary dependencies
RUN rustup target add x86_64-unknown-linux-musl

# Install latest Node.js LTS (v20) and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs npm \
    && npm install -g yarn \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy project files
COPY . .

# Build the Rust application
RUN cargo build --release

# Create a minimal final image
FROM alpine:latest  

# Install required runtime dependencies
RUN apk add --no-cache ca-certificates

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder stage
COPY --from=builder /app/target/release/xen /usr/local/bin/xen

# Expose application port
EXPOSE 8000

# Set the entry point for the container
CMD ["xen"]
