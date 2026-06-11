package com.matt.portfolio;

public record InfrastructureMetrics(
    String engine,
    long uptime_hours,
    long uptime_minutes,
    long jvm_memory_used_mb,
    long jvm_memory_total_mb,
    int active_threads
) {}
