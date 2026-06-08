import { useState, useEffect } from 'react';
import { apiUrl } from '../api';

export default function useSpotify(isVisible) {
  const [spotifyData, setSpotifyData] = useState(null);
  const [localProgressMs, setLocalProgressMs] = useState(0);

  // Fetch Live Spotify Session via Rust (with 10s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchSpotify = () => {
      fetch(apiUrl('rust', '/api/spotify'))
        .then(response => response.json())
        .then(data => setSpotifyData(data))
        .catch(error => console.error("Error fetching Spotify API:", error));
    };

    fetchSpotify();
    const interval = setInterval(fetchSpotify, 10000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Sync local progress when new data is fetched
  useEffect(() => {
    if (spotifyData) {
      setLocalProgressMs(spotifyData.progress_ms || 0);
    }
  }, [spotifyData]);

  // Tick the progress bar locally every second if a song is playing (only when tab is visible)
  useEffect(() => {
    if (!spotifyData || !spotifyData.is_playing || !isVisible) return;

    const interval = setInterval(() => {
      setLocalProgressMs(prev => {
        const next = prev + 1000;
        return next > spotifyData.duration_ms ? spotifyData.duration_ms : next;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [spotifyData, isVisible]);

  // Helper to format milliseconds to M:SS
  const formatTime = (ms) => {
    if (!ms) return "0:00";
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
  };

  const progressPercent = spotifyData?.duration_ms
    ? (localProgressMs / spotifyData.duration_ms) * 100
    : 0;

  return {
    spotifyData,
    localProgressMs,
    progressPercent,
    formatTime
  };
}
