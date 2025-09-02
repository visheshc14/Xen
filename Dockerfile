# Use nightly Rust as the base image
FROM rustlang/rust:nightly AS builder

# Set the working directory
WORKDIR /app

# Add the musl target for Rust
RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

# Install Node.js and npm
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
    apt -y install nodejs npm

# Install Yarn globally
RUN npm i -g yarn

# Copy dependency files
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
