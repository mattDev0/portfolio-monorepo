package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.mock.web.MockHttpServletRequest;
import org.springframework.mock.web.MockHttpServletResponse;

import java.util.concurrent.atomic.AtomicLong;

import static org.junit.jupiter.api.Assertions.*;

public class RateLimitInterceptorTest {

    private RateLimitInterceptor interceptor;

    @BeforeEach
    public void setUp() {
        interceptor = new RateLimitInterceptor();
    }

    @Test
    public void testResolveClientIpMissingHeader() {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("192.168.1.50");

        String ip = interceptor.resolveClientIp(request);
        assertEquals("192.168.1.50", ip);
    }

    @Test
    public void testResolveClientIpEmptyHeader() {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("192.168.1.50");
        request.addHeader("X-Forwarded-For", "   ");

        String ip = interceptor.resolveClientIp(request);
        assertEquals("192.168.1.50", ip);
    }

    @Test
    public void testResolveClientIpSingleEntry() {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("10.0.0.1");
        request.addHeader("X-Forwarded-For", "203.0.113.195");

        String ip = interceptor.resolveClientIp(request);
        assertEquals("203.0.113.195", ip);
    }

    @Test
    public void testResolveClientIpMultiEntryCloudRun() {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("169.254.1.1");
        // Format: client_ip, google_frontend_ip
        request.addHeader("X-Forwarded-For", "198.51.100.42, 130.211.0.1");

        String ip = interceptor.resolveClientIp(request);
        assertEquals("198.51.100.42", ip);
    }

    @Test
    public void testResolveClientIpSpoofedLeadingEntryIgnored() {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("169.254.1.1");
        // Format: spoofed_ip, real_client_ip, google_frontend_ip
        request.addHeader("X-Forwarded-For", "1.2.3.4, 198.51.100.42, 130.211.0.1");

        String ip = interceptor.resolveClientIp(request);
        assertEquals("198.51.100.42", ip);
    }

    @Test
    public void testPreHandleRateLimiting() throws Exception {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRemoteAddr("10.10.10.10");
        MockHttpServletResponse response = new MockHttpServletResponse();

        // 60 requests should pass
        for (int i = 0; i < 60; i++) {
            boolean allowed = interceptor.preHandle(request, new MockHttpServletResponse(), new Object());
            assertTrue(allowed, "Request " + (i + 1) + " should be allowed");
        }

        // 61st request should be rejected with 429
        boolean allowed = interceptor.preHandle(request, response, new Object());
        assertFalse(allowed);
        assertEquals(429, response.getStatus());
        assertTrue(response.getContentAsString().contains("Too many requests"));
    }

    @Test
    public void testBucketMapCapUsesSharedOverflowBucketWithoutGrowing() throws Exception {
        AtomicLong clock = new AtomicLong(1_000_000L);
        RateLimitInterceptor capped = new RateLimitInterceptor(2, 600_000L, clock::get);

        // Fill the map up to the cap with two IPs
        for (String ip : new String[]{"10.0.0.1", "10.0.0.2"}) {
            MockHttpServletRequest req = new MockHttpServletRequest();
            req.setRemoteAddr(ip);
            assertTrue(capped.preHandle(req, new MockHttpServletResponse(), new Object()));
        }
        assertEquals(2, capped.bucketCount());

        // New IPs beyond the cap must not grow the map and share the 60/min overflow bucket
        for (int i = 0; i < 30; i++) {
            MockHttpServletRequest reqA = new MockHttpServletRequest();
            reqA.setRemoteAddr("10.0.1." + i);
            assertTrue(capped.preHandle(reqA, new MockHttpServletResponse(), new Object()));

            MockHttpServletRequest reqB = new MockHttpServletRequest();
            reqB.setRemoteAddr("10.0.2." + i);
            assertTrue(capped.preHandle(reqB, new MockHttpServletResponse(), new Object()));
        }
        assertEquals(2, capped.bucketCount());

        // 61st request across any new overflow IP hits the shared overflow bucket limit
        MockHttpServletRequest overflowReq = new MockHttpServletRequest();
        overflowReq.setRemoteAddr("10.0.3.1");
        MockHttpServletResponse overflowRes = new MockHttpServletResponse();
        assertFalse(capped.preHandle(overflowReq, overflowRes, new Object()));
        assertEquals(429, overflowRes.getStatus());
        assertEquals(2, capped.bucketCount());

        // Existing IP still retains its own bucket and is unaffected by overflow exhaustion
        MockHttpServletRequest existingReq = new MockHttpServletRequest();
        existingReq.setRemoteAddr("10.0.0.1");
        assertTrue(capped.preHandle(existingReq, new MockHttpServletResponse(), new Object()));
        assertEquals(2, capped.bucketCount());
    }

    @Test
    public void testExpiredBucketsEvictedAndSlotsReused() throws Exception {
        AtomicLong clock = new AtomicLong(1_000_000L);
        RateLimitInterceptor capped = new RateLimitInterceptor(2, 600_000L, clock::get);

        MockHttpServletRequest req1 = new MockHttpServletRequest();
        req1.setRemoteAddr("10.0.0.1");
        assertTrue(capped.preHandle(req1, new MockHttpServletResponse(), new Object()));

        // Advance past half-expiry and touch IP 2 so only IP 1 expires on the next check
        clock.addAndGet(400_000L);
        MockHttpServletRequest req2 = new MockHttpServletRequest();
        req2.setRemoteAddr("10.0.0.2");
        assertTrue(capped.preHandle(req2, new MockHttpServletResponse(), new Object()));
        assertEquals(2, capped.bucketCount());

        // Advance so IP 1 is older than 600_000ms (elapsed 700_000ms), while IP 2 is still fresh (elapsed 300_000ms)
        clock.addAndGet(300_000L);
        MockHttpServletRequest req3 = new MockHttpServletRequest();
        req3.setRemoteAddr("10.0.0.3");
        assertTrue(capped.preHandle(req3, new MockHttpServletResponse(), new Object()));
        assertEquals(2, capped.bucketCount());

        // Exhaust the shared overflow bucket from a 4th IP to prove 10.0.0.3 got a real slot
        // and wasn't sharing the overflow bucket with 10.0.0.4
        for (int i = 0; i < 60; i++) {
            MockHttpServletRequest req4 = new MockHttpServletRequest();
            req4.setRemoteAddr("10.0.0.4");
            assertTrue(capped.preHandle(req4, new MockHttpServletResponse(), new Object()));
        }
        MockHttpServletRequest req5 = new MockHttpServletRequest();
        req5.setRemoteAddr("10.0.0.5");
        MockHttpServletResponse res5 = new MockHttpServletResponse();
        assertFalse(capped.preHandle(req5, res5, new Object()));
        assertEquals(429, res5.getStatus());

        // 10.0.0.3 still succeeds because it reused the evicted slot
        assertTrue(capped.preHandle(req3, new MockHttpServletResponse(), new Object()));
        assertEquals(2, capped.bucketCount());
    }
}
