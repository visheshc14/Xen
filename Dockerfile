# Use nightly Rust as the base image
FROM rustlang/rust:nightly AS builder

# Set the working directory
WORKDIR /app

# Add the musl target for Rust
RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

# Install Node.js 18 (includes npm by default) and Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g yarn \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency files first (for caching)
COPY Cargo.toml Cargo.lock package.json yarn.lock ./

# Install Yarn dependencies
RUN yarn install

# Copy the rest of the application code
COPY . .

# Build the frontend
RUN yarn build

# Document that the container listens on port 8000
EXPOSE 8000

# Build the Rust project in release mode
RUN cargo build --release

# Use a non-root user
USER 1000

# Run the compiled binary
CMD ["./target/release/xen"]