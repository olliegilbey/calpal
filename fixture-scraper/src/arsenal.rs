use crate::browser::BrowserScraper;
use crate::{validation::ValidatedFixture, Fixture, FixtureScraper, ScrapeError};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::Client;
use scraper::{Html, Selector};

pub struct ArsenalScraper {
    client: Client,
    browser: Option<BrowserScraper>,
    base_url: String,
    use_browser: bool,
}

impl ArsenalScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            browser: None,
            base_url: "https://www.arsenal.com/fixtures".to_string(),
            use_browser: true, // Default to browser for Arsenal's dynamic content
        }
    }

    /// Create scraper with headless browser support.
    /// Use this for production scraping of Arsenal's dynamic content.
    pub async fn with_browser() -> Result<Self, ScrapeError> {
        let browser = BrowserScraper::new().await?;
        Ok(Self {
            client: Client::new(),
            browser: Some(browser),
            base_url: "https://www.arsenal.com/fixtures".to_string(),
            use_browser: true,
        })
    }

    /// Create scraper without browser (for testing or simple HTTP scraping).
    pub fn without_browser() -> Self {
        Self {
            client: Client::new(),
            browser: None,
            base_url: "https://www.arsenal.com/fixtures".to_string(),
            use_browser: false,
        }
    }

}

#[async_trait]
impl FixtureScraper for ArsenalScraper {
    async fn scrape(&self) -> Result<Vec<ValidatedFixture>, ScrapeError> {
        // Get HTML content using browser if available, otherwise fall back to HTTP
        let html_content = if self.use_browser && self.browser.is_some() {
            // Use headless browser for dynamic content
            let browser = self.browser.as_ref().unwrap();
            browser.get_rendered_html(&self.base_url).await?
        } else {
            // Fall back to traditional HTTP scraping
            let response = self
                .client
                .get(&self.base_url)
                .header("User-Agent", "CalPal/1.0 (Sports Calendar Scraper)")
                .send()
                .await
                .map_err(|e| {
                    ScrapeError::Network(format!("Failed to fetch Arsenal fixtures: {e}"))
                })?;

            response
                .text()
                .await
                .map_err(|e| ScrapeError::Network(format!("Failed to read response body: {e}")))?
        };

        // Parse HTML
        let document = Html::parse_document(&html_content);

        // Define selectors for accordion fixtures (the real fixture data)
        let fixture_selector = Selector::parse("div.accordions article")
            .map_err(|e| ScrapeError::Parse(format!("Invalid fixture selector: {e}")))?;

        let datetime_selector = Selector::parse(".event-info__date time")
            .map_err(|e| ScrapeError::Parse(format!("Invalid datetime selector: {e}")))?;

        let venue_selector = Selector::parse(".event-info__venue")
            .map_err(|e| ScrapeError::Parse(format!("Invalid venue selector: {e}")))?;

        let competition_selector = Selector::parse(".event-info__extra")
            .map_err(|e| ScrapeError::Parse(format!("Invalid competition selector: {e}")))?;

        let opponent_selector = Selector::parse("h3.visually-hidden")
            .map_err(|e| ScrapeError::Parse(format!("Invalid opponent selector: {e}")))?;

        let mut fixtures = Vec::new();

        // Extract fixtures from HTML (now targeting accordion fixtures)
        for fixture_element in document.select(&fixture_selector) {
            // Extract datetime from the time element with datetime attribute
            let datetime_element = fixture_element.select(&datetime_selector).next();
            let datetime_str = datetime_element
                .and_then(|el| el.value().attr("datetime"))
                .unwrap_or("");

            let display_time = datetime_element
                .and_then(|el| el.text().next())
                .unwrap_or("Unknown Time");

            // Extract venue information
            let venue_text = fixture_element
                .select(&venue_selector)
                .next()
                .and_then(|el| el.text().next())
                .unwrap_or("TBD Venue")
                .to_string();

            // Extract competition
            let competition_text = fixture_element
                .select(&competition_selector)
                .next()
                .and_then(|el| el.text().next())
                .unwrap_or("Unknown Competition")
                .to_string();

            // Extract opponent from the visually-hidden header (e.g., "Villarreal - Wed Aug 6 - 18:00")
            let opponent = fixture_element
                .select(&opponent_selector)
                .next()
                .and_then(|el| el.text().next())
                .and_then(|text| text.split(" - ").next()) // Take first part before " - "
                .unwrap_or("TBD Opponent")
                .trim()
                .to_string();

            // Parse datetime from ISO format
            if !datetime_str.is_empty() {
                match chrono::DateTime::parse_from_rfc3339(datetime_str) {
                    Ok(parsed_datetime) => {
                        let utc_datetime = parsed_datetime.with_timezone(&Utc);

                        // Create ParseMetadata for the datetime
                        let metadata = crate::parsing::ParseMetadata {
                            original_source: format!("{display_time} ({datetime_str})"),
                            weekday_mismatch: None, // ISO format parsing is exact
                            timezone_assumptions: "Parsed from ISO datetime attribute".to_string(),
                            parsing_strategy: crate::parsing::ParsingStrategy::ExactMatch,
                        };

                        // Create fixture with proper venue and opponent
                        let fixture = Fixture::new(
                            "Arsenal".to_string(),
                            opponent,
                            utc_datetime,
                            venue_text,
                            competition_text,
                            metadata,
                        );

                        // Wrap in validation system
                        let validated_fixture = ValidatedFixture::new(fixture);
                        fixtures.push(validated_fixture);
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to parse ISO datetime '{datetime_str}': {e}"
                        );
                    }
                }
            } else {
                eprintln!("Warning: No datetime found for fixture");
            }
        }

