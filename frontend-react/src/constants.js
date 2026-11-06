export const TOPOLOGY_INFO = {
  client: {
    title: "Client Browser",
    tech: "React 19 + Vite + Tailwind CSS",
    protocol: "HTTPS (Port 443)",
    description: "The interactive UI dashboard running in the visitor's browser. It polls the microservice APIs periodically (Telemetry every 5s, JVM every 10s, Spotify every 10s) and updates the layout reactively.",
    badge: "Frontend"
  },
  nginx: {
    title: "Cloud Run Domain Mapping",
    tech: "Custom domain + Google-managed TLS",
    protocol: "HTTPS (Google Front End)",
    description: "mattdev0.tech points at Google's front end through a Cloud Run domain mapping, which terminates TLS with a Google-managed certificate and routes requests to the frontend service. The browser then calls the Rust and Java APIs directly on their own Cloud Run URLs.",
    badge: "Gateway"
  },
  k8s: {
    title: "Google Cloud Run (Serverless)",
    tech: "Serverless Container Platform",
    protocol: "HTTPS (public run.app URLs)",
    description: "A fully managed container platform: each service scales with traffic and down to zero when idle. Kubernetes manifests for the same services (Kustomize, in infrastructure/k8s) are validated with kubeconform in CI, but production runs on Cloud Run.",
    badge: "Infrastructure"
  },
  frontend: {
    title: "frontend-react Service",
    tech: "Docker + Nginx Server",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "Nginx (unprivileged) container serving the built React bundle. Deployed as a Cloud Run service: a new revision only takes traffic once it has started, and the service scales to zero when idle.",
    badge: "Microservice"
  },
  rust: {
    title: "backend-rust Service",
    tech: "Rust + Axum Engine",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "Axum web server compiled to a native binary. Refreshes CPU and memory telemetry for its own container in a background task, and manages the Spotify access token.",
    badge: "Microservice"
  },
  java: {
    title: "backend-java Service",
    tech: "Java 25 + Spring Boot 4",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "Spring Boot service that fetches recent GitHub activity and caches it with Spring Cache (@Cacheable), so page visits don't use up GitHub's API rate limit.",
    badge: "Microservice"
  },
  spotify: {
    title: "Spotify Web API",
    tech: "OAuth 2.0 REST Web Services",
    protocol: "External API Calls",
    description: "Third-party streaming API. The Rust backend holds an OAuth 2.0 refresh token and exchanges it for short-lived access tokens to fetch the current and recent tracks.",
    badge: "External Service"
  },
  github: {
    title: "GitHub REST API",
    tech: "GitHub Public REST API",
    protocol: "External API Calls",
    description: "Third-party platform API queried by the Java backend to fetch live repository events. Spring Cache limits API calls to prevent IP rate-limiting blocks.",
    badge: "External Service"
  }
};
