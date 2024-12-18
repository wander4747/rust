FROM rust:alpine

RUN apk add --no-cache musl-dev openssl-dev

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY . .

CMD ["cargo-watch", "-x", "run"]
