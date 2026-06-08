export const TOPOLOGY_INFO = {
  client: {
    title: "Client Browser",
    tech: "React 19 + Vite + Tailwind CSS",
    protocol: "HTTPS (Port 443)",
    description: "The interactive UI dashboard running in the visitor's browser. It polls the microservice APIs periodically (Telemetry every 5s, JVM every 10s, Spotify every 10s) and updates the layout reactively.",
    badge: "Frontend"
  },
  nginx: {
    title: "Nginx Reverse Proxy",
    tech: "Nginx Gateway Config",
    protocol: "HTTPS -> NodePorts",
    description: "Operates as the entry point on the Azure VM. It terminates SSL (Let's Encrypt), resolves CORS by hosting services on the same domain, and routes paths: / to Port 30000, /api/status & /api/spotify to Port 30080, and /api/github to Port 30081.",
    badge: "Gateway"
  },
  k8s: {
    title: "K3s Kubernetes Namespace",
    tech: "K3s Cluster (Orchestration)",
    protocol: "K8s Service DNS",
    description: "A lightweight, secure Kubernetes namespace ('portfolio') hosting isolated container pods. Manages scaling, self-healing, rolling updates, and enforces strict resource limits (limits: memory: 32Mi, cpu: 100m).",
    badge: "Infrastructure"
  },
  frontend: {
    title: "frontend-react Pod",
    tech: "Docker + Nginx Server",
    protocol: "HTTP (Container Port 80)",
    description: "Containerized deployment serving the static built React application bundle. Managed via rolling zero-downtime restarts in Kubernetes (K3s NodePort 30000).",
    badge: "Deployment Pod"
  },
  rust: {
    title: "backend-rust Pod",
    tech: "Rust + Axum Engine",
    protocol: "HTTP (Container Port 8080)",
    description: "High-performance Axum web server compiled to native machine code. Refreshes host CPU/Memory telemetry in a thread-safe background task and acts as the Spotify authentication token manager.",
    badge: "Deployment Pod"
  },
  java: {
    title: "backend-java Pod",
    tech: "Java 21 + Spring Boot 4",
    protocol: "HTTP (Container Port 8081)",
    description: "Enterprise-grade Spring Boot microservice. Manages caching abstractions using Spring Cache (@Cacheable) to bypass public rate boundaries, pulling recent commit payloads from the GitHub API.",
    badge: "Deployment Pod"
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
