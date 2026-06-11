package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.setup.MockMvcBuilders;

import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

class InfrastructureControllerTest {

    private MockMvc mockMvc;

    @BeforeEach
    void setUp() {
        InfrastructureController controller = new InfrastructureController();
        mockMvc = MockMvcBuilders.standaloneSetup(controller).build();
    }

    @Test
    void getMetrics_ShouldReturnInfrastructureMetrics() throws Exception {
        mockMvc.perform(get("/api/infrastructure/metrics"))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.engine").exists())
                .andExpect(jsonPath("$.uptime").exists());
    }
}
