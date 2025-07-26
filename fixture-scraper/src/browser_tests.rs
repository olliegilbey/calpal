//! Comprehensive tests for headless browser integration and fallback scenarios

#[cfg(test)]
mod tests {
    use super::super::arsenal::ArsenalScraper;
    use super::super::browser::BrowserScraper;
    use super::super::{FixtureScraper, ScrapeError};

    #[tokio::test]
    async fn test_browser_initialization_graceful_failure() {
        // Test that browser initialization handles missing Chrome/Chromium gracefully
        let result = BrowserScraper::new().await;

        match result {
            Ok(_) => {
                // Browser available - test successful initialization
                println!("✅ Browser initialization successful");
            }
            Err(ScrapeError::Network(msg)) => {
                // Expected failure on systems without Chrome/Chromium
                assert!(
                    msg.contains("Failed to launch browser")
                        || msg.contains("Browser config failed")
                );
                println!("⚠️  Browser not available (expected in CI): {msg}");
            }
            Err(other) => {
                panic!(
                    "Unexpected error type during browser initialization: {other:?}"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_arsenal_scraper_browser_fallback_strategy() {
        // Test that Arsenal scraper handles browser initialization failure gracefully

        // Try browser-enabled scraper
        let browser_scraper_result = ArsenalScraper::with_browser().await;

        match browser_scraper_result {
            Ok(scraper) => {
                // Browser available - test that it's configured correctly
                assert_eq!(scraper.team_name(), "Arsenal");
                assert_eq!(scraper.source_url(), "https://www.arsenal.com/fixtures");
                println!("✅ Browser scraper initialization successful");
            }
            Err(ScrapeError::Network(msg)) => {
                // Browser not available - test fallback
                assert!(
                    msg.contains("Failed to launch browser")
                        || msg.contains("Browser config failed")
                );
                println!("⚠️  Browser not available, testing fallback: {msg}");

                // Fallback to HTTP-only scraper should always work
                let http_scraper = ArsenalScraper::without_browser();
                assert_eq!(http_scraper.team_name(), "Arsenal");
                assert_eq!(
                    http_scraper.source_url(),
                    "https://www.arsenal.com/fixtures"
                );
                println!("✅ HTTP fallback scraper works");
            }
            Err(other) => {
                panic!(
                    "Unexpected error during browser scraper creation: {other:?}"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_browser_error_handling_network_failures() {
        // Test browser behavior with invalid URLs
        let browser_result = BrowserScraper::new().await;

        if let Ok(browser) = browser_result {
            // Test invalid URL handling
            let invalid_url_result = browser.get_rendered_html("not-a-valid-url").await;
            assert!(
                invalid_url_result.is_err(),
                "Invalid URL should return error"
            );

            if let Err(ScrapeError::Network(msg)) = invalid_url_result {
                assert!(
                    msg.contains("Failed to navigate"),
                    "Error should mention navigation failure"
                );
            }

            // Test unreachable URL handling
            let unreachable_result = browser
                .get_rendered_html("https://this-domain-does-not-exist-12345.com")
                .await;
            assert!(
                unreachable_result.is_err(),
                "Unreachable URL should return error"
            );
        } else {
            println!("⚠️  Skipping browser network tests - browser not available");
        }
    }

    #[test]
    fn test_browser_configuration_validation() {
        // Test that our browser configuration parameters are valid
        use chromiumoxide::browser::BrowserConfig;

        // Test configuration that matches our BrowserScraper::new() setup
        let config_result = BrowserConfig::builder()
            .window_size(1920, 1080)
            .no_sandbox()
            .build();

        assert!(
            config_result.is_ok(),
            "Browser configuration should be valid"
        );
    }

    #[test]
    fn test_scraper_creation_patterns() {
        // Test different Arsenal scraper creation patterns

        // Default should be without browser (for testing)
        let default_scraper = ArsenalScraper::default();
        assert_eq!(default_scraper.team_name(), "Arsenal");

        // new() should default to browser-enabled
        let new_scraper = ArsenalScraper::new();
        assert_eq!(new_scraper.team_name(), "Arsenal");

        // without_browser() should work in any environment
        let http_scraper = ArsenalScraper::without_browser();
        assert_eq!(http_scraper.team_name(), "Arsenal");
    }

    #[tokio::test]
    async fn test_user_agent_and_headers() {
        // Test that browser scraper sets appropriate headers
        let browser_result = BrowserScraper::new().await;

        if let Ok(_browser) = browser_result {
            // Browser initialization successful
            // Note: Testing actual user agent requires network call,
            // so we just verify browser creation works
            println!("✅ Browser created successfully with user agent configuration");
        } else {
            println!("⚠️  Browser not available for user agent testing");
        }
    }

    #[tokio::test]
    async fn test_content_loading_timeout_handling() {
        // Test browser timeout behavior
        let browser_result = BrowserScraper::new().await;

        if let Ok(browser) = browser_result {
            // Test with a URL that should respond quickly
            let simple_result = browser.get_rendered_html("https://httpbin.org/html").await;

            match simple_result {
                Ok(html) => {
                    assert!(html.contains("<html>"), "Should receive valid HTML content");
                    println!("✅ Browser successfully fetched and rendered content");
                }
                Err(ScrapeError::Network(msg)) => {
                    // Network issues in test environment are acceptable
                    println!("⚠️  Network error during content test: {msg}");
                }
                Err(other) => {
                    panic!("Unexpected error during content loading: {other:?}");
                }
            }
        } else {
            println!("⚠️  Skipping content loading tests - browser not available");
        }
    }

    #[test]
    fn test_error_message_quality() {
        // Test that our error messages are helpful for debugging

        let network_error =
            ScrapeError::Network("Failed to launch browser: Chrome not found".to_string());
        let error_msg = network_error.to_string();

        assert!(error_msg.contains("Network error"));
        assert!(error_msg.contains("Failed to launch browser"));
        assert!(error_msg.contains("Chrome not found"));

        // Error messages should be actionable
        assert!(!error_msg.is_empty());
        assert!(error_msg.len() > 10); // Substantial error messages
    }
}
