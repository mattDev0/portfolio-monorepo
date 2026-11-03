# Stage 1: Build the Rust binary
FROM rust:slim-bookworm AS builder
WORKDIR /usr/src/app

# Build the dependencies alone first, against a placeholder main.rs, so this
# layer stays cached until Cargo.toml or Cargo.lock changes.
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Build the real binary. The touch makes cargo rebuild this crate instead of
# reusing the placeholder build.
COPY src ./src
RUN touch src/main.rs && \
    cargo build --release && \
    cp target/release/backend-rust /usr/local/bin/backend-rust

# Stage 2: Create the minimal runner image
FROM debian:bookworm-slim
WORKDIR /app

# Create a non-root user and group with numeric UID/GID
RUN groupadd -g 10001 -r appuser && useradd -u 10001 -m -r -g appuser appuser

# Install certificates and refresh perl-base with Debian security fixes.
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates perl-base && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder and set ownership
COPY --from=builder --chown=appuser:appuser /usr/local/bin/backend-rust .

# Switch to the non-root user (using numeric ID for Kubernetes runAsNonRoot verification)
USER 10001

EXPOSE 8080
CMD ["./backend-rust"]
