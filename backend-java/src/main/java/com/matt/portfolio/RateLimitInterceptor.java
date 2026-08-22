package com.matt.portfolio;

import io.github.bucket4j.Bandwidth;
import io.github.bucket4j.Bucket;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.stereotype.Component;
import org.springframework.web.servlet.HandlerInterceptor;

import java.time.Duration;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

@Component
public class RateLimitInterceptor implements HandlerInterceptor {

    private static final Logger logger = LoggerFactory.getLogger(RateLimitInterceptor.class);
    private static final int MAX_BUCKETS = 10_000;
    private static final long EXPIRY_MS = 10 * 60 * 1000L; // 10 minutes

    private static class TimedBucket {
        final Bucket bucket;
        volatile long lastAccessTime;

        TimedBucket(Bucket bucket, long lastAccessTime) {
            this.bucket = bucket;
            this.lastAccessTime = lastAccessTime;
        }
    }

    private final Map<String, TimedBucket> buckets = new ConcurrentHashMap<>();
    private final ScheduledExecutorService cleanupScheduler = Executors.newSingleThreadScheduledExecutor();

    @PostConstruct
    public void startCleanup() {
        cleanupScheduler.scheduleAtFixedRate(() -> {
            long now = System.currentTimeMillis();
            buckets.entrySet().removeIf(entry -> now - entry.getValue().lastAccessTime > EXPIRY_MS);
        }, 5, 5, TimeUnit.MINUTES);
    }

    @PreDestroy
    public void stopCleanup() {
        cleanupScheduler.shutdown();
    }

    private Bucket createNewBucket() {
        return Bucket.builder()
                .addLimit(Bandwidth.simple(60, Duration.ofMinutes(1)))
                .build();
    }

    private Bucket resolveBucket(String clientIp) {
        long now = System.currentTimeMillis();
        if (buckets.size() >= MAX_BUCKETS && !buckets.containsKey(clientIp)) {
            buckets.entrySet().removeIf(entry -> now - entry.getValue().lastAccessTime > EXPIRY_MS);
        }
        TimedBucket tb = buckets.computeIfAbsent(clientIp, k -> new TimedBucket(createNewBucket(), now));
        tb.lastAccessTime = now;
        return tb.bucket;
    }

    String resolveClientIp(HttpServletRequest request) {
        String xff = request.getHeader("X-Forwarded-For");
        if (xff == null || xff.isBlank()) {
            return request.getRemoteAddr();
        }

        List<String> ips = Arrays.stream(xff.split(","))
                .map(String::trim)
                .filter(s -> !s.isEmpty())
                .toList();

        if (ips.size() >= 2) {
            // On Cloud Run, the last entry is Google's front end and second-from-last is the client IP
            return ips.get(ips.size() - 2);
        } else if (ips.size() == 1) {
            return ips.get(0);
        }

        return request.getRemoteAddr();
    }

    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
        String clientIp = resolveClientIp(request);

        Bucket bucket = resolveBucket(clientIp);
        if (bucket.tryConsume(1)) {
            return true;
        }

        logger.warn("Rate limit exceeded for IP: {}", clientIp);
        response.setStatus(HttpStatus.TOO_MANY_REQUESTS.value());
        response.setContentType("application/json");
        response.getWriter().write("{\"error\": \"Too many requests. Please try again later.\"}");
        return false;
    }
}
