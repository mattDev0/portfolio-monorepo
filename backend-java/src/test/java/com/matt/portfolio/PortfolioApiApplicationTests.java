package com.matt.portfolio;

import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

import static org.assertj.core.api.Assertions.assertThat;

@SpringBootTest
class PortfolioApiApplicationTests {

	@Autowired
	private GitHubController gitHubController;

	@Autowired
	private InfrastructureController infrastructureController;

	@Test
	void contextLoads() {
		assertThat(gitHubController).isNotNull();
		assertThat(infrastructureController).isNotNull();
	}

}
