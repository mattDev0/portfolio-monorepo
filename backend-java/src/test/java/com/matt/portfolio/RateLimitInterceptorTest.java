package com.matt.portfolio;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.springframework.mock.web.MockHttpServletRequest;
import org.springframework.mock.web.MockHttpServletResponse;

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
}
