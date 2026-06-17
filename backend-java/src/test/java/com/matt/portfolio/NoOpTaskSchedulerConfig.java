package com.matt.portfolio;

import org.springframework.boot.test.context.TestConfiguration;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Primary;
import org.springframework.scheduling.TaskScheduler;
import org.springframework.scheduling.concurrent.ThreadPoolTaskScheduler;

import java.time.Duration;
import java.time.Instant;
import java.util.concurrent.ScheduledFuture;

@TestConfiguration
public class NoOpTaskSchedulerConfig {

    @Bean
    @Primary
    public TaskScheduler taskScheduler() {
        return new ThreadPoolTaskScheduler() {
            @Override
            public ScheduledFuture<?> scheduleAtFixedRate(Runnable task, long period) {
                return null;
            }
            @Override
            public ScheduledFuture<?> scheduleAtFixedRate(Runnable task, Duration period) {
                return null;
            }
        };
    }
}
