//! # Three-Tier Fixture Validation System
//!
//! This module implements intelligent quality assessment for sports fixtures,
//! designed specifically for calendar integration and organizing watching parties.
//!
//! ## Validation Philosophy
//!
//! Not all scraped data is equal. This system classifies fixtures into:
//!
//! - **Valid**: Perfect quality, ready for calendar
//! - **ValidWithWarnings**: Usable with quality notes (weekday mismatches, unusual times)
//! - **Invalid**: Critical problems, should not be used 
//! - **Historical**: Past fixtures, filtered out for planning
//!
//! ## Calendar Integration Focus
//!
//! The validation system is designed around Ollie's goal of organizing friend watching parties:
//! - Date range limits (current year to 2 years future)
//! - Quality warnings become calendar event descriptions
//! - Rich metadata from parsing feeds into validation decisions
//! - London timezone focus for display

use crate::Fixture;
use crate::parsing::{ParseMetadata, ParsingStrategy};
use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidatedFixture {
    pub fixture: Fixture,
    pub validation: FixtureValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FixtureValidation {
    Valid,
    ValidWithWarnings(Vec<ValidationIssue>),
    Invalid(Vec<ValidationIssue>),
    Historical(DateTime<Utc>), // When the fixture was scraped
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub message: String,
    pub suggested_fix: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueSeverity {
    Warning,  // Keep fixture, but note the issue
    Error,    // Fixture is problematic but might be usable
    Critical, // Fixture should not be used
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IssueCategory {
    DateWeekdayMismatch,
    HistoricalFixture,
    SuspiciousTime,
    MissingData,
    DataInconsistency,
}

impl ValidatedFixture {
    pub fn new(fixture: Fixture) -> Self {
        let validation = FixtureValidator::validate(&fixture);
        Self { fixture, validation }
    }

    pub fn is_usable(&self) -> bool {
        !matches!(self.validation, 
            FixtureValidation::Invalid(_) | FixtureValidation::Historical(_)
        )
    }

    pub fn get_calendar_description(&self) -> String {
        let mut description = format!(
            "{} vs {} at {}\nCompetition: {}",
            self.fixture.team, self.fixture.opponent, 
            self.fixture.venue, self.fixture.competition
        );

        match &self.validation {
            FixtureValidation::Valid => {},
            FixtureValidation::ValidWithWarnings(issues) => {
                description.push_str("\n\n⚠️ Data Quality Notes:");
                for issue in issues {
                    description.push_str(&format!("\n• {}", issue.message));
                }
            },
            FixtureValidation::Invalid(issues) => {
                description.push_str("\n\n❌ Data Issues Detected:");
                for issue in issues {
                    description.push_str(&format!("\n• {}", issue.message));
                }
            },
            FixtureValidation::Historical(scraped_at) => {
                description.push_str(&format!(
                    "\n\n📅 Historical fixture (scraped {})", 
                    scraped_at.format("%Y-%m-%d")
                ));
            }
        }

        description
    }
}

pub struct FixtureValidator;

impl FixtureValidator {
    pub fn validate(fixture: &Fixture) -> FixtureValidation {
        let mut issues = Vec::new();

        // Check if fixture is historical
        if fixture.datetime < Utc::now() {
            return FixtureValidation::Historical(Utc::now());
        }

        // Check for reasonable date range (current year start to 2 years future)
        if let Some(issue) = Self::validate_date_range(fixture) {
            issues.push(issue);
        }

        // Validate date-weekday consistency (the sophisticated part!)
        if let Some(issue) = Self::validate_date_consistency(fixture) {
            issues.push(issue);
        }

        // Check for suspicious times
        if let Some(issue) = Self::validate_fixture_time(fixture) {
            issues.push(issue);
        }

        // Check for missing/suspicious data
        issues.extend(Self::validate_fixture_data(fixture));

        // Categorize based on issue severity
        let has_critical = issues.iter().any(|i| i.severity == IssueSeverity::Critical);
        let has_errors = issues.iter().any(|i| i.severity == IssueSeverity::Error);

        if has_critical {
            FixtureValidation::Invalid(issues)
        } else if has_errors || !issues.is_empty() {
            FixtureValidation::ValidWithWarnings(issues)
        } else {
            FixtureValidation::Valid
        }
    }

    fn validate_date_consistency(fixture: &Fixture) -> Option<ValidationIssue> {
        let london_time = fixture.to_london_time();
        let date = london_time.date_naive();
        let actual_weekday = date.weekday();

        // Try to extract weekday from timezone_info or other sources
        // This is where we'd parse "London Time (GMT/BST) - originally Sun Jan 15"
        if let Some(expected_weekday) = Self::extract_expected_weekday(fixture) {
            if actual_weekday != expected_weekday {
                return Some(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    category: IssueCategory::DateWeekdayMismatch,
                    message: format!(
                        "Date {} is a {}, but source indicated {}. Source may have incorrect weekday.",
                        date.format("%b %d, %Y"),
                        Self::weekday_to_string(actual_weekday),
                        Self::weekday_to_string(expected_weekday)
                    ),
                    suggested_fix: Some(format!(
                        "Verify fixture date. If date is correct, ignore weekday discrepancy."
                    )),
                });
            }
        }

        None
    }

    fn validate_date_range(fixture: &Fixture) -> Option<ValidationIssue> {
        let current_year = Utc::now().year();
        let fixture_year = fixture.datetime.year();
        
        // Accept fixtures from current year start to 2 years in future
        let min_year = current_year;
        let max_year = current_year + 2;
        
        if fixture_year < min_year || fixture_year > max_year {
            Some(ValidationIssue {
                severity: IssueSeverity::Critical, // Critical = thrown out entirely
                category: IssueCategory::DataInconsistency,
                message: format!(
                    "Fixture date {} is outside reasonable range ({}-{}). Fixtures only planned {} years ahead.",
                    fixture.datetime.format("%Y-%m-%d"),
                    min_year,
                    max_year,
                    max_year - min_year
                ),
                suggested_fix: Some("Check date parsing and source data accuracy".to_string()),
            })
        } else {
            None
        }
    }

    fn validate_fixture_time(fixture: &Fixture) -> Option<ValidationIssue> {
        let london_time = fixture.to_london_time();
        let hour = london_time.hour();

        // Flag unusual fixture times
        if hour < 8 || hour > 23 {
            Some(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::SuspiciousTime,
                message: format!(
                    "Unusual fixture time: {}:{:02} London time",
                    hour, london_time.minute()
                ),
                suggested_fix: Some("Verify time zone conversion is correct".to_string()),
            })
        } else {
            None
        }
    }

    fn validate_fixture_data(fixture: &Fixture) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check for placeholder/missing data
        if fixture.opponent.contains("TBD") || fixture.opponent.contains("Unknown") {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                category: IssueCategory::MissingData,
                message: "Opponent not yet determined".to_string(),
                suggested_fix: Some("Check source closer to fixture date".to_string()),
            });
        }

        if fixture.venue.contains("Unknown") || fixture.venue.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                category: IssueCategory::MissingData,
                message: "Venue information missing".to_string(),
                suggested_fix: None,
            });
        }

        issues
    }

    fn extract_expected_weekday(fixture: &Fixture) -> Option<Weekday> {
        // Use rich ParseMetadata instead of primitive string parsing
        if let Some(weekday_mismatch) = &fixture.parse_metadata.weekday_mismatch {
            // Extract weekday from the claimed weekday in the mismatch data
            let claimed = weekday_mismatch.claimed_weekday.to_lowercase();
            if claimed.contains("sun") || claimed.starts_with("sun") { Some(Weekday::Sun) }
            else if claimed.contains("mon") || claimed.starts_with("mon") { Some(Weekday::Mon) }
            else if claimed.contains("tue") || claimed.starts_with("tue") { Some(Weekday::Tue) }
            else if claimed.contains("wed") || claimed.starts_with("wed") { Some(Weekday::Wed) }
            else if claimed.contains("thu") || claimed.starts_with("thu") { Some(Weekday::Thu) }
            else if claimed.contains("fri") || claimed.starts_with("fri") { Some(Weekday::Fri) }
            else if claimed.contains("sat") || claimed.starts_with("sat") { Some(Weekday::Sat) }
            else { None }
        } else {
            // No weekday mismatch recorded - parsing was exact
            None
        }
    }

    fn weekday_to_string(weekday: Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday", 
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        }
    }
}

