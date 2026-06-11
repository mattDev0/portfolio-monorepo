package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.setup.MockMvcBuilders;

import java.util.List;

import static org.mockito.Mockito.when;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

class GitHubControllerTest {

    private MockMvc mockMvc;
    private GitHubService gitHubService;

    @BeforeEach
    void setUp() {
        gitHubService = Mockito.mock(GitHubService.class);
        GitHubController controller = new GitHubController(gitHubService);
        mockMvc = MockMvcBuilders.standaloneSetup(controller).build();
    }

    @Test
    void getActivity_ShouldReturnCommitActivity() throws Exception {
        List<GitHubCommitActivity> mockActivity = List.of(
            new GitHubCommitActivity("portfolio-monorepo", "feat: add tests", "2023-10-27T10:00:00Z", "abc1234"),
            new GitHubCommitActivity("other-repo", "fix: bug", "2023-10-26T10:00:00Z", "def5678")
        );

        when(gitHubService.getRecentActivity()).thenReturn(mockActivity);

        mockMvc.perform(get("/api/github/activity"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.length()").value(2))
                .andExpect(jsonPath("$[0].repo").value("portfolio-monorepo"))
                .andExpect(jsonPath("$[0].message").value("feat: add tests"))
                .andExpect(jsonPath("$[0].date").value("2023-10-27T10:00:00Z"))
                .andExpect(jsonPath("$[0].hash").value("abc1234"));
    }

    @Test
    void getActivity_ShouldReturnEmptyListWhenServiceReturnsEmpty() throws Exception {
        when(gitHubService.getRecentActivity()).thenReturn(List.of());

        mockMvc.perform(get("/api/github/activity"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.length()").value(0));
    }
}
