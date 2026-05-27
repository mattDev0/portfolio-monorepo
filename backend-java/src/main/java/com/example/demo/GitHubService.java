package com.example.demo;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;
import org.springframework.cache.annotation.CacheEvict;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.ResponseEntity;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Service
public class GitHubService {

    private final RestTemplate restTemplate = new RestTemplate();
    private final ObjectMapper objectMapper = new ObjectMapper(); 

    private String fetchUrl(String url) {
        String token = System.getenv("GITHUB_TOKEN");
        if (token != null && !token.trim().isEmpty()) {
            HttpHeaders headers = new HttpHeaders();
            headers.set("Authorization", "Bearer " + token.trim());
            HttpEntity<String> entity = new HttpEntity<>(headers);
            try {
                ResponseEntity<String> response = restTemplate.exchange(url, HttpMethod.GET, entity, String.class);
                return response.getBody();
            } catch (Exception e) {
                System.err.println("Authenticated request failed for url: " + url + ". Error: " + e.getMessage() + ". Retrying unauthenticated.");
            }
        }
        // Fallback or default to unauthenticated request
        return restTemplate.getForObject(url, String.class);
    }

    @Cacheable("githubActivity")
    public List<Map<String, String>> getRecentActivity() {
        List<Map<String, String>> recentCommits = new ArrayList<>();
        String url = "https://api.github.com/users/mattDev0/events/public";

        try {
            String rawJson = fetchUrl(url);

            if (rawJson != null) {
                JsonNode rootNode = objectMapper.readTree(rawJson);

                for (JsonNode event : rootNode) {
                    if (recentCommits.size() >= 4) break;

                    if ("PushEvent".equals(event.get("type").asText())) {
                        String repoName = event.get("repo").get("name").asText();
                        String date = event.get("created_at").asText();
                        JsonNode payload = event.get("payload");

                        String message = "Pushed repository update";
                        String hash = "unknown";
                        String fullSha = null;

                        // 1. Extract the Commit Hash (SHA)
                        if (payload.has("head")) {
                            fullSha = payload.get("head").asText();
                            if (fullSha.length() >= 7) {
                                hash = fullSha.substring(0, 7);
                            }
                        }

                        // 2. Secondary API Call: Fetch the actual commit message 
                        // (Required because GitHub removed the 'commits' array from PushEvents in late 2025)
                        boolean messageFound = false;
                        if (fullSha != null) {
                            try {
                                String commitUrl = "https://api.github.com/repos/" + repoName + "/commits/" + fullSha;
                                String commitJson = fetchUrl(commitUrl);
                                
                                if (commitJson != null) {
                                    JsonNode commitRoot = objectMapper.readTree(commitJson);
                                    if (commitRoot.has("commit") && commitRoot.get("commit").has("message")) {
                                        String fullMessage = commitRoot.get("commit").get("message").asText();
                                        message = fullMessage.split("\n")[0]; // Grab just the first line
                                        messageFound = true;
                                    }
                                }
                            } catch (Exception e) {
                                // If we hit a rate limit, fail silently and rely on the fallback
                                System.err.println("Rate limit hit or commit missing for hash: " + hash);
                            }
                        }

                        // 3. Fallback (If secondary fetch failed or rate limit exceeded)
                        if (!messageFound && payload.has("ref")) {
                            String ref = payload.get("ref").asText();
                            message = "Pushed updates to branch: " + ref.replace("refs/heads/", "");
                        }

                        if (!message.startsWith("Merge branch")) {
                            recentCommits.add(Map.of(
                                "repo", repoName.replace("mattDev0/", ""), 
                                "message", message,
                                "date", date,
                                "hash", hash
                            ));
                        }
                    }
                }
            }
        } catch (Exception e) {
            System.err.println("Failed to fetch GitHub activity: " + e.getMessage());
        }

        return recentCommits;
    }

    // AUTOMATIC CACHE CLEAR: Runs every 10 minutes to fetch fresh commits
    @CacheEvict(value = "githubActivity", allEntries = true)
    @Scheduled(fixedRate = 600000) 
    public void emptyGitHubCache() {
        System.out.println("Flushing GitHub cache for fresh commits...");
    }
}