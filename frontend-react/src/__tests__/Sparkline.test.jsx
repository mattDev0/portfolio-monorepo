import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import Sparkline from '../components/Sparkline';

describe('Sparkline', () => {
  it('renders "syncing..." when data is empty or less than 2 points', () => {
    const { rerender } = render(<Sparkline data={[]} color="red" />);
    expect(screen.getByText('syncing...')).toBeInTheDocument();

    rerender(<Sparkline data={[10]} color="red" />);
    expect(screen.getByText('syncing...')).toBeInTheDocument();
  });

  it('renders an SVG polyline when data has at least 2 points', () => {
    const { container } = render(<Sparkline data={[10, 20, 30]} color="blue" max={100} />);
    const polyline = container.querySelector('polyline');
    expect(polyline).toBeInTheDocument();
    expect(polyline).toHaveAttribute('stroke', 'blue');
    expect(polyline).toHaveAttribute('points');
    expect(polyline.getAttribute('points').length).toBeGreaterThan(0);
  });

  it('calculates max dynamically if not provided', () => {
    const { container } = render(<Sparkline data={[10, 50, 10]} color="green" />);
    const polyline = container.querySelector('polyline');
    // We just verify it renders without crashing and generates points
    expect(polyline).toHaveAttribute('points');
    const points = polyline.getAttribute('points');
    // Ensure there are 3 points generated for our 3 data points
    expect(points.split(' ').length).toBe(3);
  });
});
