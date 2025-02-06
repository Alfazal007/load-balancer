FROM rust:slim-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
   pkg-config \
   libssl-dev \
   && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
   --mount=type=cache,target=/usr/local/cargo/git \
   --mount=type=cache,target=/app/target \
   cargo build --release

COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
   --mount=type=cache,target=/usr/local/cargo/git \
   --mount=type=cache,target=/app/target \
   cargo build --release && cp target/release/orchestrator .

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y \
   libssl3 \
   ca-certificates \
   && rm -rf /var/lib/apt/lists/*
 
COPY --from=builder /app/orchestrator .
USER nobody
ENTRYPOINT ["./orchestrator"]
