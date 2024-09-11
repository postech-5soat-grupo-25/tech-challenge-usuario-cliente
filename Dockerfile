FROM rust:1.79-alpine3.20 AS builder

# Feature apenas em nightly
RUN rustup update nightly 

RUN rustup default nightly

# Definindo a pasta dentro do container onde
WORKDIR /usr/src/tech-challenge

# Install necessary packages for building Rust applications and OpenSSL development headers
RUN apk add musl-dev openssl-dev

COPY . .

RUN apk --update add ca-certificates

RUN cargo build --release

RUN cargo build --release
ENTRYPOINT [ "/usr/src/tech-challenge/target/release/api" ]
EXPOSE 3000