        if fixtures.is_empty() {
            return Err(ScrapeError::MissingElement(
                "No fixtures found on Arsenal page".to_string(),
            ));
        }

        Ok(fixtures)
    }

    fn team_name(&self) -> &str {
        "Arsenal"
    }

    fn source_url(&self) -> &str {
        &self.base_url
    }
}

impl Default for ArsenalScraper {
    fn default() -> Self {
        // For tests and simple use cases, default to no browser
        Self::without_browser()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arsenal_scraper_creation() {
        let scraper = ArsenalScraper::new();
        assert_eq!(scraper.team_name(), "Arsenal");
        assert_eq!(scraper.source_url(), "https://www.arsenal.com/fixtures");
    }


    // Integration test - HTTP scraping (likely to fail with dynamic content)
    #[tokio::test]
    #[ignore] // Ignore by default, run with --ignored for actual scraping
    async fn test_scrape_arsenal_fixtures_http() {
        let scraper = ArsenalScraper::without_browser();
        let result = scraper.scrape().await;

        match result {
            Ok(fixtures) => {
                println!(
                    "HTTP scraping succeeded: {} Arsenal fixtures",
                    fixtures.len()
                );
                for fixture in fixtures.iter().take(3) {
                    println!(
                        "  {} vs {} at {} on {}",
                        fixture.fixture.team,
                        fixture.fixture.opponent,
                        fixture.fixture.venue,
                        fixture.fixture.to_london_time().format("%Y-%m-%d %H:%M")
                    );
                }
                assert!(!fixtures.is_empty());
            }
            Err(e) => {
                println!("HTTP scraping failed (expected for dynamic content): {e}");
                // This is expected to fail since Arsenal uses dynamic loading
            }
        }
    }

    // Integration test - Browser scraping (requires Chrome/Chromium)
    #[tokio::test]
    #[ignore] // Ignore by default, run with --ignored for actual scraping
    async fn test_scrape_arsenal_fixtures_browser() {
        let scraper = ArsenalScraper::with_browser().await;

        match scraper {
            Ok(scraper) => {
                let result = scraper.scrape().await;

                match result {
                    Ok(fixtures) => {
                        println!(
                            "Browser scraping succeeded: {} Arsenal fixtures",
                            fixtures.len()
                        );
                        for fixture in fixtures.iter().take(3) {
                            println!(
                                "  {} vs {} at {} on {}",
                                fixture.fixture.team,
                                fixture.fixture.opponent,
                                fixture.fixture.venue,
                                fixture.fixture.to_london_time().format("%Y-%m-%d %H:%M")
                            );
                        }
                        assert!(
                            !fixtures.is_empty(),
                            "Should find fixtures with browser rendering"
                        );
                    }
                    Err(e) => {
                        println!("Browser scraping failed: {e}");
                        panic!("Browser scraping should work for dynamic content");
                    }
                }
            }
            Err(e) => {
                println!(
                    "Browser initialization failed (Chrome/Chromium not available?): {e}"
                );
                // Skip test if browser not available
            }
        }
    }
}
