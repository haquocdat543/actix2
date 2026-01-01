# syntax=docker/dockerfile:1.6

FROM rust:1.89 AS builder

ENV CARGO_BUILD_JOBS=8
ENV RUSTC_WRAPPER=""

RUN rustup target add aarch64-unknown-linux-musl

RUN apt-get update -y && apt-get install -y \
	musl-tools \
	libssl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	cargo fetch

COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	cargo build \
	--bin app \
	--release \
	--target aarch64-unknown-linux-musl

FROM scratch

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/app /app

CMD ["/app"]
