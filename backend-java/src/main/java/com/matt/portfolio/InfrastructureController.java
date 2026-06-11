package com.matt.portfolio;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.lang.management.ManagementFactory;
import java.util.Map;

@RestController
@RequestMapping("/api/infrastructure")
public class InfrastructureController {

    @GetMapping("/metrics")
    public InfrastructureMetrics getMetrics() {
        // Get Memory Stats
        Runtime runtime = Runtime.getRuntime();
        long totalMem = runtime.totalMemory() / (1024 * 1024);
        long freeMem = runtime.freeMemory() / (1024 * 1024);
        long usedMem = totalMem - freeMem;

        // Get Uptime
        long uptimeMillis = ManagementFactory.getRuntimeMXBean().getUptime();
        long uptimeHours = uptimeMillis / (1000 * 60 * 60);
        long uptimeMinutes = (uptimeMillis / (1000 * 60)) % 60;

        // Get Active Threads
        int threadCount = ManagementFactory.getThreadMXBean().getThreadCount();

        return new InfrastructureMetrics(
            "Spring Boot " + org.springframework.boot.SpringBootVersion.getVersion(),
            uptimeHours,
            uptimeMinutes,
            usedMem,
            totalMem,
            threadCount
        );
    }
}
