package com.example.demo;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

@Service
public class GitHubService {

    private final RestTemplate restTemplate = new RestTemplate();
    private final ObjectMapper objectMapper = new ObjectMapper(); // Add the manual parser

    @Cacheable(value = "githubActivity", sync = true)
    public List<Map<String, String>> fetchRecentCommits() {
        String url = "https://api.github.com/users/mattDev0/events/public";
        List<Map<String, String>> recentCommits = new ArrayList<>();

        try {
            // 1. Fetch the data as a plain raw String instead of a JsonNode array
            String rawJson = restTemplate.getForObject(url, String.class);

            if (rawJson != null) {
                // 2. Safely parse the String into a JSON tree
                JsonNode rootNode = objectMapper.readTree(rawJson);

                // 3. Loop through the parsed tree
                for (JsonNode event : rootNode) {
                    if (recentCommits.size() >= 5) break; 

                    if ("PushEvent".equals(event.get("type").asText())) {
                        String repoName = event.get("repo").get("name").asText();
                        String date = event.get("created_at").asText();
                        JsonNode payload = event.get("payload");
                        JsonNode commits = payload.get("commits");

                        String message = "Pushed repository update"; // Default fallback

                        // If GitHub provided commit details, use the actual commit message
                        if (commits != null && commits.isArray() && commits.size() > 0) {
                            message = commits.get(0).get("message").asText();
                        } 
                        // If no commits array, extract the branch name that was updated
                        else if (payload.has("ref")) {
                            String ref = payload.get("ref").asText();
                            String branch = ref.replace("refs/heads/", "");
                            message = "Pushed updates to branch: " + branch;
                        }

                        // Filter out generic merge spam
                        if (!message.startsWith("Merge branch")) {
                            recentCommits.add(Map.of(
                                "repo", repoName.replace("mattDev0/", ""), 
                                "message", message,
                                "date", date
                            ));
                        }
                    }
                }
            }
        } catch (Exception e) {
            System.err.println("GITHUB FETCH ERROR: " + e.getMessage());
            recentCommits.add(Map.of(
                "repo", "System",
                "message", "Live activity feed temporarily unavailable.",
                "date", "Just now"
            ));
        }

        return recentCommits;
    }
}