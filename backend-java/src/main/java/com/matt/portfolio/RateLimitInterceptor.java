package com.matt.portfolio;

import io.github.bucket4j.Bandwidth;
import io.github.bucket4j.Bucket;
import jakarta.annotation.PostConstruct;
import jakarta.annotation.PreDestroy;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
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
import java.util.function.LongSupplier;

@Component
public class RateLimitInterceptor implements HandlerInterceptor {

    private static final Logger logger = LoggerFactory.getLogger(RateLimitInterceptor.class);
    private static final int DEFAULT_MAX_BUCKETS = 10_000;
    private static final long DEFAULT_EXPIRY_MS = 10 * 60 * 1000L; // 10 minutes

    private static class TimedBucket {
        final Bucket bucket;
        volatile long lastAccessTime;

        TimedBucket(Bucket bucket, long lastAccessTime) {
            this.bucket = bucket;
            this.lastAccessTime = lastAccessTime;
        }
    }

    private final int maxBuckets;
    private final long expiryMs;
    private final LongSupplier currentTimeMillis;
    private final Map<String, TimedBucket> buckets = new ConcurrentHashMap<>();
    private final Bucket overflowBucket = createNewBucket();
    private final ScheduledExecutorService cleanupScheduler = Executors.newSingleThreadScheduledExecutor();

    @Autowired
    public RateLimitInterceptor() {
        this(DEFAULT_MAX_BUCKETS, DEFAULT_EXPIRY_MS, System::currentTimeMillis);
    }

    RateLimitInterceptor(int maxBuckets, long expiryMs, LongSupplier currentTimeMillis) {
        this.maxBuckets = maxBuckets;
        this.expiryMs = expiryMs;
        this.currentTimeMillis = currentTimeMillis;
    }

    int bucketCount() {
        return buckets.size();
    }

    @PostConstruct
    public void startCleanup() {
        cleanupScheduler.scheduleAtFixedRate(() -> {
            long now = currentTimeMillis.getAsLong();
            buckets.entrySet().removeIf(entry -> now - entry.getValue().lastAccessTime > expiryMs);
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
        long now = currentTimeMillis.getAsLong();
        TimedBucket existing = buckets.get(clientIp);
        if (existing != null) {
            existing.lastAccessTime = now;
            return existing.bucket;
        }

        if (buckets.size() >= maxBuckets) {
            buckets.entrySet().removeIf(entry -> now - entry.getValue().lastAccessTime > expiryMs);
            if (buckets.size() >= maxBuckets) {
                return overflowBucket;
            }
        }

        TimedBucket tb = buckets.computeIfAbsent(clientIp, k -> {
            if (buckets.size() >= maxBuckets) {
                return null;
            }
            return new TimedBucket(createNewBucket(), now);
        });
        if (tb == null) {
            return overflowBucket;
        }
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
