import { renderHook, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import useTelemetry from '../hooks/useTelemetry';

const mockRustStatus = {
  cpu_usage_percent: 25.5,
  memory_used_mb: 512,
  memory_total_mb: 1024
};

const mockRustHistory = [
  { cpu: 10, memory: 40 },
  { cpu: 15, memory: 45 }
];

const mockJavaStatus = {
  engine: "Spring Boot 3.1.5",
  uptime: "1h 30m"
};

const mockNetworkStatus = {
  google_dns: { latency_ms: 10 },
  cloudflare_dns: { latency_ms: 12 },
  riot_games: { latency_ms: 25 }
};

const mockNetworkHistory = [
  { google_dns: 10, cloudflare_dns: 12, riot_games: 25 }
];

describe('useTelemetry', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    global.fetch = vi.fn((url) => {
      if (url.includes('/api/status/history')) {
        return Promise.resolve({ json: () => Promise.resolve(mockRustHistory) });
      }
      if (url.includes('/api/status/network/history')) {
        return Promise.resolve({ json: () => Promise.resolve(mockNetworkHistory) });
      }
      if (url.includes('/api/status/network')) {
        return Promise.resolve({ json: () => Promise.resolve(mockNetworkStatus) });
      }
      if (url.includes('/api/status')) {
        return Promise.resolve({ json: () => Promise.resolve(mockRustStatus) });
      }
      if (url.includes('/api/infrastructure/metrics')) {
        return Promise.resolve({ json: () => Promise.resolve(mockJavaStatus) });
      }
      return Promise.reject(new Error('Unknown URL: ' + url));
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('fetches and returns initial data when visible', async () => {
    const { result } = renderHook(() => useTelemetry(true));

    await act(async () => {
      await vi.advanceTimersByTimeAsync(1);
    });

    expect(result.current.rustStatus).toEqual(mockRustStatus);
    expect(result.current.javaStatus).toEqual(mockJavaStatus);
    expect(result.current.networkStatus).toEqual(mockNetworkStatus);

    // It should have fetched 5 times on mount
    expect(global.fetch).toHaveBeenCalledTimes(5);
  });

  it('does not fetch when isVisible is false', () => {
    renderHook(() => useTelemetry(false));
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('polls at intervals', async () => {
    const { result } = renderHook(() => useTelemetry(true));

    await act(async () => {
      await vi.advanceTimersByTimeAsync(1);
    });

    expect(result.current.rustStatus).toEqual(mockRustStatus);
    expect(global.fetch).toHaveBeenCalledTimes(5);

    // Fast-forward 5 seconds (Rust and Network polls)
    await act(async () => {
      await vi.advanceTimersByTimeAsync(5000);
    });
    // 2 more fetches for rust and network
    expect(global.fetch).toHaveBeenCalledTimes(7);

    // Fast-forward another 5 seconds (Java also polls at 10s)
    await act(async () => {
      await vi.advanceTimersByTimeAsync(5000);
    });
    // 2 more for rust/network, 1 for java
    expect(global.fetch).toHaveBeenCalledTimes(10);
  });
});
