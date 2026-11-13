package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.http.HttpMethod;
import org.springframework.http.MediaType;
import org.springframework.cache.annotation.CachePut;
import org.springframework.cache.annotation.Cacheable;
import org.springframework.test.context.TestPropertySource;
import org.springframework.test.web.client.MockRestServiceServer;
import org.springframework.web.client.RestTemplate;

import java.lang.reflect.Method;
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

    private static final String REPOS_URL = GitHubService.REPOS_URL;

    private static String commitsUrl(String repo) {
        return "https://api.github.com/repos/mattDev0/" + repo + "/commits?per_page=" + GitHubService.COMMITS_PER_REPO;
    }

    @Test
    void getRecentActivity_ShouldParseGitHubCommitsAcrossReposAndSortByDate() {
        String mockReposJson = """
            [
              { "name": "devops-control-center", "fork": false },
              { "name": "some-forked-repo", "fork": true },
              { "name": "portfolio-monorepo", "fork": false }
            ]
            """;
        mockServer.expect(requestTo(REPOS_URL))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(mockReposJson, MediaType.APPLICATION_JSON));

        String dccCommitsJson = """
            [
              {
                "sha": "1111111abcdef00",
                "commit": {
                  "message": "fix: dcc newest",
                  "committer": { "date": "2026-11-13T06:00:52Z" }
                }
              },
              {
                "sha": "2222222abcdef00",
                "commit": {
                  "message": "chore: dcc older",
                  "committer": { "date": "2026-11-12T10:00:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("devops-control-center")))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(dccCommitsJson, MediaType.APPLICATION_JSON));

        String portfolioCommitsJson = """
            [
              {
                "sha": "3333333abcdef00",
                "commit": {
                  "message": "feat: portfolio newer",
                  "committer": { "date": "2026-11-13T05:30:00Z" }
                }
              },
              {
                "sha": "4444444abcdef00",
                "commit": {
                  "message": "docs: portfolio middle",
                  "committer": { "date": "2026-11-13T04:00:00Z" }
                }
              },
              {
                "sha": "5555555abcdef00",
                "commit": {
                  "message": "chore: portfolio oldest",
                  "committer": { "date": "2026-11-11T09:00:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(portfolioCommitsJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(4);
        assertThat(activity).extracting(GitHubCommitActivity::repo)
                .containsExactly("devops-control-center", "portfolio-monorepo", "portfolio-monorepo", "devops-control-center");
        assertThat(activity).extracting(GitHubCommitActivity::message)
                .containsExactly("fix: dcc newest", "feat: portfolio newer", "docs: portfolio middle", "chore: dcc older");
        assertThat(activity).extracting(GitHubCommitActivity::hash)
                .containsExactly("1111111", "3333333", "4444444", "2222222");

        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleEmptyResponse() {
        mockServer.reset();

        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess("[]", MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleRateLimitGracefully() {
        mockServer.reset();
        // First request (authenticated) fails with 403 Forbidden
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));
        // Second request (unauthenticated fallback) also fails
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleMalformedJson() {
        mockServer.reset();
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess("not json at all", MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHandleServerError() {
        mockServer.reset();
        // First request (authenticated) fails with 500 Server Error
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withServerError());
        // Second request (unauthenticated fallback) also fails
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withServerError());

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).isEmpty();
        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldFallbackToUnauthenticatedAndSucceed() {
        mockServer.reset();

        // 1. Authenticated repos request fails with 403 Forbidden
        mockServer.expect(requestTo(REPOS_URL))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        // 2. Fallback unauthenticated repos request succeeds
        String mockReposJson = """
            [
              { "name": "portfolio-monorepo", "fork": false }
            ]
            """;
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess(mockReposJson, MediaType.APPLICATION_JSON));

        // 3. Authenticated commits request fails
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        // 4. Fallback unauthenticated commits request succeeds
        String mockCommitsJson = """
            [
              {
                "sha": "abc1234567890",
                "commit": {
                  "message": "feat: add java tests\\n\\nMore details here.",
                  "committer": { "date": "2023-10-27T10:00:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andRespond(withSuccess(mockCommitsJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(1);
        assertThat(activity.get(0).repo()).isEqualTo("portfolio-monorepo");
        assertThat(activity.get(0).message()).isEqualTo("feat: add java tests");
        assertThat(activity.get(0).hash()).isEqualTo("abc1234");
        assertThat(activity.get(0).date()).isEqualTo("2023-10-27T10:00:00Z");

        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldContinueWhenOneRepoCommitsFetchFails() {
        mockServer.reset();

        String mockReposJson = """
            [
              { "name": "devops-control-center", "fork": false },
              { "name": "portfolio-monorepo", "fork": false }
            ]
            """;
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess(mockReposJson, MediaType.APPLICATION_JSON));

        // First repo fails both authenticated and fallback
        mockServer.expect(requestTo(commitsUrl("devops-control-center")))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));
        mockServer.expect(requestTo(commitsUrl("devops-control-center")))
                  .andRespond(org.springframework.test.web.client.response.MockRestResponseCreators.withStatus(org.springframework.http.HttpStatus.FORBIDDEN));

        // Second repo succeeds
        String portfolioCommitsJson = """
            [
              {
                "sha": "deadbee1234567",
                "commit": {
                  "message": "fix: resilient feed",
                  "committer": { "date": "2026-11-13T06:00:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andRespond(withSuccess(portfolioCommitsJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(1);
        assertThat(activity.get(0).repo()).isEqualTo("portfolio-monorepo");
        assertThat(activity.get(0).message()).isEqualTo("fix: resilient feed");
        assertThat(activity.get(0).hash()).isEqualTo("deadbee");

        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldHaveCacheableAnnotationWithUnlessCondition() throws NoSuchMethodException {
        Method getActivityMethod = GitHubService.class.getMethod("getRecentActivity");
        Cacheable cacheable = getActivityMethod.getAnnotation(Cacheable.class);

        assertThat(cacheable).isNotNull();
        assertThat(cacheable.value()).containsExactly("githubActivity");
        assertThat(cacheable.unless()).isEqualTo("#result == null or #result.isEmpty()");

        Method refreshCacheMethod = GitHubService.class.getMethod("refreshGitHubCache");
        CachePut cachePut = refreshCacheMethod.getAnnotation(CachePut.class);
        assertThat(cachePut).isNotNull();
        assertThat(cachePut.value()).containsExactly("githubActivity");
        assertThat(cachePut.unless()).isEqualTo(cacheable.unless());
    }

    @Test
    void getRecentActivity_ShouldIgnoreUnknownFieldsInRealisticCommitPayload() {
        mockServer.reset();

        String mockReposJson = """
            [
              {
                "id": 999,
                "name": "portfolio-monorepo",
                "full_name": "mattDev0/portfolio-monorepo",
                "fork": false,
                "pushed_at": "2026-11-12T10:00:00Z"
              }
            ]
            """;
        mockServer.expect(requestTo(REPOS_URL))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(mockReposJson, MediaType.APPLICATION_JSON));

        String realisticCommitJson = """
            [
              {
                "sha": "deadbeef1234567",
                "node_id": "C_kwDOExampleNodeId",
                "html_url": "https://github.com/mattDev0/portfolio-monorepo/commit/deadbeef1234567",
                "author": {
                  "login": "mattDev0",
                  "id": 12345
                },
                "committer": {
                  "login": "mattDev0",
                  "id": 12345
                },
                "parents": [
                  {
                    "sha": "000000000000000",
                    "url": "https://api.github.com/repos/mattDev0/portfolio-monorepo/commits/000000000000000"
                  }
                ],
                "commit": {
                  "message": "fix: deserialize full commit payload\\n\\nDetailed body line.",
                  "author": {
                    "name": "mattDev0",
                    "email": "amatthew642@gmail.com",
                    "date": "2026-11-12T09:59:00Z"
                  },
                  "committer": {
                    "name": "mattDev0",
                    "email": "amatthew642@gmail.com",
                    "date": "2026-11-12T10:00:00Z"
                  },
                  "tree": {
                    "sha": "111111111111111",
                    "url": "https://api.github.com/repos/mattDev0/portfolio-monorepo/git/trees/111111111111111"
                  },
                  "url": "https://api.github.com/repos/mattDev0/portfolio-monorepo/git/commits/deadbeef1234567"
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andExpect(method(HttpMethod.GET))
                  .andExpect(header("Authorization", "Bearer test-token"))
                  .andRespond(withSuccess(realisticCommitJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).hasSize(1);
        assertThat(activity.get(0).message()).isEqualTo("fix: deserialize full commit payload");
        assertThat(activity.get(0).date()).isEqualTo("2026-11-12T10:00:00Z");
        assertThat(activity.get(0).hash()).isEqualTo("deadbee");

        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldShowPullRequestTitlesSkipBranchMergesAndDeduplicate() {
        mockServer.reset();

        String mockReposJson = """
            [
              { "name": "portfolio-monorepo", "fork": false }
            ]
            """;
        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess(mockReposJson, MediaType.APPLICATION_JSON));

        String mockCommitsJson = """
            [
              {
                "sha": "aaaaaaa1111111",
                "commit": {
                  "message": "Merge pull request #64 from mattDev0/fix/thing\\n\\nfix: report Spotify errors correctly",
                  "committer": { "date": "2026-11-12T12:00:00Z" }
                }
              },
              {
                "sha": "bbbbbbb2222222",
                "commit": {
                  "message": "Merge branch 'main' into fix/thing",
                  "committer": { "date": "2026-11-12T11:30:00Z" }
                }
              },
              {
                "sha": "ddddddd4444444",
                "commit": {
                  "message": "fix: report Spotify errors correctly",
                  "committer": { "date": "2026-11-12T11:00:00Z" }
                }
              },
              {
                "sha": "ccccccc3333333",
                "commit": {
                  "message": "fix: thing\\n\\nLonger body.",
                  "committer": { "date": "2026-11-12T10:00:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andRespond(withSuccess(mockCommitsJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).extracting(GitHubCommitActivity::message)
                .containsExactly("fix: report Spotify errors correctly", "fix: thing");
        assertThat(activity).extracting(GitHubCommitActivity::hash)
                .containsExactly("aaaaaaa", "ccccccc");

        mockServer.verify();
    }

    @Test
    void getRecentActivity_ShouldSkipBotWork() {
        mockServer.reset();

        mockServer.expect(requestTo(REPOS_URL))
                  .andRespond(withSuccess("""
            [ { "name": "portfolio-monorepo", "fork": false } ]
            """, MediaType.APPLICATION_JSON));

        String mockCommitsJson = """
            [
              {
                "sha": "aaaaaaa1111111",
                "author": { "login": "mattDev0" },
                "commit": {
                  "message": "Merge pull request #59 from mattDev0/dependabot/npm_and_yarn/x\\n\\nchore(deps): bump things",
                  "committer": { "date": "2026-11-13T06:00:00Z" }
                }
              },
              {
                "sha": "bbbbbbb2222222",
                "author": { "login": "mattDev0" },
                "commit": {
                  "message": "Merge pull request #47 from mattDev0/imgbot\\n\\n[ImgBot] Optimize images",
                  "committer": { "date": "2026-11-13T05:59:00Z" }
                }
              },
              {
                "sha": "ccccccc3333333",
                "author": { "login": "dependabot[bot]" },
                "commit": {
                  "message": "chore(deps): bump other things",
                  "committer": { "date": "2026-11-13T05:58:00Z" }
                }
              },
              {
                "sha": "ddddddd4444444",
                "author": null,
                "commit": {
                  "message": "[ImgBot] Optimize images",
                  "committer": { "date": "2026-11-13T05:57:00Z" }
                }
              },
              {
                "sha": "eeeeeee5555555",
                "author": { "login": "mattDev0" },
                "commit": {
                  "message": "fix: real work",
                  "committer": { "date": "2026-11-13T05:56:00Z" }
                }
              }
            ]
            """;
        mockServer.expect(requestTo(commitsUrl("portfolio-monorepo")))
                  .andRespond(withSuccess(mockCommitsJson, MediaType.APPLICATION_JSON));

        List<GitHubCommitActivity> activity = gitHubService.getRecentActivity();

        assertThat(activity).extracting(GitHubCommitActivity::message).containsExactly("fix: real work");
        mockServer.verify();
    }

    @Test
    void feedMessage_ShouldSkipBotBranchMerges() {
        assertThat(GitHubService.feedMessage("Merge pull request #59 from mattDev0/dependabot/npm_and_yarn/x\n\nchore(deps): bump")).isNull();
        assertThat(GitHubService.feedMessage("Merge pull request #47 from mattDev0/imgbot\n\n[ImgBot] Optimize images")).isNull();
        assertThat(GitHubService.feedMessage("[ImgBot] Optimize images")).isNull();
        assertThat(GitHubService.feedMessage("Merge pull request #68 from mattDev0/deps/sysinfo\n\nchore(rust): upgrade sysinfo")).isEqualTo("chore(rust): upgrade sysinfo");
    }

    @Test
    void feedMessage_ShouldUseTitlesAndSkipBranchMerges() {
        assertThat(GitHubService.feedMessage("Merge pull request #64 from mattDev0/fix/thing\n\nfix: thing")).isEqualTo("fix: thing");
        assertThat(GitHubService.feedMessage("Merge pull request #64 from mattDev0/fix/thing")).isNull();
        assertThat(GitHubService.feedMessage("Merge branch 'main' into feature")).isNull();
        assertThat(GitHubService.feedMessage("Merge remote-tracking branch 'origin/main'")).isNull();
        assertThat(GitHubService.feedMessage("fix: merge pull request handling\r\n\r\nbody")).isEqualTo("fix: merge pull request handling");
    }
}
