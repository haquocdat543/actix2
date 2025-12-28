# Step 1: Build stage
FROM rust:1.89 AS builder

ENV RUSTC_WRAPPER=""
ENV TARGET="aarch64-unknown-linux-musl"

RUN rustup target add ${TARGET}

WORKDIR /usr/src/app

RUN apt-get update -y \
	&& apt-get install -y \
	musl-tools

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --bin app \
	--release \
	--target ${TARGET}

FROM scratch

ENV TARGET="aarch64-unknown-linux-musl"

COPY --from=builder /usr/src/app/target/${TARGET}/release/app /

CMD ["/app"]
