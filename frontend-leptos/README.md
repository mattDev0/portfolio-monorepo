# Portfolio Frontend (Leptos CSR)

This is the Leptos 0.8 Client-Side Rendered (CSR) WebAssembly port of my portfolio single-page application, mirroring the user interface, design system, responsive layouts, and telemetry integrations of the original React frontend.

It compiles to WebAssembly (`wasm32-unknown-unknown`) served statically through an unprivileged Nginx container, sharing strongly-typed API models with the backend services via `shared/portfolio-api-types`.

## Tech Stack

- **Framework**: [Leptos 0.8](https://leptos.dev/) (CSR mode)
- **Language**: Rust (Edition 2024, toolchain pinned to 1.95.0)
- **Bundler & Dev Server**: [Trunk](https://trunkrs.dev/) 0.22.1
- **Styling**: Tailwind CSS v4 (using the standalone CLI)
- **Shared Types**: `portfolio-api-types` (local path dependency)
- **Deployment**: Static assets hosted on `nginxinc/nginx-unprivileged:alpine`

## Local Development

### Prerequisites

1. Rust 1.95.0 with the `wasm32-unknown-unknown` target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Trunk:
   ```bash
   cargo install --locked trunk@0.22.1 --ignore-rust-version
   ```
3. Tailwind CSS standalone CLI (v4.3.3):
   ```bash
   curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64
   chmod +x tailwindcss-linux-x64
   sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
   ```

### Running the Dev Server

To start the Trunk local development server with hot-reloading:

```bash
trunk serve
```

Trunk executes Tailwind CSS automatically before each build using the configured build hook in `Trunk.toml`, serving the application at `http://localhost:8080`.

### Runtime Configuration

The frontend dynamically detects its runtime environment:
- When running locally (`localhost`, `127.0.0.1`, or when the local dev query flag is present), requests to `/api/v1/system` route to the local Rust service (`http://localhost:8081` by default) and `/api/v1/network` route to the local Java service (`http://localhost:8082` by default).
- In production (`mattdev0.tech`), API requests route through the shared domain endpoints (`/api/v1/...`).

## Building for Production

To produce release-optimized WebAssembly and CSS bundles:

```bash
trunk build --release --locked
```

Compiled assets will be placed in the `dist/` directory.

## Docker Build

To build the unprivileged Nginx container image:

```bash
docker build -f frontend-leptos/Dockerfile -t portfolio-frontend-leptos:local .
```

To run the container locally:

```bash
docker run --rm -p 8080:8080 portfolio-frontend-leptos:local
```

## Bundle Size Comparison (Stage 1)

Measured production release bundle sizes comparing the React baseline to Leptos CSR:

| Asset | Leptos CSR (Raw) | Leptos CSR (Gzip) | React Baseline (Raw) | React Baseline (Gzip) |
|---|---|---|---|---|
| JavaScript Loader / Runtime | 42.9 KB (42,857 B) | 7.8 KB (7,816 B) | 299.3 KB (299,281 B) | 85.2 KB (85,154 B) |
| WebAssembly Binary | 460.9 KB (460,931 B) | 183.0 KB (183,003 B) | N/A | N/A |
| Stylesheet (Tailwind CSS v4) | 50.2 KB (50,161 B) | 9.1 KB (9,055 B) | 50.4 KB (50,364 B) | 9.1 KB (9,059 B) |
| **Total Transfer (Gzipped)** | | **199.9 KB** | | **94.2 KB** |

The compressed WASM payload is 183 KB, comfortably under the 300 KB budget constraint.
