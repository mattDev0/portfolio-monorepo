# Stage 1: Build the Leptos WASM application
FROM rust:1.95-slim-bookworm AS builder

# Install build dependencies, Tailwind CLI, Trunk, and WASM target
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64 \
    && echo "dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a  tailwindcss-linux-x64" > tailwind.sha256 \
    && sha256sum -c tailwind.sha256 \
    && rm tailwind.sha256 \
    && chmod +x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss \
    && cargo install --locked trunk@0.22.1 --ignore-rust-version \
    && rustup target add wasm32-unknown-unknown

WORKDIR /workspace

# Copy shared types and frontend crate
COPY shared/portfolio-api-types shared/portfolio-api-types
COPY frontend-leptos frontend-leptos

# Build release artifacts
WORKDIR /workspace/frontend-leptos
RUN trunk build --release --locked

# Stage 2: Serve with Nginx (Unprivileged)
FROM nginxinc/nginx-unprivileged:alpine
USER 0
RUN apk upgrade --no-cache && rm /etc/nginx/conf.d/default.conf
USER 101
COPY --from=builder /workspace/frontend-leptos/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /workspace/frontend-leptos/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
