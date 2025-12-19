FROM rust:1.92-slim as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/coolify-badge /app/badge-service

USER 1000

ENV COOLIFY_URL=""
ENV API_TOKEN=""
ENV PORT=3000

CMD ["./badge-service"]
