import React from 'react';

export default function SpotifyPlayer({ spotifyData, progressPercent, localProgressMs, formatTime }) {
  const isPlaying = spotifyData?.is_playing;
  const isRecentlyPlayed = spotifyData?.is_recently_played;

  const innerContent = (
    <>
      <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
        <span className="flex h-3 w-3 relative">
          <span className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${isPlaying ? 'bg-emerald-400' : isRecentlyPlayed ? 'bg-amber-400' : 'bg-gray-500'}`}></span>
          {isPlaying && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400/40 opacity-40 delay-300"></span>}
          <span className={`relative inline-flex rounded-full h-3 w-3 ${isPlaying ? 'bg-emerald-500' : isRecentlyPlayed ? 'bg-amber-500' : 'bg-gray-500'} ${isPlaying ? 'shadow-[0_0_8px_#10b981]' : isRecentlyPlayed ? 'shadow-[0_0_8px_#f59e0b]' : ''}`}></span>
        </span>
        {/* Custom Tooltip */}
        <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-slate-950/95 border border-emerald-500/20 p-2 text-center text-[10px] text-emerald-400 font-mono shadow-xl z-20">
          {isPlaying ? 'Live playback' : isRecentlyPlayed ? 'Playback idle' : 'Offline'}
        </div>
      </div>
      
      <h3 className="text-lg font-bold text-emerald-400 mb-0 tracking-wide flex items-center">
        <svg className="w-5 h-5 mr-2 text-emerald-400 group-hover:animate-spin" style={{ animationDuration: isPlaying ? '6s' : '0s' }} fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z"/></svg>
        Spotify Session
      </h3>
      <p className="text-[10px] text-gray-500 font-medium mb-5">Web API sync via Rust token auth</p>
      
      {spotifyData ? (
        <div>
          <div className="flex items-center space-x-4 mt-2">
            <div className={`w-16 h-16 bg-slate-800 rounded-lg flex-shrink-0 shadow-md overflow-hidden relative border border-white/5 transition-transform duration-500 ${isPlaying ? 'group-hover:rotate-12 group-hover:scale-105' : ''}`}>
              {spotifyData.album_art ? (
                <img src={spotifyData.album_art} alt="Album Art" className={`w-full h-full object-cover transition-all duration-500 ${isPlaying ? '' : 'grayscale opacity-60'}`} />
              ) : (
                <div className="absolute inset-0 bg-gradient-to-tr from-emerald-500 to-slate-900 opacity-50"></div>
              )}
            </div>
            <div className="overflow-hidden flex-grow">
              <p className="text-gray-100 font-bold truncate group-hover:text-emerald-400 transition-colors">{spotifyData.title}</p>
              <p className="text-gray-400 text-sm truncate">{spotifyData.artist}</p>
              <p className={`text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold ${isPlaying ? 'text-emerald-400' : isRecentlyPlayed ? 'text-amber-500/80' : 'text-gray-500'}`}>
                {isPlaying ? 'Now Playing' : isRecentlyPlayed ? 'Recently Played' : 'Offline'}
              </p>
            </div>
          </div>
          
          {isPlaying && spotifyData.duration_ms > 0 && (
            <div className="mt-5">
              <div className="w-full h-1 bg-slate-950 rounded-full overflow-hidden">
                <div 
                  className="h-full bg-emerald-500 transition-all duration-1000 ease-linear shadow-[0_0_8px_#10b981]"
                  style={{ width: `${progressPercent}%` }}
                ></div>
              </div>
              <div className="flex justify-between text-[9px] text-gray-500 mt-1.5 font-mono">
                <span>{formatTime(localProgressMs)}</span>
                <span>{formatTime(spotifyData.duration_ms)}</span>
              </div>
            </div>
          )}
        </div>
      ) : (
        <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Connecting to Spotify Web API...</p>
      )}
    </>
  );

  if (spotifyData?.track_url) {
    return (
      <a 
        href={spotifyData.track_url}
        target="_blank"
        rel="noopener noreferrer"
        className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-emerald-500/10 hover:border-emerald-500/30 relative overflow-hidden transition-all duration-300 group hover:shadow-lg hover:shadow-emerald-950/5 flex flex-col justify-center cursor-pointer"
      >
        {innerContent}
      </a>
    );
  }

  return (
    <section className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-white/5 relative overflow-hidden transition-all duration-300 flex flex-col justify-center">
      {innerContent}
    </section>
  );
}
