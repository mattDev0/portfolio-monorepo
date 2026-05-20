# Stage 1: Build the Rust binary
FROM rust:slim-bookworm AS builder
WORKDIR /usr/src/app
# Copy the source code
COPY . .
# Compile in release mode for maximum performance
RUN cargo build --release

# Stage 2: Create the minimal runner image
FROM debian:bookworm-slim
WORKDIR /app
# Install OpenSSL certificates (required for the Spotify reqwest calls)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/target/release/backend-rust .

EXPOSE 8080
CMD ["./backend-rust"]
