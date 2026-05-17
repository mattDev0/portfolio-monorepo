package com.example.demo;

import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.lang.management.ManagementFactory;
import java.util.Map;

@RestController
@RequestMapping("/api/infrastructure")
@CrossOrigin(origins = "*")
public class InfrastructureController {

    @GetMapping("/metrics")
    public Map<String, Object> getMetrics() {
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

        return Map.of(
            "engine", "Spring Boot " + org.springframework.boot.SpringBootVersion.getVersion(),
            "uptime", String.format("%dh %dm", uptimeHours, uptimeMinutes),
            "jvm_memory", String.format("%d MB / %d MB", usedMem, totalMem),
            "active_threads", String.valueOf(threadCount) + " Threads"
        );
    }
}
