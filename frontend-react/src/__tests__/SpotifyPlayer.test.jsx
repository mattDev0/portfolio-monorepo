import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import SpotifyPlayer from '../components/SpotifyPlayer';

describe('SpotifyPlayer Component', () => {
  const mockFormatTime = (ms) => `${Math.floor(ms / 60000)}:${((ms % 60000) / 1000).toFixed(0).padStart(2, '0')}`;

  it('renders loading state when spotifyData is null', () => {
    render(<SpotifyPlayer spotifyData={null} progressPercent={0} localProgressMs={0} formatTime={mockFormatTime} />);
    expect(screen.getByText('Connecting to Spotify Web API...')).toBeInTheDocument();
  });

  it('renders track info when playing', () => {
    const mockData = {
      is_playing: true,
      title: 'Mock Track',
      artist: 'Mock Artist',
      album_art: 'http://mock.art',
      duration_ms: 180000,
      track_url: 'http://mock.url'
    };
    render(<SpotifyPlayer spotifyData={mockData} progressPercent={50} localProgressMs={90000} formatTime={mockFormatTime} />);
    expect(screen.getByText('Mock Track')).toBeInTheDocument();
    expect(screen.getByText('Mock Artist')).toBeInTheDocument();
    expect(screen.getByText('Now Playing')).toBeInTheDocument();
  });

  it('renders recently played state', () => {
    const mockData = {
      is_playing: false,
      is_recently_played: true,
      title: 'Recent Track',
      artist: 'Recent Artist',
      album_art: 'http://mock.art',
      duration_ms: 180000,
      track_url: 'http://mock.url'
    };
    render(<SpotifyPlayer spotifyData={mockData} progressPercent={0} localProgressMs={0} formatTime={mockFormatTime} />);
    expect(screen.getByText('Recently Played')).toBeInTheDocument();
  });

  it('renders progress bar when playing', () => {
    const mockData = {
      is_playing: true,
      title: 'Mock Track',
      artist: 'Mock Artist',
      album_art: 'http://mock.art',
      duration_ms: 180000,
      track_url: 'http://mock.url'
    };
    const { container } = render(<SpotifyPlayer spotifyData={mockData} progressPercent={50} localProgressMs={90000} formatTime={mockFormatTime} />);
    expect(screen.getByText('1:30')).toBeInTheDocument();
    expect(screen.getByText('3:00')).toBeInTheDocument();
  });

  it('renders as link when track_url is provided', () => {
    const mockData = {
      is_playing: true,
      title: 'Mock Track',
      artist: 'Mock Artist',
      album_art: 'http://mock.art',
      duration_ms: 180000,
      track_url: 'http://mock.url'
    };
    render(<SpotifyPlayer spotifyData={mockData} progressPercent={50} localProgressMs={90000} formatTime={mockFormatTime} />);
    const link = screen.getByRole('link');
    expect(link).toHaveAttribute('href', 'http://mock.url');
  });
});
