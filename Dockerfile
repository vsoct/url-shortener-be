FROM lukemathwalker/cargo-chef:latest-rust-1.65-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update
RUN apt-get install -y protobuf-compiler
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin url-shortener-be

FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt-get update
RUN apt-get install -y libssl-dev
COPY --from=builder /app/target/release/url-shortener-be /usr/local/bin

ENTRYPOINT ["/usr/local/bin/url-shortener-be"]

# https://github.com/LukeMathWalker/cargo-chef