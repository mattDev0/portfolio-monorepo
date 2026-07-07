import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import GitHubActivity from '../GitHubActivity';
import { apiUrl } from '../api';

// Mock the api module
vi.mock('../api', () => ({
  apiUrl: vi.fn((service, path) => `http://mock-java-api${path}`)
}));

describe('GitHubActivity Component', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('renders loading skeleton initially', () => {
    // Keep fetch unresolved to test loading state
    global.fetch = vi.fn(() => new Promise(() => {}));
    const { container } = render(<GitHubActivity />);
    expect(screen.getByText('Recent GitHub Commits (Java Infra)')).toBeInTheDocument();
    // Check for skeleton divs (they have animate-pulse)
    expect(container.querySelectorAll('.animate-pulse.space-y-4').length).toBe(1);
  });

  it('renders commits after fetch', async () => {
    const mockCommits = [
      { repo: 'portfolio-repo', message: 'test commit', date: '2023-10-27T10:00:00Z', hash: 'abc1234' }
    ];
    global.fetch = vi.fn(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve(mockCommits)
    }));

    render(<GitHubActivity />);

    await waitFor(() => {
      expect(screen.getByText('portfolio-repo')).toBeInTheDocument();
      expect(screen.getByText('test commit')).toBeInTheDocument();
      expect(screen.getByText('abc1234')).toBeInTheDocument();
    });
  });

  it('renders error message on fetch failure', async () => {
    global.fetch = vi.fn(() => Promise.reject(new Error('Network error')));

    render(<GitHubActivity />);

    await waitFor(() => {
      expect(screen.getByText('Activity feed temporarily unavailable.')).toBeInTheDocument();
    });
  });
});
