FROM rustlang/rust:nightly AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl --toolchain=nightly

RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -

RUN apt -y install nodejs npm

RUN npm i -g yarn

COPY Cargo.toml Cargo.lock package.json yarn.lock ./

RUN yarn install

COPY . .

RUN yarn build

RUN cargo build --release 

USER 1000

CMD ["./target/release/Xen"]
