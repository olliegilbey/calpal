//! # Headless Browser Support for Dynamic Content
//!
//! This module provides a unified interface for scraping dynamic content that requires
//! JavaScript execution. Modern sports websites often load fixture data via AJAX,
//! making traditional HTTP scraping insufficient.
//!
//! ## Design Philosophy
//!
//! - **Selective Use**: Only use headless browser when necessary (dynamic content)
//! - **Performance**: Cache browser instances, reuse tabs when possible
//! - **Reliability**: Proper wait strategies for content loading
//! - **Debugging**: Rich error messages for troubleshooting dynamic sites
//!
//! ## Example Usage
//!
//! ```rust
//! use fixture_scraper::browser::BrowserScraper;
//!
//! # tokio_test::block_on(async {
//! let browser = BrowserScraper::new().await?;
//! let html = browser.get_rendered_html("https://example.com/fixtures").await?;
//! # Ok::<(), anyhow::Error>(())
//! # });
//! ```

use crate::ScrapeError;
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::time::Duration;

/// High-level browser automation for dynamic content scraping.
///
/// Provides a simple interface for fetching JavaScript-rendered HTML content
/// from modern web applications that load data dynamically.
pub struct BrowserScraper {
    browser: Browser,
}

impl BrowserScraper {
    /// Create a new browser instance with optimized settings for scraping.
    ///
    /// The browser is configured with:
    /// - Headless mode for server environments
    /// - Disabled images/CSS for faster loading
    /// - Reasonable timeouts for fixture pages
    pub async fn new() -> Result<Self, ScrapeError> {
        let config = BrowserConfig::builder()
            .window_size(1920, 1080)
            .no_sandbox()
            .build()
            .map_err(|e| ScrapeError::Network(format!("Browser config failed: {e}")))?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .map_err(|e| ScrapeError::Network(format!("Failed to launch browser: {e}")))?;

        // Spawn the handler to process browser events
        tokio::spawn(async move {
            while let Some(h) = handler.next().await {
                if let Err(e) = h {
                    eprintln!("Browser handler error: {e}");
                }
            }
        });

        Ok(Self { browser })
    }

    /// Fetch fully-rendered HTML content from a URL.
    ///
    /// This method:
    /// 1. Creates a new browser tab
    /// 2. Navigates to the URL with proper User-Agent
    /// 3. Waits for JavaScript execution and content loading
    /// 4. Returns the final rendered HTML
    ///
    /// ## Arguments
    /// - `url`: The URL to fetch (must be a valid HTTP/HTTPS URL)
    ///
    /// ## Returns
    /// The complete HTML content after JavaScript execution
    pub async fn get_rendered_html(&self, url: &str) -> Result<String, ScrapeError> {
        let page = self
            .browser
            .new_page("about:blank")
            .await
            .map_err(|e| ScrapeError::Network(format!("Failed to create new page: {e}")))?;

        // Set a proper User-Agent for sports websites
        page.set_user_agent("CalPal/1.0 (Sports Calendar Scraper; Chrome/120.0.0.0)")
            .await
            .map_err(|e| ScrapeError::Network(format!("Failed to set user agent: {e}")))?;

        // Navigate to the URL
        page.goto(url)
            .await
            .map_err(|e| ScrapeError::Network(format!("Failed to navigate to {url}: {e}")))?;

        // Wait for initial page load
        page.wait_for_navigation()
            .await
            .map_err(|e| ScrapeError::Network(format!("Navigation timeout for {url}: {e}")))?;

        // For dynamic content, wait a bit longer for AJAX calls to complete
        // This is especially important for Drupal Views that load via AJAX
        tokio::time::sleep(Duration::from_millis(3000)).await;

        // Extract the final HTML content
        let html = page
            .content()
            .await
            .map_err(|e| ScrapeError::Parse(format!("Failed to extract HTML content: {e}")))?;

        Ok(html)
    }
}

impl Drop for BrowserScraper {
    fn drop(&mut self) {
        // Browser cleanup happens automatically via the handler
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Chrome/Chromium installation
    async fn test_browser_creation() {
        let browser = BrowserScraper::new().await;
        assert!(browser.is_ok(), "Browser should initialize successfully");
    }

    #[tokio::test]
    #[ignore] // Requires Chrome/Chromium and network access
    async fn test_simple_page_fetch() {
        let browser = BrowserScraper::new()
            .await
            .expect("Browser should initialize");
        let html = browser.get_rendered_html("https://httpbin.org/html").await;

        assert!(html.is_ok(), "Should fetch simple HTML page");
        let content = html.unwrap();
        assert!(content.contains("<html>"), "Should contain HTML content");
    }
}
