package com.matt.portfolio;

public record GitHubCommitActivity(
    String repo,
    String message,
    String date,
    String hash
) {}
