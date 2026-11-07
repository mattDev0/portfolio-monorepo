# Stage 1: Build the Leptos WASM application
FROM rust:1.95-slim-bookworm AS builder

# Install the build tools as prebuilt release binaries, each checked against
# its published SHA-256: Tailwind CLI, Trunk, wasm-bindgen and wasm-opt.
# Trunk then runs offline and never downloads tools itself.
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && curl -fsSLo /tmp/tailwindcss https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64 \
    && echo "dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a  /tmp/tailwindcss" > /tmp/tailwind.sha256 \
    && sha256sum -c /tmp/tailwind.sha256 \
    && install -m 0755 /tmp/tailwindcss /usr/local/bin/tailwindcss \
    && curl -fsSLo /tmp/trunk.tar.gz https://github.com/trunk-rs/trunk/releases/download/v0.22.1/trunk-x86_64-unknown-linux-gnu.tar.gz \
    && echo "2ab8d548fd5ed9cda50906f61fd8654204e7893bd52ebbfd90533862b7d71ee1  /tmp/trunk.tar.gz" > /tmp/trunk.sha256 \
    && sha256sum -c /tmp/trunk.sha256 \
    && tar -xzf /tmp/trunk.tar.gz -C /usr/local/bin trunk \
    && curl -fsSLo /tmp/wasm-bindgen.tar.gz https://github.com/wasm-bindgen/wasm-bindgen/releases/download/0.2.130/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz \
    && echo "9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f  /tmp/wasm-bindgen.tar.gz" > /tmp/wasm-bindgen.sha256 \
    && sha256sum -c /tmp/wasm-bindgen.sha256 \
    && tar -xzf /tmp/wasm-bindgen.tar.gz -C /usr/local/bin --strip-components=1 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm-bindgen \
    && curl -fsSLo /tmp/binaryen.tar.gz https://github.com/WebAssembly/binaryen/releases/download/version_131/binaryen-version_131-x86_64-linux.tar.gz \
    && echo "b5bf1f0eaf17c63ee588ff7a5954dc8f6ce2c26989051c66f24dfe9ece3e46db  /tmp/binaryen.tar.gz" > /tmp/binaryen.sha256 \
    && sha256sum -c /tmp/binaryen.sha256 \
    && tar -xzf /tmp/binaryen.tar.gz -C /usr/local/bin --strip-components=2 binaryen-version_131/bin/wasm-opt \
    && rm -f /tmp/*.tar.gz /tmp/*.sha256 /tmp/tailwindcss \
    && rustup target add wasm32-unknown-unknown

WORKDIR /workspace

# Copy shared types and frontend crate
COPY shared/portfolio-api-types shared/portfolio-api-types
COPY frontend-leptos frontend-leptos

# Build release artifacts
WORKDIR /workspace/frontend-leptos
RUN trunk build --release --locked --offline

# Stage 2: Serve with Nginx (Unprivileged)
FROM nginxinc/nginx-unprivileged:alpine
USER 0
RUN apk upgrade --no-cache && rm /etc/nginx/conf.d/default.conf
USER 101
COPY --from=builder /workspace/frontend-leptos/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /workspace/frontend-leptos/dist /usr/share/nginx/html
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
