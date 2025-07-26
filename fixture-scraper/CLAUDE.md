# Fixture Scraper Package

## Overview
The core parsing and validation engine for CalPal. This package contains the sophisticated multi-stage parsing system, three-tier validation, and all team-specific scrapers including the breakthrough Arsenal implementation.

## Architecture Achievements

### 🏆 Production Arsenal Scraper (Working!)
- **42 Real Arsenal Fixtures** extracted from live website
- **Headless Browser Integration** with chromiumoxide for dynamic content
- **Real Venue Data**: Emirates Stadium, Old Trafford, Anfield, etc.
- **Complete Season Coverage**: Friendlies, Premier League, competitions through May 2026

### ✅ Production-Ready Code Quality
- **37/37 Tests Passing** - Comprehensive unit, integration, and browser tests
- **Zero Clippy Warnings** - Clean, idiomatic Rust throughout
- **Zero Compiler Warnings** - No dead code or unused imports
- **Production Build Verified** - Release compilation successful

### 🧠 Multi-Stage Parsing System
- **Graceful Degradation**: Exact → Weekday tolerance → Year assumptions
- **Rich ParseMetadata**: Structured data replacing primitive strings
- **Time Independence**: Deterministic tests using mocked dates (works in 2027!)
- **Shared Utilities**: DateTimeParser used across all team scrapers

### ⚡ Three-Tier Validation System  
- **ValidatedFixture Wrapper**: Ready for calendar integration
- **Quality Assessment**: Warning → Error → Critical severity levels
- **Date Range Filtering**: Current year to 2 years future only
- **Calendar Descriptions**: Rich output with data quality notes

## Module Structure

```
fixture-scraper/src/
├── lib.rs              # Core traits, domain models, comprehensive tests
├── parsing.rs          # Multi-stage DateTimeParser with metadata
├── validation.rs       # Three-tier ValidatedFixture system
├── browser.rs          # Headless browser automation for dynamic content
├── debug_browser.rs    # Debug utilities for HTML structure analysis
└── arsenal.rs          # Production Arsenal scraper with 42 fixtures
```

## Key Traits & Models

### FixtureScraper Trait
All team scrapers implement this async trait:

```rust
#[async_trait]
pub trait FixtureScraper {
    async fn scrape(&self) -> Result<Vec<ValidatedFixture>, ScrapeError>;
    fn team_name(&self) -> &str;
    fn source_url(&self) -> &str;
}
```

### Rich Fixture Model
```rust
pub struct Fixture {
    pub team: String,
    pub opponent: String,
    pub datetime: DateTime<Utc>,        // Always UTC internally
    pub venue: String,
    pub competition: String,
    pub parse_metadata: ParseMetadata,  // Rich structured metadata
}
```

### ParseMetadata Structure
```rust
pub struct ParseMetadata {
    pub original_source: String,
    pub weekday_mismatch: Option<WeekdayMismatch>,
    pub timezone_assumptions: String,
    pub parsing_strategy: ParsingStrategy,
}
```

## Technical Breakthroughs

### Arsenal Scraper Success
- **CSS Selector Discovery**: Fixed from `.fixture-teaser` (4 results) to `div.accordions article` (42 results)
- **Venue Extraction**: Added `.event-info__venue` selector for real venue data
- **Opponent Parsing**: Using `h3.visually-hidden` content for accurate team names
- **Dynamic Content**: Browser waits for JavaScript accordion loading

### Headless Browser Integration
- **chromiumoxide 0.7.0** with tokio runtime
- **Selective Use**: Only for JavaScript-heavy sites (Arsenal)
- **Performance Optimized**: Proper wait strategies, disabled images/CSS
- **Error Handling**: Rich debugging for dynamic site troubleshooting

## Testing Philosophy

### AI Guardrails
- **Time-independent tests** prevent context drift
- **Direct assertions** keep implementations focused
- **Mocked dates** ensure deterministic behavior
- **Comprehensive coverage** of parsing, validation, error handling

### Test Categories
- **Unit Tests**: Core functionality, parsing logic, timezone handling
- **Integration Tests**: Real scraping scenarios (with `#[ignore]` for CI)
- **Property Tests**: Validation behavior across different input patterns

## Implementation Notes

### Timezone Strategy
- **Internal Storage**: Always UTC using `chrono::DateTime<Utc>`
- **Source Parsing**: Handle local time zones (GMT, BST, SAST)
- **Display Conversion**: Convert to London time for user interface
- **DST Handling**: Proper GMT ↔ BST transitions

### Error Handling
- **Structured ScrapeError enum** for different failure modes
- **anyhow integration** for error propagation
- **Rich error messages** with context and debugging information

### Browser vs HTTP Strategy
- **Default to HTTP** for simple, stable content
- **Upgrade to Browser** for dynamic JavaScript content
- **Fallback Pattern**: Browser → HTTP → Error
- **Performance**: Cache browser instances, reuse tabs

## Future Expansion

### Ready for Springboks
- **Proven headless browser approach** ready for rugby sites
- **Shared parsing utilities** handle different date formats
- **Validation system** supports rugby-specific requirements

### Calendar Integration
- **ICS file generation** ready for implementation
- **Quality indicators** from validation system
- **Rich descriptions** with venue and competition data

## Development Guidelines

### Adding New Team Scrapers
1. Implement `FixtureScraper` trait
2. Use shared `DateTimeParser` for consistency
3. Create `ParseMetadata` with source details
4. Wrap results in `ValidatedFixture`
5. Write comprehensive tests with time mocking

### Browser Integration
1. Check if content is dynamic (JavaScript-loaded)
2. Use `BrowserScraper::new().await` for setup
3. Target specific CSS selectors for fixture data
4. Handle async content loading with proper waits
5. Provide HTTP fallback for resilience

### Testing Standards
- Use mocked dates for time independence
- Test parsing edge cases (weekday mismatches, year boundaries)
- Validate error handling for malformed input
- Include integration tests with `#[ignore]` for live scraping