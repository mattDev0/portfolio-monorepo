import React from 'react';

export default function TopologyNode({ id, icon, title, tech, hoveredTopologyNode, setHoveredTopologyNode }) {
  const isHovered = hoveredTopologyNode === id;
  
  let accentBorder = "border-[var(--color-border-primary)]";
  let accentBg = "bg-[var(--color-bg-surface-deep)] hover:border-[var(--color-accent-emerald-border)]";
  let accentText = "text-[var(--color-text-secondary)]";
  
  if (isHovered) {
    if (id === 'rust') {
      accentBorder = "border-[var(--color-accent-orange-border)] shadow-[0_0_12px_rgba(249,115,22,0.15)] animate-pulse";
      accentBg = "bg-[var(--color-accent-orange-bg)]";
      accentText = "text-[var(--color-accent-orange)]";
    } else if (id === 'java') {
      accentBorder = "border-[var(--color-accent-rose)]/40 shadow-[0_0_12px_rgba(244,63,94,0.15)] animate-pulse";
      accentBg = "bg-[var(--color-accent-rose)]/5";
      accentText = "text-[var(--color-accent-rose)]";
    } else if (id === 'frontend' || id === 'client') {
      accentBorder = "border-[var(--color-accent-teal)]/40 shadow-[0_0_12px_rgba(20,184,166,0.15)] animate-pulse";
      accentBg = "bg-[var(--color-accent-teal)]/5";
      accentText = "text-[var(--color-accent-teal)]";
    } else if (id === 'nginx') {
      accentBorder = "border-[var(--color-accent-emerald-border)] shadow-[0_0_12px_rgba(16,185,129,0.15)] animate-pulse";
      accentBg = "bg-[var(--color-accent-emerald-bg)]";
      accentText = "text-[var(--color-accent-emerald)]";
    } else if (id === 'k8s') {
      accentBorder = "border-[var(--color-accent-cyan)]/50 shadow-[0_0_12px_rgba(6,182,212,0.2)]";
      accentBg = "bg-[var(--color-accent-cyan)]/5";
      accentText = "text-[var(--color-accent-cyan)]";
    } else {
      accentBorder = "border-[var(--color-accent-emerald-border)] shadow-[0_0_12px_rgba(16,185,129,0.15)]";
      accentBg = "bg-[var(--color-accent-emerald-bg)]";
      accentText = "text-[var(--color-accent-emerald)]";
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
      <span className="text-[8px] md:text-[9px] text-[var(--color-text-muted)] font-mono mt-0.5">{tech}</span>
    </div>
  );
}
