# Capture the information required to build dependencies
FROM rust:alpine AS planner
ARG CHEF_VERSION=0.1.73
RUN apk add --no-cache curl
WORKDIR /app
RUN curl --location \
    "https://github.com/LukeMathWalker/cargo-chef/releases/download/v${CHEF_VERSION}/cargo-chef-x86_64-unknown-linux-musl.tar.gz" | \
    tar -xz -C /usr/local/bin/
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build binary with caching
FROM rust:alpine AS builder
RUN apk add --no-cache cargo-edit libc-dev
RUN rustup default nightly
WORKDIR /app
COPY --from=planner /usr/local/bin/cargo-chef /usr/local/bin/cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Build only the dependencies
RUN cargo chef cook --release --package alnwick --recipe-path recipe.json
COPY . .
# Declared after the cook step because a changed ARG invalidates every later RUN
ARG VERSION=0.0.0
RUN cargo set-version "${VERSION}"
RUN cargo build --release --locked --package alnwick

# Copy binary to minimal image
FROM alpine:latest
COPY --from=builder /app/target/release/alnwick /bin/alnwick
WORKDIR /
ENTRYPOINT ["alnwick"]
