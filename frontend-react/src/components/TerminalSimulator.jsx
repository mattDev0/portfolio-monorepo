import { useState, useEffect } from 'react';

const terminalSequence = [
  { text: "guest@mattdev0.tech:~$ initialize_telemetry", delay: 200 },
  { text: "[INFO] Establishing WebSocket handshake with rust-agent...", delay: 600 },
  { text: "[INFO] Client handshake accepted. Handshaking Spring orchestrator...", delay: 500 },
  { text: "[INFO] Session validated via JWT authority claims (ROLE_GUEST).", delay: 400 },
  { text: "[SUCCESS] Bidirectional PTY bridge established.", delay: 300 },
  { text: "guest@mattdev0.tech:~$ kubectl get pods -n devops", delay: 500 },
  { text: "NAME                                READY   STATUS    RESTARTS   AGE", delay: 300 },
  { text: "devops-frontend-6b4d5b6c-8x2p       1/1     Running   0          45d", delay: 100 },
  { text: "devops-orchestrator-9c4d5b6-7y1q    1/1     Running   2          45d", delay: 100 },
  { text: "devops-agent-8b3d4a5-9z0p           1/1     Running   0          45d", delay: 100 },
  { text: "guest@mattdev0.tech:~$ sysinfo --telemetry", delay: 600 },
  { text: "[OS] Linux 6.1.0-azure-amd64 | [Telemetry Mode] Active SSE Streams", delay: 400 },
  { text: "[Cluster metrics] Prometheus Node Exporter DaemonSet operational.", delay: 200 },
  { text: "guest@mattdev0.tech:~$ █", delay: 800 }
];

export default function TerminalSimulator({ active }) {
  const [lines, setLines] = useState([]);

  useEffect(() => {
    if (!active) {
      const timer = setTimeout(() => setLines([]), 0);
      return () => clearTimeout(timer);
    }

    let isMounted = true;
    
    const runSequence = async () => {
      for (const step of terminalSequence) {
        if (!isMounted) break;
        await new Promise(resolve => setTimeout(resolve, step.delay));
        if (!isMounted) break;
        setLines(prev => [...prev, step.text]);
      }
    };

    runSequence();

    return () => {
      isMounted = false;
    };
  }, [active]);

  return (
    <div className="bg-[var(--bg-inset)] border border-[var(--border-default)] rounded-[var(--radius-md)] p-4 font-mono text-[11px] text-[var(--accent-primary)] leading-relaxed shadow-inner h-60 overflow-y-auto custom-scrollbar select-none">
      {lines.map((line, idx) => {
        if (line.startsWith("guest@")) {
          return (
            <div key={idx} className="text-[var(--fg-muted)] mt-2">
              <span className="text-[var(--status-success)]">guest@mattdev0.tech</span>:<span className="text-[var(--accent-primary)]">~</span>$ {line.substring(23)}
            </div>
          );
        }
        if (line.startsWith("[SUCCESS]")) {
          return <div key={idx} className="text-[var(--status-success)] font-semibold">{line}</div>;
        }
        if (line.startsWith("[INFO]")) {
          return <div key={idx} className="text-[var(--fg-subtle)]">{line}</div>;
        }
        return <div key={idx} className="text-[var(--fg-default)]">{line}</div>;
      })}
    </div>
  );
}
