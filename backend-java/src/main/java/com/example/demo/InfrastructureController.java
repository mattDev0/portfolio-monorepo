package com.example.demo;

import org.springframework.web.bind.annotation.CrossOrigin;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequestMapping("/api/infrastructure")
// Allow the React frontend on port 5173 to access this endpoint
@CrossOrigin(origins = "http://localhost:5173")
public class InfrastructureController {

    @GetMapping("/metrics")
    public Map<String, Object> getMetrics() {
        // Using a Map to quickly generate a JSON response
        return Map.of(
            "engine", "Spring Boot Enterprise",
            "active_containers", 14,
            "database_status", "Connected (PostgreSQL)",
            "cloud_region", "Azure West Europe"
        );
    }
}