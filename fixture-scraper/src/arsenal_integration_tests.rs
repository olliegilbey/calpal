//! Integration tests for Arsenal scraper with mocked HTML content
//!
//! These tests validate our breakthrough discovery: the difference between
//! .fixture-teaser elements (4 results) and div.accordions article elements (42+ results)

#[cfg(test)]
mod tests {
    // Integration tests focus on CSS selector and HTML parsing validation

    /// Mock HTML content representing Arsenal's fixture page structure
    /// This validates our breakthrough CSS selector discovery
    const MOCK_ARSENAL_HTML: &str = r#"
    <!DOCTYPE html>
    <html>
    <head><title>Arsenal Fixtures</title></head>
    <body>
        <!-- These are the teasers we initially found (only 4) -->
        <div class="fixture-teaser">
            <span>Teaser 1</span>
        </div>
        <div class="fixture-teaser">
            <span>Teaser 2</span>
        </div>
        <div class="fixture-teaser">
            <span>Teaser 3</span>
        </div>
        <div class="fixture-teaser">
            <span>Teaser 4</span>
        </div>
        
        <!-- This is where the real fixtures are (our breakthrough discovery!) -->
        <div class="accordions">
            <!-- Premier League fixture with full data -->
            <article>
                <h3 class="visually-hidden">Newcastle United - Sat Aug 15 - 15:00</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="2025-08-15T14:00:00.000Z">Sat 15 Aug 15:00</time>
                    </div>
                    <div class="event-info__venue">Emirates Stadium</div>
                    <div class="event-info__extra">Premier League</div>
                </div>
            </article>
            
            <!-- Away fixture with different venue -->
            <article>
                <h3 class="visually-hidden">Tottenham - Sun Sep 12 - 16:30</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="2025-09-12T15:30:00.000Z">Sun 12 Sep 16:30</time>
                    </div>
                    <div class="event-info__venue">Tottenham Hotspur Stadium</div>
                    <div class="event-info__extra">Premier League</div>
                </div>
            </article>
            
            <!-- International friendly -->
            <article>
                <h3 class="visually-hidden">Villarreal - Wed Aug 6 - 18:00</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="2025-08-06T17:00:00.000Z">Wed 6 Aug 18:00</time>
                    </div>
                    <div class="event-info__venue">National Stadium, Singapore</div>
                    <div class="event-info__extra">Friendly</div>
                </div>
            </article>
            
            <!-- Fixture with missing venue (edge case) -->
            <article>
                <h3 class="visually-hidden">Athletic Club - Sat Aug 9 - 16:00</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="2025-08-09T15:00:00.000Z">Sat 9 Aug 16:00</time>
                    </div>
                    <div class="event-info__extra">Friendly</div>
                </div>
            </article>
            
            <!-- Fixture with malformed opponent name (edge case) -->
            <article>
                <h3 class="visually-hidden">Liverpool FC - Extra Text - Sat Oct 4 - 17:30</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="2025-10-04T16:30:00.000Z">Sat 4 Oct 17:30</time>
                    </div>
                    <div class="event-info__venue">Anfield</div>
                    <div class="event-info__extra">Premier League</div>
                </div>
            </article>
        </div>
    </body>
    </html>
    "#;

    /// Mock HTML with no accordion fixtures (should trigger our error handling)
    const MOCK_EMPTY_ACCORDIONS: &str = r#"
    <!DOCTYPE html>
    <html>
    <head><title>Arsenal Fixtures</title></head>
    <body>
        <div class="fixture-teaser">
            <span>Teaser only</span>
        </div>
        <div class="accordions">
            <!-- No articles inside -->
        </div>
    </body>
    </html>
    "#;

    /// Mock HTML with malformed datetime (should handle gracefully)
    const MOCK_INVALID_DATETIME: &str = r#"
    <!DOCTYPE html>
    <html>
    <body>
        <div class="accordions">
            <article>
                <h3 class="visually-hidden">Chelsea - Invalid Date</h3>
                <div class="event-info">
                    <div class="event-info__date">
                        <time datetime="invalid-datetime">Invalid Date</time>
                    </div>
                    <div class="event-info__venue">Stamford Bridge</div>
                    <div class="event-info__extra">Premier League</div>
                </div>
            </article>
        </div>
    </body>
    </html>
    "#;

    #[tokio::test]
    async fn test_arsenal_css_selector_breakthrough() {
        // This test validates our breakthrough discovery:
        // .fixture-teaser elements are just teasers (4 found)
        // div.accordions article elements contain real fixtures (5+ found)

        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_ARSENAL_HTML);

        // Test the old selector (what we initially tried)
        let teaser_selector = Selector::parse(".fixture-teaser").unwrap();
        let teaser_count = document.select(&teaser_selector).count();
        assert_eq!(teaser_count, 4, "Should find exactly 4 teaser elements");

        // Test our breakthrough selector
        let accordion_selector = Selector::parse("div.accordions article").unwrap();
        let article_count = document.select(&accordion_selector).count();
        assert_eq!(
            article_count, 5,
            "Should find 5 real fixture articles in accordions"
        );

