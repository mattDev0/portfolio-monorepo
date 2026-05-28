# Stage 1: Build the Rust binary
FROM rust:slim-bookworm AS builder
WORKDIR /usr/src/app

# Copy dependency definition and source files
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Compile in release mode using BuildKit cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release && \
    cp target/release/backend-rust /usr/local/bin/backend-rust

# Stage 2: Create the minimal runner image
FROM debian:bookworm-slim
WORKDIR /app
# Install OpenSSL certificates (required for the Spotify reqwest calls)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder
COPY --from=builder /usr/local/bin/backend-rust .

EXPOSE 8080
CMD ["./backend-rust"]
