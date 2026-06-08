import { useState, useEffect } from 'react';
import { apiUrl } from '../api';

export default function useTelemetry(isVisible) {
  const [rustStatus, setRustStatus] = useState(null);
  const [javaStatus, setJavaStatus] = useState(null);
  const [telemetryHistory, setTelemetryHistory] = useState([]);
  const [networkStatus, setNetworkStatus] = useState(null);
  const [networkHistory, setNetworkHistory] = useState([]);

  const getCpuPercentage = (cpuUsageStr) => {
    if (!cpuUsageStr) return 0;
    const parsed = parseFloat(cpuUsageStr.replace('%', ''));
    return isNaN(parsed) ? 0 : parsed;
  };

  // Fetch Rust Hardware Metrics (with 5s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchRust = () => {
      fetch(apiUrl('rust', '/api/status'))
        .then(response => response.json())
        .then(data => setRustStatus(data))
        .catch(error => console.error("Error fetching from Rust API:", error));
    };

    fetchRust();
    const interval = setInterval(fetchRust, 5000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Rust Telemetry History on mount (and on regaining visibility if history was blank)
  useEffect(() => {
    if (!isVisible) return;
    fetch(apiUrl('rust', '/api/status/history'))
      .then(response => response.json())
      .then(data => setTelemetryHistory(data))
      .catch(error => console.error("Error fetching telemetry history:", error));
  }, [isVisible]);

  // Sync polled metrics with history buffer
  useEffect(() => {
    if (!rustStatus || !isVisible) return;

    const cpu = getCpuPercentage(rustStatus.cpu_usage);

    // Parse memory percentage from "used MB / total MB"
    let memory = 0;
    if (rustStatus.memory_usage) {
      const parts = rustStatus.memory_usage.split('/');
      if (parts.length === 2) {
        const used = parseFloat(parts[0].replace('MB', '').trim());
        const total = parseFloat(parts[1].replace('MB', '').trim());
        if (total > 0) {
          memory = (used / total) * 100;
        }
      }
    }

    setTelemetryHistory(prev => {
      // Check if this new data point is already the latest point in history to prevent duplicate entries
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
  }, [rustStatus, isVisible]);
  
  // Fetch Rust Network Metrics (with 5s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchNetwork = () => {
      fetch(apiUrl('rust', '/api/status/network'))
        .then(response => response.json())
        .then(data => setNetworkStatus(data))
        .catch(error => console.error("Error fetching from Rust Network API:", error));
    };

    fetchNetwork();
    const interval = setInterval(fetchNetwork, 5000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Rust Network History on mount (and on regaining visibility)
  useEffect(() => {
    if (!isVisible) return;
    fetch(apiUrl('rust', '/api/status/network/history'))
      .then(response => response.json())
      .then(data => setNetworkHistory(data))
      .catch(error => console.error("Error fetching network history:", error));
  }, [isVisible]);

  // Sync polled network metrics with history buffer
  useEffect(() => {
    if (!networkStatus || !isVisible) return;

    const google = networkStatus.google_dns.latency_ms;
    const cloudflare = networkStatus.cloudflare_dns.latency_ms;
    const riot = networkStatus.riot_games.latency_ms;

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
  }, [networkStatus, isVisible]);

  // Fetch Java JVM Metrics (with 10s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchJava = () => {
      fetch(apiUrl('java', '/api/infrastructure/metrics'))
        .then(response => response.json())
        .then(data => setJavaStatus(data))
        .catch(error => console.error("Error fetching from Java API:", error));
    };

    fetchJava();
    const interval = setInterval(fetchJava, 10000);
    return () => clearInterval(interval);
  }, [isVisible]);

  return {
    rustStatus,
    javaStatus,
    telemetryHistory,
    networkStatus,
    networkHistory,
    getCpuPercentage
  };
}
