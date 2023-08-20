## INIT
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR app

## PLANNING
FROM chef AS planner

COPY Cargo.toml .
COPY src/ src/

RUN cargo chef prepare --recipe-path recipe.json

## BUILDER
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml .
COPY src/ src/

RUN cargo build --release

## RUNTIME
FROM debian:latest AS runtime

RUN apt update && apt install -y openssl ca-certificates
RUN update-ca-certificates

WORKDIR app

COPY --from=builder /app/target/release/zachsbot-discord /usr/local/bin

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV CARGO_TERM_COLOR=always

ENTRYPOINT ["/usr/local/bin/zachsbot-discord"]