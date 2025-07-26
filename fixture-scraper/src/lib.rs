//! # CalPal Fixture Scraper
//!
//! A sophisticated sports fixture scraping library with intelligent parsing and validation.
//! 
//! ## Architecture Overview
//! 
//! This library follows a multi-stage pipeline:
//! 1. **Scraping**: Team-specific scrapers extract raw HTML
//! 2. **Parsing**: Multi-stage parsing with graceful degradation and rich metadata
//! 3. **Validation**: Three-tier quality assessment for calendar integration
//! 
//! ## Key Design Principles
//! 
//! - **Time Independence**: All parsing uses injected time for deterministic tests
//! - **Rich Metadata**: Structured `ParseMetadata` instead of primitive strings
//! - **Graceful Degradation**: Multiple parsing strategies with fallbacks
//! - **Calendar First**: Validation designed for organizing friend watching parties
//! 
//! ## Example Usage
//! 
//! ```rust
//! use fixture_scraper::{arsenal::ArsenalScraper, FixtureScraper};
//! 
//! # tokio_test::block_on(async {
//! let scraper = ArsenalScraper::new();
//! let validated_fixtures = scraper.scrape().await?;
//! 
//! for validated_fixture in validated_fixtures {
//!     if validated_fixture.is_usable() {
//!         println!("{}", validated_fixture.get_calendar_description());
//!     }
//! }
//! # Ok::<(), fixture_scraper::ScrapeError>(())
//! # });
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod arsenal;
pub mod browser;
pub mod debug_browser;
pub mod parsing;
pub mod validation;

use parsing::ParseMetadata;

/// Core fixture representation with rich parsing metadata.
///
/// Fixtures are always stored with UTC timestamps internally and include
/// comprehensive parsing metadata for quality assessment and debugging.
///
/// ## Design Decisions
/// 
/// - **UTC First**: All datetime fields stored as UTC, converted to local time on display
/// - **Rich Metadata**: `ParseMetadata` provides structured information about parsing decisions
/// - **Validation Ready**: Designed to be wrapped in `ValidatedFixture` for quality assessment
/// 
/// ## Example
/// 
/// ```rust
/// use fixture_scraper::{Fixture, parsing::ParseMetadata, parsing::ParsingStrategy};
/// use chrono::{DateTime, Utc};
/// 
/// let metadata = ParseMetadata {
///     original_source: "Sun Jul 27 15:30".to_string(),
///     weekday_mismatch: None,
///     timezone_assumptions: "Parsed as Europe/London timezone".to_string(), 
///     parsing_strategy: ParsingStrategy::ExactMatch,
/// };
/// 
/// let fixture = Fixture::new(
///     "Arsenal".to_string(),
///     "Chelsea".to_string(),
///     Utc::now(),
///     "Emirates Stadium".to_string(),
///     "Premier League".to_string(),
///     metadata,
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fixture {
    /// Team name (e.g., "Arsenal", "Springboks")
    pub team: String,
    /// Opponent team name (may be "TBD" for unconfirmed fixtures)
    pub opponent: String,
    /// Fixture datetime in UTC (always UTC for consistency)
    pub datetime: DateTime<Utc>,
    /// Venue name and location
    pub venue: String,
    /// Competition name (e.g., "Premier League", "Rugby Championship")
    pub competition: String,
    /// Rich parsing metadata for quality assessment and debugging
    pub parse_metadata: ParseMetadata,
}

impl Fixture {
    pub fn new(
        team: String,
        opponent: String,
        datetime: DateTime<Utc>,
        venue: String,
        competition: String,
        parse_metadata: ParseMetadata,
    ) -> Self {
        Self {
            team,
            opponent,
            datetime,
            venue,
            competition,
            parse_metadata,
        }
    }

    /// Convert fixture time to London timezone (GMT/BST) for display.
    /// 
    /// This is the primary display method since Ollie is in London and 
    /// organizes watching parties in London time.
    pub fn to_london_time(&self) -> DateTime<chrono_tz::Tz> {
        use chrono_tz::Europe::London;
        self.datetime.with_timezone(&London)
    }
}

