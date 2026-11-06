# Portfolio Frontend (Leptos CSR)

This is the Leptos 0.8 Client-Side Rendered (CSR) WebAssembly port of my portfolio single-page application, mirroring the user interface, design system, responsive layouts, and telemetry integrations of the original React frontend.

It compiles to WebAssembly (`wasm32-unknown-unknown`) served statically through an unprivileged Nginx container, sharing strongly-typed API models with the backend services via `shared/portfolio-api-types`.

## Tech Stack

- **Framework**: [Leptos 0.8](https://leptos.dev/) (CSR mode)
- **Language**: Rust (Edition 2024, toolchain pinned to 1.95)
- **Bundler & Dev Server**: [Trunk](https://trunkrs.dev/) 0.22.1
- **Styling**: Tailwind CSS v4 (using the standalone CLI)
- **Shared Types**: `portfolio-api-types` (local path dependency)
- **Deployment**: Static assets hosted on `nginxinc/nginx-unprivileged:alpine`

## Local Development

### Prerequisites

1. Rust 1.95 with the `wasm32-unknown-unknown` target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Trunk 0.22.1:
   ```bash
   cargo install --locked trunk@0.22.1
   ```
3. Tailwind CSS standalone CLI (v4.3.3):
   ```bash
   curl -fsSLO https://github.com/tailwindlabs/tailwindcss/releases/download/v4.3.3/tailwindcss-linux-x64
   echo "dc61b3ac6b8c9ca874c0cc4c57b2409791a64c5540404ca5f5367360babc313a  tailwindcss-linux-x64" | sha256sum -c -
   chmod +x tailwindcss-linux-x64
   sudo mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss
   ```

### Running the Dev Server

To start the Trunk local development server with hot-reloading:

```bash
trunk serve --port 3002
```

Trunk runs Tailwind before each build (the hook in `Trunk.toml`). Use a port other than
8080, which the local Rust API uses.

### Runtime Configuration

`src/api.rs` picks the API base URLs the same way `frontend-react/src/api.js` does:

- **Local** when the build-time variable `PORTFOLIO_LOCAL_DEV` is `1`, `true`, `yes`,
  `local`, `dev` or `development`; or, unless it's set to `0`, `false`, `no`, `prod` or
  `production`, when it's a debug build (`trunk serve`) or the page is opened on
  `localhost`, `127.0.0.1`, `::1` or `0.0.0.0`. Local calls go to the Rust API on
  `http://localhost:8080` and the Java API on `http://localhost:8081`, which can be changed
  at build time with `PORTFOLIO_LOCAL_RUST_API_BASE_URL` and
  `PORTFOLIO_LOCAL_JAVA_API_BASE_URL`.
- **Production** otherwise: the browser calls the Rust and Java services directly on
  their Cloud Run `*.run.app` URLs.

Endpoints used: Rust `/api/status`, `/api/status/history`, `/api/status/network`,
`/api/status/network/history` and `/api/spotify`; Java `/api/infrastructure/metrics` and
`/api/github/activity`.

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
docker run --rm -p 3001:8080 portfolio-frontend-leptos:local
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
