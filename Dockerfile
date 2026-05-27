FROM rust:bookworm AS builder
WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

RUN cargo build --release --jobs 1

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/portfolio_enoocdev .
COPY --from=builder /usr/src/app/templates ./templates

EXPOSE 3000

CMD ["./portfolio_enoocdev"]
