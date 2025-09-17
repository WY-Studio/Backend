# syntax=docker/dockerfile:1

# ------- Builder stage -------
FROM rust:1.80 AS builder
WORKDIR /wooyeon/wy_backend

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY resources ./resources

RUN rustup toolchain install nightly && rustup default nightly
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/wooyeon/wy_backend/target \
    cargo build --release && \
    cp target/release/wy_backend /wooyeon/wy_backend/wy_backend

# ------- Runtime stage -------
FROM debian:bookworm-slim
WORKDIR /wooyeon/wy_backend

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /wooyeon/wy_backend/wy_backend /wooyeon/wy_backend/wy_backend
COPY config.yaml /wooyeon/wy_backend/config.yaml

EXPOSE 3000
ENV RUST_LOG=info


ENTRYPOINT ["/wooyeon/wy_backend/wy_backend"]

