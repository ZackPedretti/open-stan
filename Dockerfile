FROM rust:1.91 AS builder
WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y ca-certificates binutils && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/open-stan /app/

RUN strip /app/open-stan

EXPOSE 3000
CMD ["/app/open-stan"]
