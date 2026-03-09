# syntax=docker/dockerfile:1.6

FROM rust:1.89-slim AS base

RUN apt-get update -y && apt-get install -y \
	build-essential \
	musl-tools \
	pkg-config \
	upx \
	&& rm -rf /var/lib/apt/lists/*

RUN rustup target add aarch64-unknown-linux-musl

RUN \
	# --mount=type=cache,target=/usr/local/cargo/registry \
	cargo install cargo-chef --locked

ENV \
	RUSTC_WRAPPER="" \
	CARGO_INCREMENTAL=0

FROM base AS deps
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs

RUN cargo chef prepare --recipe-path recipe.json && \
	# --mount=type=cache,target=/usr/local/cargo/registry \
	# --mount=type=cache,target=/usr/local/cargo/git \
	# --mount=type=cache,target=/app/target \
	cargo chef cook \
	--release \
	--target aarch64-unknown-linux-musl \
	--recipe-path recipe.json

FROM base AS builder
WORKDIR /app

COPY --from=deps /app/target target
COPY --from=deps /usr/local/cargo /usr/local/cargo

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN \
	# --mount=type=cache,target=/usr/local/cargo/registry \
	# --mount=type=cache,target=/usr/local/cargo/git \
	# --mount=type=cache,target=/app/target \
	cargo build \
	--bin app \
	--release \
	--target aarch64-unknown-linux-musl \
	--offline

RUN strip --strip-unneeded target/aarch64-unknown-linux-musl/release/app
RUN upx --best target/aarch64-unknown-linux-musl/release/app

FROM scratch

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/app /app
CMD ["/app"]
