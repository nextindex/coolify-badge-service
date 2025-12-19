# Stage 1: Build
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime (Security Hardened)
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/coolify-badge-service /app/badge-service

# Run as non-root user for security
USER 1000

ENV COOLIFY_URL=""
ENV API_TOKEN=""
ENV PORT=3000

CMD ["./badge-service"]