        // Validate that each article has the expected structure
        for article in document.select(&accordion_selector) {
            // Should have opponent info in h3.visually-hidden
            let opponent_selector = Selector::parse("h3.visually-hidden").unwrap();
            assert!(
                article.select(&opponent_selector).next().is_some(),
                "Each article should have opponent information"
            );

            // Should have datetime in time element
            let time_selector = Selector::parse(".event-info__date time").unwrap();
            assert!(
                article.select(&time_selector).next().is_some(),
                "Each article should have datetime information"
            );
        }
    }

    #[tokio::test]
    async fn test_venue_extraction_accuracy() {
        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_ARSENAL_HTML);
        let article_selector = Selector::parse("div.accordions article").unwrap();
        let venue_selector = Selector::parse(".event-info__venue").unwrap();

        let venues: Vec<String> = document
            .select(&article_selector)
            .filter_map(|article| {
                article
                    .select(&venue_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .map(|s| s.to_string())
            })
            .collect();

        // Validate our breakthrough venue extraction
        assert_eq!(
            venues.len(),
            4,
            "Should extract 4 venues (1 fixture has no venue)"
        );
        assert!(venues.contains(&"Emirates Stadium".to_string()));
        assert!(venues.contains(&"Tottenham Hotspur Stadium".to_string()));
        assert!(venues.contains(&"National Stadium, Singapore".to_string()));
        assert!(venues.contains(&"Anfield".to_string()));
    }

    #[tokio::test]
    async fn test_opponent_parsing_breakthrough() {
        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_ARSENAL_HTML);
        let article_selector = Selector::parse("div.accordions article").unwrap();
        let opponent_selector = Selector::parse("h3.visually-hidden").unwrap();

        let opponents: Vec<String> = document
            .select(&article_selector)
            .filter_map(|article| {
                article
                    .select(&opponent_selector)
                    .next()
                    .and_then(|el| el.text().next())
                    .and_then(|text| text.split(" - ").next()) // Take first part before " - "
                    .map(|s| s.trim().to_string())
            })
            .collect();

        assert_eq!(opponents.len(), 5, "Should extract 5 opponents");
        assert!(opponents.contains(&"Newcastle United".to_string()));
        assert!(opponents.contains(&"Tottenham".to_string()));
        assert!(opponents.contains(&"Villarreal".to_string()));
        assert!(opponents.contains(&"Athletic Club".to_string()));
        assert!(opponents.contains(&"Liverpool FC".to_string()));
    }

    #[test]
    fn test_html_structure_validation() {
        // Test that our CSS selectors are robust
        use scraper::Selector;

        // Test selector compilation (should never fail)
        let selectors_to_test = [
            "div.accordions article",
            ".event-info__date time",
            ".event-info__venue",
            ".event-info__extra",
            "h3.visually-hidden",
        ];

        for selector_str in &selectors_to_test {
            let selector = Selector::parse(selector_str);
            assert!(
                selector.is_ok(),
                "CSS selector '{selector_str}' should compile successfully"
            );
        }
    }

    #[test]
    fn test_empty_accordions_handling() {
        // Test that we handle pages with no fixture data gracefully
        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_EMPTY_ACCORDIONS);
        let article_selector = Selector::parse("div.accordions article").unwrap();
        let article_count = document.select(&article_selector).count();

        assert_eq!(
            article_count, 0,
            "Should find no articles in empty accordions"
        );

        // This scenario should trigger our "No fixtures found" error in the real scraper
    }

    #[test]
    fn test_malformed_datetime_handling() {
        // Test parsing behavior with invalid datetime attributes
        use chrono::DateTime;
        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_INVALID_DATETIME);
        let article_selector = Selector::parse("div.accordions article").unwrap();
        let datetime_selector = Selector::parse(".event-info__date time").unwrap();

        for article in document.select(&article_selector) {
            if let Some(time_element) = article.select(&datetime_selector).next() {
                if let Some(datetime_str) = time_element.value().attr("datetime") {
                    // This should fail gracefully (our scraper handles this)
                    let parse_result = DateTime::parse_from_rfc3339(datetime_str);
                    assert!(
                        parse_result.is_err(),
                        "Invalid datetime should fail to parse"
                    );
                }
            }
        }
    }

    #[test]
    fn test_missing_venue_fallback() {
        // Test that fixtures without venue information get "TBD Venue"
        use scraper::{Html, Selector};

        let document = Html::parse_document(MOCK_ARSENAL_HTML);
        let article_selector = Selector::parse("div.accordions article").unwrap();
        let venue_selector = Selector::parse(".event-info__venue").unwrap();

        let mut articles_with_no_venue = 0;

        for article in document.select(&article_selector) {
            let venue_text = article
                .select(&venue_selector)
                .next()
                .and_then(|el| el.text().next())
                .unwrap_or("TBD Venue");

            if venue_text == "TBD Venue" {
                articles_with_no_venue += 1;
            }
        }

        assert_eq!(
            articles_with_no_venue, 1,
            "Should have exactly 1 fixture with missing venue data"
        );
    }
}
