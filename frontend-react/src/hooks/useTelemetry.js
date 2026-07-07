import { useState, useEffect } from 'react';
import { apiUrl } from '../api';

export default function useTelemetry(isVisible) {
  const [rustStatus, setRustStatus] = useState(null);
  const [rustError, setRustError] = useState(false);
  
  const [javaStatus, setJavaStatus] = useState(null);
  const [javaError, setJavaError] = useState(false);
  
  const [telemetryHistory, setTelemetryHistory] = useState([]);
  
  const [networkStatus, setNetworkStatus] = useState(null);
  const [networkError, setNetworkError] = useState(false);
  
  const [networkHistory, setNetworkHistory] = useState([]);

  // Fetch Rust Hardware Metrics (with 5s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchRust = () => {
      fetch(apiUrl('rust', '/api/status'))
        .then(response => {
          if (!response.ok) throw new Error('Not ok');
          return response.json();
        })
        .then(data => {
          setRustStatus(data);
          setRustError(false);

          // Sync polled metrics with history buffer in the same update cycle
          const cpu = data.cpu_usage_percent ?? 0;
          let memory = 0;
          if (data.memory_total_mb > 0) {
            memory = (data.memory_used_mb / data.memory_total_mb) * 100;
          }

          setTelemetryHistory(prev => {
            const lastPoint = prev[prev.length - 1];
            if (lastPoint && lastPoint.cpu === cpu && lastPoint.memory === memory) {
              return prev;
            }
            const newHistory = [...prev, { cpu, memory }];
            if (newHistory.length > 20) {
              newHistory.shift();
            }
            return newHistory;
          });
        })
        .catch(error => {
          console.error("Error fetching from Rust API:", error);
          setRustError(true);
        });
    };

    fetchRust();
    const interval = setInterval(fetchRust, 5000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Rust Telemetry History on mount
  useEffect(() => {
    if (!isVisible) return;
    fetch(apiUrl('rust', '/api/status/history'))
      .then(response => {
        if (!response.ok) throw new Error('Not ok');
        return response.json();
      })
      .then(data => setTelemetryHistory(data))
      .catch(error => console.error("Error fetching telemetry history:", error));
  }, [isVisible]);
  
  // Fetch Rust Network Metrics (with 5s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchNetwork = () => {
      fetch(apiUrl('rust', '/api/status/network'))
        .then(response => {
          if (!response.ok) throw new Error('Not ok');
          return response.json();
        })
        .then(data => {
          setNetworkStatus(data);
          setNetworkError(false);

          // Sync polled network metrics with history buffer in the same update cycle
          const google = data.google_dns.latency_ms;
          const cloudflare = data.cloudflare_dns.latency_ms;
          const riot = data.riot_games.latency_ms;

          setNetworkHistory(prev => {
            const lastPoint = prev[prev.length - 1];
            if (lastPoint && lastPoint.google_dns === google && lastPoint.cloudflare_dns === cloudflare && lastPoint.riot_games === riot) {
              return prev;
            }
            const newHistory = [...prev, { google_dns: google, cloudflare_dns: cloudflare, riot_games: riot }];
            if (newHistory.length > 20) {
              newHistory.shift();
            }
            return newHistory;
          });
        })
        .catch(error => {
          console.error("Error fetching from Rust Network API:", error);
          setNetworkError(true);
        });
    };

    fetchNetwork();
    const interval = setInterval(fetchNetwork, 5000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Rust Network History on mount
  useEffect(() => {
    if (!isVisible) return;
    fetch(apiUrl('rust', '/api/status/network/history'))
      .then(response => {
        if (!response.ok) throw new Error('Not ok');
        return response.json();
      })
      .then(data => setNetworkHistory(data))
      .catch(error => console.error("Error fetching network history:", error));
  }, [isVisible]);

  // Fetch Java JVM Metrics (with 10s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchJava = () => {
      fetch(apiUrl('java', '/api/infrastructure/metrics'))
        .then(response => {
          if (!response.ok) throw new Error('Not ok');
          return response.json();
        })
        .then(data => {
          setJavaStatus(data);
          setJavaError(false);
        })
        .catch(error => {
          console.error("Error fetching from Java API:", error);
          setJavaError(true);
        });
    };

    fetchJava();
    const interval = setInterval(fetchJava, 10000);
    return () => clearInterval(interval);
  }, [isVisible]);

  return {
    rustStatus,
    rustError,
    javaStatus,
    javaError,
    telemetryHistory,
    networkStatus,
    networkError,
    networkHistory
  };
}
