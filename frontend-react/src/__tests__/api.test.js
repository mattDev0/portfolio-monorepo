import { describe, it, expect, vi } from 'vitest';
import { isLocalRuntime, apiUrl } from '../api';

describe('api.js', () => {
  it('detects local runtime in jsdom test environment', () => {
    expect(isLocalRuntime).toBe(true);
  });

  it('constructs rust api urls correctly', () => {
    const url = apiUrl('rust', '/status');
    // In local dev, it should point to localhost:8080 by default
    expect(url).toBe('http://localhost:8080/status');
  });

  it('constructs java api urls correctly', () => {
    const url = apiUrl('java', '/github/activity');
    expect(url).toBe('http://localhost:8081/github/activity');
  });

  it('handles paths missing leading slash', () => {
    const url = apiUrl('rust', 'status');
    expect(url).toBe('http://localhost:8080/status');
  });
});
