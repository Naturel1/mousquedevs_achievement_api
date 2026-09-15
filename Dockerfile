# ==========================================
# Multi-Stage Dockerfile for Rust API
# ==========================================

# Stage 1: Build the binary
FROM rust:1.88-alpine AS builder

# System dependencies for Diesel (libpq) and C compilation
RUN apk add --no-cache musl-dev postgresql-dev

WORKDIR /app

# Copy dependency configurations
COPY Cargo.toml Cargo.lock ./

# Pre-compile dependencies to speed up Docker cache
RUN mkdir src && echo "pub fn dummy() {}" > src/lib.rs && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Copy source code and SQL migrations (embedded at compile time)
COPY src ./src
COPY migrations ./migrations

# Final compilation of the optimized binary
RUN cargo build --release

# Stage 2: Lightweight runtime image for production
FROM alpine:3.20

RUN apk add --no-cache libpq ca-certificates tzdata

WORKDIR /app

# Copy compiled binary from builder
COPY --from=builder /app/target/release/app /app/app
COPY Rocket.toml ./

EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

CMD ["/app/app"]
