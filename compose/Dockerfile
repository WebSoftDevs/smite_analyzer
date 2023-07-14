FROM rust:1.68

WORKDIR /app

COPY Cargo.toml .

COPY src src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    cargo build --release