impl fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Error => write!(f, "ERROR"),
            IssueSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl fmt::Display for IssueCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IssueCategory::DateWeekdayMismatch => write!(f, "Date/Weekday Mismatch"),
            IssueCategory::HistoricalFixture => write!(f, "Historical Fixture"),
            IssueCategory::SuspiciousTime => write!(f, "Suspicious Time"),
            IssueCategory::MissingData => write!(f, "Missing Data"),
            IssueCategory::DataInconsistency => write!(f, "Data Inconsistency"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn create_test_fixture() -> Fixture {
        // Create mock ParseMetadata for testing
        let metadata = ParseMetadata {
            original_source: "Fri Aug 15 16:30".to_string(),
            weekday_mismatch: None, // No mismatch for valid test fixture
            timezone_assumptions: "Parsed as Europe/London timezone".to_string(),
            parsing_strategy: ParsingStrategy::ExactMatch,
        };
        
        Fixture::new(
            "Arsenal".to_string(),
            "Chelsea".to_string(),
            Utc.with_ymd_and_hms(2025, 8, 15, 16, 30, 0).unwrap(), // Friday
            "Emirates Stadium".to_string(),
            "Premier League".to_string(),
            metadata,
        )
    }

    #[test]
    fn test_valid_fixture() {
        let fixture = create_test_fixture();
        let validated = ValidatedFixture::new(fixture);
        
        assert!(validated.is_usable());
        assert!(matches!(validated.validation, FixtureValidation::Valid));
    }

    #[test]
    fn test_historical_fixture() {
        let mut fixture = create_test_fixture();
        fixture.datetime = Utc.with_ymd_and_hms(2020, 1, 1, 15, 0, 0).unwrap();
        
        let validated = ValidatedFixture::new(fixture);
        assert!(!validated.is_usable()); // Historical fixtures are not usable
        assert!(matches!(validated.validation, FixtureValidation::Historical(_)));
    }

    #[test]
    fn test_weekday_mismatch_detection() {
        let mut fixture = create_test_fixture();
        // Set parse_metadata to indicate weekday mismatch (Aug 15, 2025 is actually Friday, not Sunday)
        fixture.parse_metadata.weekday_mismatch = Some(crate::parsing::WeekdayMismatch {
            claimed_weekday: "Sun".to_string(),
            actual_weekday: "Friday".to_string(),
            date: "Aug 15".to_string(),
        });
        fixture.parse_metadata.parsing_strategy = ParsingStrategy::WeekdayTolerant;
        
        let validated = ValidatedFixture::new(fixture);
        assert!(validated.is_usable()); // Still usable, just warned
        
        if let FixtureValidation::ValidWithWarnings(issues) = &validated.validation {
            assert!(issues.iter().any(|i| i.category == IssueCategory::DateWeekdayMismatch));
        } else {
            panic!("Expected validation warnings");
        }
    }

    #[test]
    fn test_suspicious_time() {
        let mut fixture = create_test_fixture();
        fixture.datetime = Utc.with_ymd_and_hms(2025, 8, 15, 3, 0, 0).unwrap(); // 3 AM UTC = 4 AM BST
        
        let validated = ValidatedFixture::new(fixture);
        
        if let FixtureValidation::ValidWithWarnings(issues) = &validated.validation {
            assert!(issues.iter().any(|i| i.category == IssueCategory::SuspiciousTime));
        } else {
            panic!("Expected validation warnings for suspicious time");
        }
    }

    #[test]
    fn test_missing_data() {
        let mut fixture = create_test_fixture();
        fixture.opponent = "TBD Opponent".to_string();
        fixture.venue = "Unknown Venue".to_string();
        
        let validated = ValidatedFixture::new(fixture);
        
        if let FixtureValidation::ValidWithWarnings(issues) = &validated.validation {
            assert!(issues.iter().any(|i| i.category == IssueCategory::MissingData));
            assert!(issues.len() >= 2); // Both opponent and venue issues
        } else {
            panic!("Expected validation warnings for missing data");
        }
    }

    #[test]
    fn test_date_range_validation() {
        let mut fixture = create_test_fixture();
        // Set fixture date to year 2030 (beyond 2 year limit from 2025)
        fixture.datetime = Utc.with_ymd_and_hms(2030, 8, 15, 16, 30, 0).unwrap();
        
        let validated = ValidatedFixture::new(fixture);
        assert!(!validated.is_usable()); // Should be unusable due to critical date range issue
        
        if let FixtureValidation::Invalid(issues) = &validated.validation {
            assert!(issues.iter().any(|i| i.severity == IssueSeverity::Critical));
            assert!(issues.iter().any(|i| i.category == IssueCategory::DataInconsistency));
        } else {
            panic!("Expected invalid fixture due to date range");
        }
    }

    #[test]
    fn test_calendar_description() {
        let mut fixture = create_test_fixture();
        // Set parse_metadata to trigger weekday mismatch warning
        fixture.parse_metadata.weekday_mismatch = Some(crate::parsing::WeekdayMismatch {
            claimed_weekday: "Sun".to_string(),
            actual_weekday: "Friday".to_string(),
            date: "Aug 15".to_string(),
        });
        fixture.parse_metadata.parsing_strategy = ParsingStrategy::WeekdayTolerant;
        
        let validated = ValidatedFixture::new(fixture);
        let description = validated.get_calendar_description();
        
        assert!(description.contains("Arsenal vs Chelsea"));
        assert!(description.contains("⚠️ Data Quality Notes"));
        assert!(description.contains("Date"));
    }
}