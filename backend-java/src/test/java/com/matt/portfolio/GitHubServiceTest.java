package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.HttpMethod;
import org.springframework.http.MediaType;
import org.springframework.test.context.TestPropertySource;
import org.springframework.test.web.client.MockRestServiceServer;
import org.springframework.web.client.RestTemplate;

import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.springframework.test.web.client.match.MockRestRequestMatchers.header;
import static org.springframework.test.web.client.match.MockRestRequestMatchers.method;
import static org.springframework.test.web.client.match.MockRestRequestMatchers.requestTo;
import static org.springframework.test.web.client.response.MockRestResponseCreators.withSuccess;
import static org.springframework.test.web.client.response.MockRestResponseCreators.withServerError;
import org.springframework.context.annotation.Import;

@SpringBootTest
@Import(NoOpTaskSchedulerConfig.class)
@TestPropertySource(properties = {
    "github.token=test-token",
    "spring.cache.type=none"
})
class GitHubServiceTest {

    @Autowired
    private GitHubService gitHubService;

    @Autowired
    private RestTemplate restTemplate;

    private MockRestServiceServer mockServer;

    @BeforeEach
    void setUp() {
        mockServer = MockRestServiceServer.bindTo(restTemplate).build();
    }

    @Test
    void getRecentActivity_ShouldParseGitHubEvents() {
        String mockEventsJson = """
            [
              {
                "type": "PushEvent",
                "created_at": "2023-10-27T10:00:00Z",
                "repo": {
                  "name": "mattDev0/portfolio-monorepo"
                },
                "payload": {
                  "head": "abc1234567890",
                  "ref": "refs/heads/main"
                }
              }
            ]
            """;
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(mockEventsJson, MediaType.APPLICATION_JSON));

        String mockCommitJson = """
            {
              "commit": {
                "message": "feat: add java tests\\n\\nMore details here."
              }
            }
            """;
        mockServer.expect(requestTo("https://api.github.com/repos/mattDev0/portfolio-monorepo/commits/abc1234567890"))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(mockCommitJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(1);
        assertThat(activity.get(0).repo()).isEqualTo("portfolio-monorepo");
        assertThat(activity.get(0).message()).isEqualTo("feat: add java tests");
        assertThat(activity.get(0).date()).isEqualTo("2023-10-27T10:00:00Z");
        assertThat(activity.get(0).hash()).isEqualTo("abc1234");
        
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleEmptyResponse() {
        // Clear mock server expectations between tests
        mockServer.reset();
        
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(withSuccess("[]", MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleRateLimitGracefully() {
        mockServer.reset();
        // First request (authenticated) fails with 403 Forbidden
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));
        // Second request (unauthenticated fallback) also fails
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleMalformedJson() {
        mockServer.reset();
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(withSuccess("not json at all", MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleServerError() {
        mockServer.reset();
        // First request (authenticated) fails with 500 Server Error
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(withServerError());
        // Second request (unauthenticated fallback) also fails
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(withServerError());

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldFallbackToUnauthenticatedAndSucceed() {
        mockServer.reset();
        
        // 1. Authenticated request fails with 403 Forbidden
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        // 2. Fallback unauthenticated request succeeds
        String mockEventsJson = """
            [
              {
                "type": "PushEvent",
                "created_at": "2023-10-27T10:00:00Z",
                "repo": {
                  "name": "mattDev0/portfolio-monorepo"
                },
                "payload": {
                  "head": "abc1234567890",
                  "ref": "refs/heads/main"
                }
              }
            ]
            """;
        mockServer.expect(requestTo("https://api.github.com/users/mattDev0/events/public"))
                  .andRespond(withSuccess(mockEventsJson, MediaType.APPLICATION_JSON));

        // 3. Secondary commit detail request (authenticated) fails
        mockServer.expect(requestTo("https://api.github.com/repos/mattDev0/portfolio-monorepo/commits/abc1234567890"))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        // 4. Secondary commit detail request (unauthenticated fallback) succeeds
        String mockCommitJson = """
            {
              "commit": {
                "message": "feat: add java tests\\n\\nMore details here."
              }
            }
            """;
        mockServer.expect(requestTo("https://api.github.com/repos/mattDev0/portfolio-monorepo/commits/abc1234567890"))
                  .andRespond(withSuccess(mockCommitJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(1);
        assertThat(activity.get(0).repo()).isEqualTo("portfolio-monorepo");
        assertThat(activity.get(0).message()).isEqualTo("feat: add java tests");
        
        mockServer.verify();
    }
}
