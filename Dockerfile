# Step 1: Build stage
FROM rust:1.89 AS builder

ENV RUSTC_WRAPPER=""

RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /usr/src/app

RUN apt-get update -y \
	&& apt-get install -y \
	musl-tools

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --bin app \
	--release \
	--target aarch64-unknown-linux-musl

FROM alpine:latest

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/app /usr/local/bin/app

CMD ["app"]
