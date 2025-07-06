# Step 1: Build stage
FROM rust:1.72.0 as builder

# Install dependencies (if any)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

# Set the working directory inside the container
WORKDIR /usr/src/actix

# Copy the Cargo.toml and Cargo.lock (to leverage Docker caching)
COPY Cargo.toml Cargo.lock ./

# Copy the rest of the source code into the container
COPY . .

# Build the Rust project
RUN cargo build --release

# Step 2: Final image (for running the application)
FROM ubuntu:22.04

RUN apt-get update

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/actix/target/release/actix /usr/local/bin/actix

# Set the entry point for the container
CMD ["actix"]
