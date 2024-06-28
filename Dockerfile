FROM rustlang/rust:nightly AS builder

WORKDIR /app

# Add Rust target
RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

# Install Node.js and Yarn
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt -y install nodejs npm
RUN npm i -g yarn

# Copy necessary files
COPY Cargo.toml Rocket.toml RustConfig package.json ./

# Install Yarn dependencies
RUN yarn install

# Copy the remaining files
COPY . .

# Build Yarn project
RUN yarn build

# Build Rust project
RUN cargo build --release 

# Change to non-root user
USER 1000

CMD ["cd ./target/release", "./Xen"]
