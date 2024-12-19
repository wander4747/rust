FROM rust:alpine

RUN apk add --no-cache musl-dev openssl-dev

RUN cargo install

WORKDIR /usr/src/app

COPY . .