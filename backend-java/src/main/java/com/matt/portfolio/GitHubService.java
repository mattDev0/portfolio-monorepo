package com.matt.portfolio;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
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

import java.time.Instant;
import java.time.OffsetDateTime;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

@Service
public class GitHubService {

    private static final Logger logger = LoggerFactory.getLogger(GitHubService.class);

    static final String REPOS_URL = "https://api.github.com/users/mattDev0/repos?type=owner&sort=pushed&per_page=4";
    static final int COMMITS_PER_REPO = 10;
    private static final int FEED_LIMIT = 4;

    @Value("${github.token:}")
    private String githubToken;

    private final RestTemplate restTemplate;
    private final ObjectMapper objectMapper;

    @JsonIgnoreProperties(ignoreUnknown = true)
    public record RepoSummary(String name, boolean fork) {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    public record CommitActor(String date) {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    public record CommitInfo(String message, CommitActor author, CommitActor committer) {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    public record GitHubUser(String login) {}

    @JsonIgnoreProperties(ignoreUnknown = true)
    public record CommitResponse(String sha, CommitInfo commit, GitHubUser author) {}

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

    static String commitDate(CommitInfo commit) {
        if (commit.committer() != null && commit.committer().date() != null && !commit.committer().date().isBlank()) {
            return commit.committer().date();
        }
        if (commit.author() != null && commit.author().date() != null && !commit.author().date().isBlank()) {
            return commit.author().date();
        }
        return "";
    }

    private static Instant parseInstant(String date) {
        if (date == null || date.isBlank()) {
            return Instant.EPOCH;
        }
        try {
            return OffsetDateTime.parse(date).toInstant();
        } catch (Exception e) {
            return Instant.EPOCH;
        }
    }

    /**
     * The line to show for a commit, or null to leave it out of the feed.
     * A "Merge pull request #N from ..." commit shows the pull request title, which
     * GitHub puts in the message body. "Merge branch ..." commits (updating a branch
     * from main) carry no new work, so they are skipped, as are merges of bot
     * branches (Dependabot, ImgBot) and ImgBot's own commits.
     */
    static String feedMessage(String fullMessage) {
        String[] lines = fullMessage.split("\\R");
        String first = lines[0].trim();
        if (first.startsWith("Merge pull request")) {
            if (isBotBranchMerge(first)) {
                return null;
            }
            for (int i = 1; i < lines.length; i++) {
                if (!lines[i].isBlank()) {
                    return lines[i].trim();
                }
            }
            return null;
        }
        if (first.startsWith("Merge branch") || first.startsWith("Merge remote-tracking branch")) {
            return null;
        }
        if (first.startsWith("[ImgBot]")) {
            return null;
        }
        return first;
    }

    private static boolean isBotBranchMerge(String firstLine) {
        int from = firstLine.indexOf(" from ");
        if (from < 0) {
            return false;
        }
        String source = firstLine.substring(from + 6).trim();
        String branch = source.contains("/") ? source.substring(source.indexOf('/') + 1) : source;
        return branch.startsWith("dependabot/") || branch.startsWith("imgbot");
    }

    static boolean isBotAuthor(CommitResponse item) {
        return item.author() != null && item.author().login() != null && item.author().login().endsWith("[bot]");
    }

    @Cacheable(value = "githubActivity", unless = "#result == null or #result.isEmpty()")
    public List<GitHubCommitActivity> getRecentActivity() {
        return fetchRecentActivity();
    }

    private List<GitHubCommitActivity> fetchRecentActivity() {
        List<GitHubCommitActivity> recentCommits = new ArrayList<>();

        try {
            String rawReposJson = fetchUrl(REPOS_URL);
            if (rawReposJson == null) {
                return List.of();
            }

            RepoSummary[] repos = objectMapper.readValue(rawReposJson, RepoSummary[].class);
            if (repos == null) {
                return List.of();
            }

            for (RepoSummary repo : repos) {
                if (repo == null || repo.fork() || repo.name() == null || repo.name().isBlank()) {
                    continue;
                }

                String commitsUrl = "https://api.github.com/repos/mattDev0/" + repo.name() + "/commits?per_page=" + COMMITS_PER_REPO;
                String commitsJson = fetchUrl(commitsUrl);
                if (commitsJson == null) {
                    continue;
                }

                try {
                    CommitResponse[] commits = objectMapper.readValue(commitsJson, CommitResponse[].class);
                    if (commits == null) {
                        continue;
                    }

                    Set<String> seenMessages = new HashSet<>();
                    for (CommitResponse item : commits) {
                        if (item == null || item.commit() == null || item.commit().message() == null || isBotAuthor(item)) {
                            continue;
                        }

                        String message = feedMessage(item.commit().message());
                        if (message == null || !seenMessages.add(message)) {
                            continue;
                        }

                        String sha = item.sha();
                        String hash = (sha != null && sha.length() >= 7) ? sha.substring(0, 7) : "unknown";
                        String date = commitDate(item.commit());

                        recentCommits.add(new GitHubCommitActivity(
                            repo.name(),
                            message,
                            date,
                            hash
                        ));
                    }
                } catch (Exception e) {
                    logger.warn("Failed to parse commits for repo {}: {}", repo.name(), e.getMessage());
                }
            }

            recentCommits.sort(Comparator.comparing((GitHubCommitActivity a) -> parseInstant(a.date())).reversed());
            return recentCommits.stream().limit(FEED_LIMIT).toList();
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
