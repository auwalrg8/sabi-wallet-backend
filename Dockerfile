FROM rust:1.84 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/sabi-wallet-backend /usr/local/bin/
CMD ["sabi-wallet-backend"]