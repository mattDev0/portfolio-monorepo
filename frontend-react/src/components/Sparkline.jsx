import React from 'react';

export default function Sparkline({ data, color, max }) {
  if (!data || data.length < 2) {
    return (
      <div className="h-4 flex items-center justify-center text-[8px] text-gray-600 font-mono tracking-wider opacity-50">
        syncing...
      </div>
    );
  }

  const height = 18;
  const width = 80;
  const padding = 1;

  const minVal = 0;
  const maxVal = max !== undefined ? max : Math.max(Math.max(...data) * 1.1, 10);

  const points = data.map((val, index) => {
    const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
    const y = padding + (1 - (val - minVal) / (maxVal - minVal || 1)) * (height - 2 * padding);
    return `${x},${y}`;
  }).join(' ');

  return (
    <svg className="w-[80px] h-[18px] opacity-85" viewBox={`0 0 ${width} ${height}`}>
      <polyline
        fill="none"
        stroke={color}
        strokeWidth="1.2"
        strokeLinecap="round"
        strokeLinejoin="round"
        points={points}
      />
    </svg>
  );
}
