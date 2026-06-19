FROM rust:1.80-slim AS builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/stockflow
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/stockflow/target/release/stockflow /app/stockflow
EXPOSE 3000
ENTRYPOINT ["/app/stockflow"]
