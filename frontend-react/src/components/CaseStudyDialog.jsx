import { useEffect, useRef } from 'react';
import { createPortal } from 'react-dom';
import { X, Monitor, Coffee, Cpu, Box, ExternalLink } from 'lucide-react';
import TerminalSimulator from './TerminalSimulator';

export default function CaseStudyDialog({ isOpen, onClose, config }) {
  const dialogRef = useRef(null);
  const closeButtonRef = useRef(null);

  // Manage focus and body scroll lock
  useEffect(() => {
    if (!isOpen) return;

    // Track the previously active element to restore focus when closed
    const previousActiveElement = document.activeElement;

    // Set scroll lock
    document.body.style.overflow = 'hidden';

    // Focus close button initially
    if (closeButtonRef.current) {
      closeButtonRef.current.focus();
    }

    // Trap focus inside dialog elements
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') {
        onClose();
        return;
      }

      if (e.key === 'Tab') {
        if (!dialogRef.current) return;
        const focusable = dialogRef.current.querySelectorAll(
          'a[href], button:not([disabled]), textarea, input, select, [tabindex]:not([tabindex="-1"])'
        );
        if (focusable.length === 0) return;
        const first = focusable[0];
        const last = focusable[focusable.length - 1];

        if (e.shiftKey) {
          if (document.activeElement === first) {
            last.focus();
            e.preventDefault();
          }
        } else {
          if (document.activeElement === last) {
            first.focus();
            e.preventDefault();
          }
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      document.body.style.overflow = '';
      if (previousActiveElement && typeof previousActiveElement.focus === 'function') {
        previousActiveElement.focus();
      }
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  const handleBackdropClick = (e) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const dialogMarkup = (
    <div
      onClick={handleBackdropClick}
      className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 overflow-y-auto"
    >
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="casestudy-title"
        className="relative bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] w-full max-w-3xl p-6 md:p-8 shadow-xl max-h-[90vh] overflow-y-auto"
      >
        {/* Header */}
        <div className="flex justify-between items-start border-b border-[var(--border-default)] pb-4 mb-6">
          <div>
            <span className="text-[10px] font-bold text-[var(--accent-primary)] uppercase tracking-widest font-mono">
              Case Study & Architecture
            </span>
            <h2 id="casestudy-title" className="text-xl font-bold text-[var(--fg-default)] mt-1">
              DevOps Control Center
            </h2>
            <p className="text-xs text-[var(--fg-muted)] mt-1">
              A custom end-to-end telemetry and K8s orchestration dashboard.
            </p>
          </div>
          <button
            ref={closeButtonRef}
            onClick={onClose}
            aria-label="Close dialog"
            className="text-[var(--fg-muted)] hover:text-[var(--fg-default)] p-1.5 hover:bg-[var(--bg-elevated)] rounded-[var(--radius-sm)] border border-transparent hover:border-[var(--border-default)] transition-colors cursor-pointer focus-visible:outline-none"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content Body */}
        <div className="space-y-6">
          {/* Overview */}
          <div>
            <h3 className="text-xs font-bold uppercase text-[var(--fg-muted)] tracking-wider mb-2 font-mono">
              Platform Overview
            </h3>
            <p className="text-[var(--fg-default)] text-sm leading-relaxed">
              This custom dashboard unifies server monitoring, remote terminal execution, Kubernetes deployment management, and CI/CD tracking into a single view. By proxying WebSocket traffic and stream channels securely, it allows remote administration from any browser interface.
            </p>
          </div>

          {/* Terminal Simulator Showcase */}
          <div>
            <h3 className="text-xs font-bold uppercase text-[var(--fg-muted)] tracking-wider mb-2 font-mono">
              Simulated Interactive PTY Terminal
            </h3>
            <p className="text-[var(--fg-subtle)] text-[11px] mb-3 leading-relaxed">
              Below is a visual simulation of the live PTY console connection which streams raw shell sessions over secure WebSockets directly proxied from the Spring gateway to the Rust systems agent.
            </p>
            <TerminalSimulator active={isOpen} />
          </div>

          {/* System Architecture Diagram */}
          <div>
            <h3 className="text-xs font-bold uppercase text-[var(--fg-muted)] tracking-wider mb-1 font-mono">
              Microservices Topology
            </h3>
            <p className="text-[var(--fg-subtle)] text-[11px] mb-4 leading-relaxed font-mono">
              The infrastructure operates inside the K3s namespace <code className="text-[var(--accent-primary)] font-mono text-[10px] bg-[var(--bg-inset)] border border-[var(--border-default)] px-1 py-0.5 rounded">devops</code> behind an Nginx reverse proxy.
            </p>

            {/* Architecture Visual Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 items-stretch justify-center my-4 py-4 px-3 border border-[var(--border-default)] bg-[var(--bg-inset)] rounded-[var(--radius-lg)]">
              {/* Client */}
              <div className="flex flex-col items-center justify-between bg-[var(--bg-surface)] p-3 rounded-[var(--radius-md)] border border-[var(--border-default)] text-center">
                <Monitor className="w-5 h-5 text-[var(--fg-muted)] mb-1" />
                <span className="text-[10px] font-bold text-[var(--fg-default)] uppercase tracking-wider mt-1">Client Dashboard</span>
                <span className="text-[9px] text-[var(--fg-subtle)] mt-1 font-mono">Vite + React UI</span>
              </div>
              {/* Spring Gateway */}
              <div className="flex flex-col items-center justify-between bg-[var(--bg-surface)] p-3 rounded-[var(--radius-md)] border border-[var(--border-default)] text-center">
                <Coffee className="w-5 h-5 text-[var(--status-success)] mb-1" />
                <span className="text-[10px] font-bold text-[var(--status-success)] uppercase tracking-wider mt-1">Spring Gateway</span>
                <span className="text-[9px] text-[var(--fg-subtle)] mt-1 font-mono">JWT & WebSockets</span>
              </div>
              {/* Rust Agent */}
              <div className="flex flex-col items-center justify-between bg-[var(--bg-surface)] p-3 rounded-[var(--radius-md)] border border-[var(--border-default)] text-center">
                <Cpu className="w-5 h-5 text-[var(--accent-primary)] mb-1" />
                <span className="text-[10px] font-bold text-[var(--accent-primary)] uppercase tracking-wider mt-1">Rust Agent</span>
                <span className="text-[9px] text-[var(--fg-subtle)] mt-1 font-mono">kube-rs & PTY</span>
              </div>
              {/* K3s API */}
              <div className="flex flex-col items-center justify-between bg-[var(--bg-surface)] p-3 rounded-[var(--radius-md)] border border-[var(--border-default)] text-center">
                <Box className="w-5 h-5 text-blue-400 mb-1" />
                <span className="text-[10px] font-bold text-blue-400 uppercase tracking-wider mt-1">K3s Cluster API</span>
                <span className="text-[9px] text-[var(--fg-subtle)] mt-1 font-mono">Logs & Replicas</span>
              </div>
            </div>
          </div>
        </div>

        {/* Modal Footer Controls */}
        <div className="mt-8 pt-4 border-t border-[var(--border-default)] flex flex-col sm:flex-row gap-3 justify-end items-center">
          <a
            href="https://devops.mattdev0.tech"
            target="_blank"
            rel="noopener noreferrer"
            className="w-full sm:w-auto px-4 py-2 bg-[var(--accent-primary)] hover:bg-[var(--accent-primary-hover)] text-white font-bold rounded-[var(--radius-md)] text-xs text-center transition-colors cursor-pointer flex items-center justify-center space-x-1.5 focus-visible:outline-none"
          >
            <span>Launch Live Demo</span>
            <ExternalLink className="w-3 h-3" />
          </a>
          <a
            href={config?.projects?.[0]?.githubUrl || "https://github.com/mattDev0/devops-control-center"}
            target="_blank"
            rel="noopener noreferrer"
            className="w-full sm:w-auto px-4 py-2 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-[var(--fg-default)] font-semibold rounded-[var(--radius-md)] text-xs text-center transition-colors cursor-pointer focus-visible:outline-none"
          >
            View Repository
          </a>
          <button
            onClick={onClose}
            className="w-full sm:w-auto px-4 py-2 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-[var(--fg-default)] font-semibold rounded-[var(--radius-md)] text-xs cursor-pointer focus-visible:outline-none"
          >
            Close
          </button>
        </div>

        <p className="text-[9px] text-[var(--fg-subtle)] text-center mt-4 font-mono">
          💡 Tip: On the live dashboard, you can bypass JWT login by clicking the "Guest Login" button to explore in read-only mode.
        </p>
      </div>
    </div>
  );

  return createPortal(dialogMarkup, document.body);
}
