package com.matt.portfolio;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.cache.annotation.CachePut;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;
import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.HttpMethod;
import org.springframework.http.ResponseEntity;

import org.springframework.beans.factory.annotation.Value;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Service
public class GitHubService {

    private static final Logger logger = LoggerFactory.getLogger(GitHubService.class);

    @Value("${github.token:}")
    private String githubToken;

    private final RestTemplate restTemplate;
    private final ObjectMapper objectMapper;

    public record CommitInfo(String message) {}
    public record CommitResponse(CommitInfo commit) {}

    public GitHubService(RestTemplate restTemplate, ObjectMapper objectMapper) {
        this.restTemplate = restTemplate;
        this.objectMapper = objectMapper;
    }

    private String fetchUrl(String url) {
        if (githubToken != null && !githubToken.trim().isEmpty()) {
            HttpHeaders headers = new HttpHeaders();
            headers.set("Authorization", "Bearer " + githubToken.trim());
            HttpEntity<String> entity = new HttpEntity<>(headers);
            try {
                ResponseEntity<String> response = restTemplate.exchange(url, HttpMethod.GET, entity, String.class);
                return response.getBody();
            } catch (Exception e) {
                logger.warn("Authenticated request failed for url: {}. Falling back to unauthenticated. Error: {}", url, e.getMessage());
            }
        }
        // Fallback or default to unauthenticated request
        try {
            return restTemplate.getForObject(url, String.class);
        } catch (Exception e) {
            logger.error("Unauthenticated request failed for url: {}. Error: {}", url, e.getMessage(), e);
            return null;
        }
    }

    @Cacheable("githubActivity")
    public List<GitHubCommitActivity> getRecentActivity() {
        return fetchRecentActivity();
    }

    private List<GitHubCommitActivity> fetchRecentActivity() {
        List<GitHubCommitActivity> recentCommits = new ArrayList<>();
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
                                    CommitResponse commitRes = objectMapper.readValue(commitJson, CommitResponse.class);
                                    if (commitRes != null && commitRes.commit() != null && commitRes.commit().message() != null) {
                                        String fullMessage = commitRes.commit().message();
                                        message = fullMessage.split("\n")[0]; // Grab just the first line
                                        messageFound = true;
                                    }
                                }
                            } catch (Exception e) {
                                // If we hit a rate limit, fail silently and rely on the fallback
                                logger.warn("Rate limit hit or commit missing for hash: {}", hash);
                            }
                        }

                        // 3. Fallback (If secondary fetch failed or rate limit exceeded)
                        if (!messageFound && payload.has("ref")) {
                            String ref = payload.get("ref").asText();
                            message = "Pushed updates to branch: " + ref.replace("refs/heads/", "");
                        }

                        if (!message.startsWith("Merge branch")) {
                            recentCommits.add(new GitHubCommitActivity(
                                repoName.replace("mattDev0/", ""), 
                                message,
                                date,
                                hash
                            ));
                        }
                    }
                }
            }
        } catch (Exception e) {
            logger.error("Failed to fetch GitHub activity: {}", e.getMessage(), e);
        }

        return recentCommits;
    }

    // AUTOMATIC CACHE PRE-WARMING: Runs every 10 minutes to fetch fresh commits and update cache
    @CachePut(value = "githubActivity", unless = "#result == null or #result.isEmpty()")
    @Scheduled(fixedRate = 600000) 
    public List<GitHubCommitActivity> refreshGitHubCache() {
        logger.info("Pre-warming GitHub cache with fresh commits...");
        return fetchRecentActivity();
    }
}