/// Comprehensive error types for the scraping pipeline.
/// 
/// Uses anyhow for error propagation but provides structured error types
/// for different failure modes in the scraping → parsing → validation pipeline.
#[derive(Debug)]
pub enum ScrapeError {
    /// Network-related errors (HTTP failures, timeouts, DNS issues)
    Network(String),
    /// General parsing errors (malformed HTML, unexpected structure)
    Parse(String), 
    /// Datetime parsing failures (invalid dates, timezone issues)
    InvalidDateTime(String),
    /// Missing HTML elements (selectors not found, empty pages)
    MissingElement(String),
}

impl fmt::Display for ScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScrapeError::Network(msg) => write!(f, "Network error: {}", msg),
            ScrapeError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ScrapeError::InvalidDateTime(msg) => write!(f, "Invalid datetime: {}", msg),
            ScrapeError::MissingElement(msg) => write!(f, "Missing element: {}", msg),
        }
    }
}

impl std::error::Error for ScrapeError {}

/// Core trait for all team-specific fixture scrapers.
/// 
/// This trait defines the contract that all team scrapers must implement.
/// The sophisticated architecture ensures that all scrapers benefit from:
/// - Shared parsing utilities with weekday tolerance
/// - Automatic validation with quality assessment  
/// - Rich metadata generation for calendar integration
/// 
/// ## Implementation Notes
/// 
/// - Always return `ValidatedFixture` (not raw `Fixture`)
/// - Use shared `DateTimeParser` for consistent behavior
/// - Leverage the three-tier validation system
/// - Handle timezone conversion properly (scrape in local time, store as UTC)
/// 
/// ## Example Implementation
/// 
/// ```rust
/// use fixture_scraper::{FixtureScraper, ScrapeError, validation::ValidatedFixture};
/// use fixture_scraper::parsing::DateTimeParser;
/// use chrono_tz::Europe::London;
/// 
/// pub struct MyTeamScraper {
///     parser: DateTimeParser,
/// }
/// 
/// #[async_trait::async_trait]
/// impl FixtureScraper for MyTeamScraper {
///     async fn scrape(&self) -> Result<Vec<ValidatedFixture>, ScrapeError> {
///         // 1. Fetch HTML
///         // 2. Extract fixture data  
///         // 3. Parse using shared DateTimeParser
///         // 4. Create Fixture with ParseMetadata
///         // 5. Wrap in ValidatedFixture
///         todo!()
///     }
///     
///     fn team_name(&self) -> &str { "My Team" }
///     fn source_url(&self) -> &str { "https://example.com/fixtures" }
/// }
/// ```
#[async_trait::async_trait]
pub trait FixtureScraper {
    /// Scrape fixtures and return validated results ready for calendar integration.
    /// 
    /// This method should:
    /// 1. Fetch HTML from the team's fixture page
    /// 2. Extract fixture data using CSS selectors 
    /// 3. Parse datetime strings using shared `DateTimeParser`
    /// 4. Create `Fixture` instances with rich `ParseMetadata`
    /// 5. Wrap each in `ValidatedFixture` for quality assessment
    async fn scrape(&self) -> Result<Vec<validation::ValidatedFixture>, ScrapeError>;
    
    /// Human-readable team name for display
    fn team_name(&self) -> &str;
    
    /// Source URL being scraped (for debugging and transparency)
    fn source_url(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike, Utc};
    use chrono_tz::Europe::London;

    struct MockScraper;

    #[async_trait::async_trait]
    impl FixtureScraper for MockScraper {
        async fn scrape(&self) -> Result<Vec<validation::ValidatedFixture>, ScrapeError> {
            let fixture = create_test_fixture();
            let validated = validation::ValidatedFixture::new(fixture);
            Ok(vec![validated])
        }

        fn team_name(&self) -> &str {
            "Arsenal"
        }

        fn source_url(&self) -> &str {
            "https://test.example.com"
        }
    }

