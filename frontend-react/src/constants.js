export const TOPOLOGY_INFO = {
  client: {
    title: "Client Browser",
    tech: "React 19 + Vite + Tailwind CSS",
    protocol: "HTTPS (Port 443)",
    description: "The interactive UI dashboard running in the visitor's browser. It polls the microservice APIs periodically (Telemetry every 5s, JVM every 10s, Spotify every 10s) and updates the layout reactively.",
    badge: "Frontend"
  },
  nginx: {
    title: "GCP Global Ingress",
    tech: "HTTPS Load Balancing + DNS",
    protocol: "HTTPS -> Cloud Run Routing",
    description: "Serves as the entry gateway for the platform. Terminates SSL (Google Managed Certificates), manages global load balancing, and routes public paths to the appropriate serverless Cloud Run container services.",
    badge: "Gateway"
  },
  k8s: {
    title: "Google Cloud Run (Serverless)",
    tech: "Serverless Container Platform",
    protocol: "GCP Internal Service Mesh",
    description: "A fully managed container platform hosting isolated, auto-scaling microservices. (Note: The codebase is fully Kubernetes-ready, with active K3s manifests and Kustomize overlays included in the repository for alternative cluster deployment).",
    badge: "Infrastructure"
  },
  frontend: {
    title: "frontend-react Service",
    tech: "Docker + Nginx Server",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "Containerized deployment serving the static built React application bundle. Deployed as a managed Google Cloud Run service with zero-downtime rolling updates and instant scale-to-zero.",
    badge: "Microservice"
  },
  rust: {
    title: "backend-rust Service",
    tech: "Rust + Axum Engine",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "High-performance Axum web server compiled to native machine code. Refreshes host CPU/Memory telemetry in a thread-safe background task and acts as the Spotify authentication token manager.",
    badge: "Microservice"
  },
  java: {
    title: "backend-java Service",
    tech: "Java 21 + Spring Boot 3",
    protocol: "HTTPS (Cloud Run Ingress)",
    description: "Enterprise-grade Spring Boot microservice. Manages caching abstractions using Spring Cache (@Cacheable) to bypass public rate boundaries, pulling recent commit payloads from the GitHub API.",
    badge: "Microservice"
  },
  spotify: {
    title: "Spotify Web API",
    tech: "OAuth 2.0 REST Web Services",
    protocol: "External API Calls",
    description: "Third-party audio streaming REST API. Authenticated securely via OAuth 2.0 client-credentials and token refresh flows executed from the Rust backend to fetch current/recent tracks.",
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
