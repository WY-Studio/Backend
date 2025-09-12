# syntax=docker/dockerfile:1

# ------- Builder stage -------
FROM rust:1.80 as builder
WORKDIR /wooyeon

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY resources ./resources

RUN rustup toolchain install nightly && rustup default nightly
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/wooyeon/target \
    cargo build --release && \
    cp target/release/wy_backend /wooyeon/wy_backend

# ------- Runtime stage -------
FROM debian:bookworm-slim
WORKDIR /wooyeon

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /wooyeon/wy_backend /wooyeon/wy_backend

EXPOSE 3000
ENV RUST_LOG=info
ARG APP_ENV=stage
ENV APP_ENV=${APP_ENV}
ENV ENV_FILE=""
ENTRYPOINT ["/wooyeon/wy-backend"]