    fn create_test_fixture() -> Fixture {
        let metadata = ParseMetadata {
            original_source: "Fri Aug 15 16:30".to_string(),
            weekday_mismatch: None,
            timezone_assumptions: "Parsed as UTC timezone".to_string(),
            parsing_strategy: parsing::ParsingStrategy::ExactMatch,
        };
        
        Fixture::new(
            "Arsenal".to_string(),
            "Chelsea".to_string(),
            Utc.with_ymd_and_hms(2025, 8, 15, 16, 30, 0).unwrap(),
            "Emirates Stadium".to_string(),
            "Premier League".to_string(),
            metadata,
        )
    }

    #[test]
    fn test_fixture_creation() {
        let fixture = create_test_fixture();
        
        assert_eq!(fixture.team, "Arsenal");
        assert_eq!(fixture.opponent, "Chelsea");
        assert_eq!(fixture.venue, "Emirates Stadium");
        assert_eq!(fixture.competition, "Premier League");
        assert_eq!(fixture.parse_metadata.original_source, "Fri Aug 15 16:30");
    }

    #[test]
    fn test_fixture_serialization() {
        let fixture = create_test_fixture();
        
        let json = serde_json::to_string(&fixture).expect("Should serialize to JSON");
        let deserialized: Fixture = serde_json::from_str(&json).expect("Should deserialize from JSON");
        
        assert_eq!(fixture, deserialized);
    }

    #[test]
    fn test_fixture_london_time_conversion() {
        let fixture = create_test_fixture();
        let london_time = fixture.to_london_time();
        
        // In August, London is BST (UTC+1)
        assert_eq!(london_time.timezone(), London);
        assert_eq!(london_time.hour(), 17); // 16:30 UTC becomes 17:30 BST
    }

    #[test]
    fn test_scrape_error_display() {
        let network_error = ScrapeError::Network("Connection failed".to_string());
        let parse_error = ScrapeError::Parse("HTML malformed".to_string());
        let datetime_error = ScrapeError::InvalidDateTime("Invalid format".to_string());
        let missing_error = ScrapeError::MissingElement("No fixture found".to_string());

        assert_eq!(network_error.to_string(), "Network error: Connection failed");
        assert_eq!(parse_error.to_string(), "Parse error: HTML malformed");
        assert_eq!(datetime_error.to_string(), "Invalid datetime: Invalid format");
        assert_eq!(missing_error.to_string(), "Missing element: No fixture found");
    }

    #[tokio::test]
    async fn test_fixture_scraper_trait() {
        let scraper = MockScraper;
        
        assert_eq!(scraper.team_name(), "Arsenal");
        assert_eq!(scraper.source_url(), "https://test.example.com");
        
        let validated_fixtures = scraper.scrape().await.expect("Should scrape successfully");
        assert_eq!(validated_fixtures.len(), 1);
        
        let fixture = &validated_fixtures[0].fixture;
        assert_eq!(fixture.team, "Arsenal");
        assert_eq!(fixture.opponent, "Chelsea");
        assert!(validated_fixtures[0].is_usable());
    }

    #[test]
    fn test_fixture_equality() {
        let fixture1 = create_test_fixture();
        let fixture2 = create_test_fixture();
        
        assert_eq!(fixture1, fixture2);
    }

    #[test]
    fn test_timezone_handling_winter() {
        // Test GMT/BST transition - January fixture
        let metadata = ParseMetadata {
            original_source: "Wed Jan 15 15:00".to_string(),
            weekday_mismatch: None,
            timezone_assumptions: "Parsed as UTC timezone".to_string(),
            parsing_strategy: parsing::ParsingStrategy::ExactMatch,
        };
        
        let winter_fixture = Fixture::new(
            "Arsenal".to_string(),
            "Liverpool".to_string(),
            Utc.with_ymd_and_hms(2025, 1, 15, 15, 0, 0).unwrap(),
            "Emirates Stadium".to_string(),
            "Premier League".to_string(),
            metadata,
        );
        
        let london_time = winter_fixture.to_london_time();
        // In January, London is GMT (UTC+0)
        assert_eq!(london_time.hour(), 15); // 15:00 UTC stays 15:00 GMT
    }
}