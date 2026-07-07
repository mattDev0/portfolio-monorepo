import React from 'react';

export default function TopologyNode({ id, icon, title, tech, hoveredTopologyNode, setHoveredTopologyNode }) {
  const isHovered = hoveredTopologyNode === id;
  
  let accentBorder = "border-white/5";
  let accentBg = "bg-slate-950/20 hover:border-indigo-500/20";
  let accentText = "text-gray-300";
  
  if (isHovered) {
    if (id === 'rust') {
      accentBorder = "border-orange-500/40 shadow-[0_0_12px_rgba(249,115,22,0.15)] animate-pulse";
      accentBg = "bg-orange-500/5";
      accentText = "text-orange-400";
    } else if (id === 'java') {
      accentBorder = "border-emerald-500/40 shadow-[0_0_12px_rgba(16,185,129,0.15)] animate-pulse";
      accentBg = "bg-emerald-500/5";
      accentText = "text-emerald-400";
    } else if (id === 'frontend' || id === 'client') {
      accentBorder = "border-blue-500/40 shadow-[0_0_12px_rgba(59,130,246,0.15)] animate-pulse";
      accentBg = "bg-blue-500/5";
      accentText = "text-blue-400";
    } else if (id === 'nginx') {
      accentBorder = "border-indigo-500/40 shadow-[0_0_12px_rgba(99,102,241,0.15)] animate-pulse";
      accentBg = "bg-indigo-500/5";
      accentText = "text-indigo-400";
    } else if (id === 'k8s') {
      accentBorder = "border-blue-400/50 shadow-[0_0_12px_rgba(96,165,250,0.2)]";
      accentBg = "bg-blue-400/5";
      accentText = "text-blue-400";
    } else {
      accentBorder = "border-indigo-500/40 shadow-[0_0_12px_rgba(99,102,241,0.15)]";
      accentBg = "bg-indigo-500/5";
      accentText = "text-indigo-400";
    }
  }

  return (
    <div
      onMouseEnter={() => setHoveredTopologyNode(id)}
      onMouseLeave={() => setHoveredTopologyNode(null)}
      className={`p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none ${accentBorder} ${accentBg} ${isHovered ? 'scale-[1.02]' : ''}`}
    >
      <span className="text-lg md:text-xl mb-0.5">{icon}</span>
      <span className={`text-[9px] md:text-[10px] font-bold uppercase tracking-wider ${accentText}`}>{title}</span>
      <span className="text-[8px] md:text-[9px] text-gray-500 font-mono mt-0.5">{tech}</span>
    </div>
  );
}
