# CalPal - Sports Calendar Scraper

## Project Overview
A Rust-based sports calendar scraper that extracts Arsenal FC and Springboks rugby fixtures and makes them easily addable to Google Calendar. Full Rust stack including WebAssembly frontend.

**End Goal**: Help Ollie organize friend groups around sports events - starting with knowing when matches are so people can get together to watch.

## Architecture Decisions Made
- **Backend**: Axum for API server  
- **Frontend**: Leptos (WebAssembly) for reactive UI
- **Scraping**: chromiumoxide (headless browser) + scraper crate for dynamic content
- **Storage**: GitHub repository as JSON database  
- **Deployment**: Target cal.ollie.gg subdomain
- **Error Handling**: anyhow (single, simple error system)
- **CLI**: Beautiful Rust CLI with colors using clap + colored crates
- **Browser Automation**: chromiumoxide 0.7.0 with tokio runtime for JavaScript-heavy sites

## Project Structure
```
calpal/
├── CLAUDE.md           # This file - your briefing
├── fixture-scraper/    # Core scraping library with headless browser support
├── api/                # Axum web server  
├── frontend/           # Leptos WASM app
├── cli/                # Beautiful command-line interface
├── data/               # JSON fixture storage
└── .github/workflows/  # Automated scraping
```

## Core Principles
1. **Test-Driven Development**: Simple, direct tests that keep AI agents within logic bounds
2. **Teaching-First Code**: Every implementation teaches Rust patterns through clear comments
3. **Incremental Building**: Start simple, add complexity thoughtfully  
4. **Full Rust Stack**: Minimize non-Rust dependencies

## Scraping Sources  
- **Arsenal**: https://www.arsenal.com/fixtures (clean HTML structure with times/venues)
- **Springboks**: https://www.planetrugby.com/news/2025-rugby-championship-fixtures-and-kick-off-times-as-springboks-return-to-eden-park-before-twickenham-finale (detailed with kickoff times)
- **Alternative Springboks**: https://rugga.co.za/rugby-championship/confirmed-springboks-kick-off-times-for-2025-tests/ (confirmed times for all tests)

## Key Traits & Patterns
```rust
// Core scraper contract - all implementations must follow this
pub trait FixtureScraper {
    async fn scrape(&self) -> Result<Vec<Fixture>, ScrapeError>;
}

// Domain model - keep this consistent, with proper timezone handling
pub struct Fixture {
    pub team: String,
    pub opponent: String,
    pub datetime: DateTime<Utc>,  // Always store as UTC internally
    pub venue: String,
    pub competition: String,
    pub timezone_info: String,    // Original timezone for debugging
}
```

## Timezone Handling Strategy
- **Internal Storage**: Always convert to UTC using `chrono::DateTime<Utc>`
- **Source Parsing**: Handle SAST (UTC+2) from .co.za sites, GMT from UK sites  
- **User Display**: Convert to London time (GMT/BST) for Ollie's use
- **Libraries**: Use `chrono` and `chrono-tz` for robust timezone handling
- **Daylight Saving**: Properly handle GMT ↔ BST transitions in London
- **Validation**: Ensure fixture times make sense (no matches at 3 AM unless explicitly international)

## Testing Strategy & AI Guardrails
- **Direct, Achievable Tests**: Focus on essential functionality that should pass consistently  
- **AI Guardrails**: Tests prevent AI-generated spaghetti code, context loss, and bad software engineering practices
- **Test-Driven Development**: Claude Code should write tests frequently and look for testing opportunities
- **Never Change Tests to Fit Code**: If logic is obviously broken, fix the code. Otherwise, tests are the specification
- **Error Propagation Testing**: Validate that errors bubble up correctly through our anyhow chain

### Test Categories
- **Unit Tests**: Core functionality, parsing logic, error handling
- **Integration Tests**: Component boundaries, realistic scraping scenarios  
- **E2E Tests**: Critical user journeys (CLI scraping, web calendar integration)

## Error Handling Pattern
```rust
// Use anyhow for everything - simple and effective
use anyhow::{Context, Result};

pub async fn scrape_fixtures() -> Result<Vec<Fixture>> {
    let html = fetch_page().await.context("Failed to fetch fixtures page")?;
    let fixtures = parse_fixtures(&html).context("Failed to parse fixture data")?;
    Ok(fixtures)
}
```

## CLI vs Web App Functionality

### CLI Tool (`calpal` command)
- **Primary Purpose**: System operations, testing, development workflow
- **Commands**: 
  - `calpal scrape --team arsenal --output fixtures.json` 
  - `calpal scrape --all` (both teams)
  - `calpal serve --port 3000` (development server)
  - `calpal validate fixtures.json` (data validation)
- **Output**: Beautiful terminal output with colors, JSON files, validation reports
- **No Calendar Integration**: CLI focuses on data operations, not user calendar actions

### Web App
- **Primary Purpose**: User-facing calendar integration and fixture browsing  
- **Features**: Filter fixtures, generate calendar links, one-click calendar addition
- **UI**: Clean, responsive interface for organizing watching parties
- **Calendar Integration**: Simple calendar event URLs initially, OAuth later

## Future Extensibility (Keep in Mind)
- Pattern detection for adding new teams via URL
- OAuth Google Calendar integration  
- Friend management for organizing watching parties
- Multi-sport support beyond football and rugby
- Calendar conflict detection

## Notes
- This is a learning project first, real-world tool second
- **Clever & Teachable Code**: Demonstrate sophisticated Rust patterns while maintaining clarity and comprehensive documentation
- **Advanced Patterns**: Leverage zero-cost abstractions, type-driven design, and functional composition
- Tests should be direct and achievable, not academic
- AI agents should be kept in bounds by simple, passing tests

---
*Update this file as the project evolves and architectural decisions change.*

## Development Workflow

### Common Commands
```bash
# Development
cargo watch -x "test --lib"
cargo leptos watch       # Frontend development  
cargo run --bin calpal   # CLI tool
cargo fmt               # Format code to Rust standards
cargo clippy            # Rust best practices linting

# Testing  
cargo test
cargo test --package scraper
cargo test --integration

# Testing  
cargo test
cargo test --package scraper
cargo test --integration

# Building
cargo build --release
cargo leptos build      # Production WASM build
```

### Code Style Preferences  
- **Use 4-space indentation** (Rust standard, enforced by `cargo fmt`)
- Follow Rust conventions throughout (prefer Rust idioms over other language patterns)
- When completely sensible, and only then, gather inspiration from other paradigms and patterns to elevate the Rust experience
- Prefer explicit error handling over unwrap() - critical for AI-generated code
- Document public functions with examples showing expected behavior
- Keep markdown documentation updated alongside code changes, to help with understanding and AI agent context. The README.md should always be excellent.
- Use descriptive variable names (avoid single letters except short loops)
- Group imports: std, external crates, internal modules
- **NeoVim Friendly**: Ollie prefers terminal-based editing, optimize for vim workflows

### AI Agent Management
- **Claude Code Evolution**: Allow Claude Code to suggest improvements to CLAUDE.md files as knowledge grows
- **Memory Optimization**: Claude Code should leverage its own documentation knowledge for best practices  
- **Controlled Suggestions**: Claude Code can propose architectural changes but must justify with clear reasoning
- **Context Preservation**: Multiple CLAUDE.md files in subdirectories to maintain focused context
- **Guardrail Updates**: As the codebase grows, update testing strategies to prevent context drift

### Project Structure Guidelines
```
calpal/
├── CLAUDE.md           # This file
├── scraper/            # Core scraping library  
├── api/                # Axum web server
├── frontend/           # Leptos WASM app
├── cli/                # Command-line interface
├── data/               # JSON fixture storage
└── .github/workflows/  # Automated scraping
```

## 🎉 MAJOR MILESTONE: Sophisticated Architecture Complete ✅

### **Phase 1: Foundation** ✅ COMPLETE
1. ✅ Set up Rust workspace with bleeding-edge dependencies (Rust 1.88, all latest crates)
2. ✅ Implement FixtureScraper trait with comprehensive async trait documentation
3. ✅ Create Fixture domain model with proper timezone handling (UTC ↔ London)
4. ✅ Implement ScrapeError enum with Display trait and anyhow integration
5. ✅ Write comprehensive TDD test suite with AI guardrails
6. ✅ Establish workspace structure for fixture-scraper, api, frontend, cli packages

### **Phase 2: Sophisticated Parsing & Validation** ✅ COMPLETE
**🧠 Advanced Parsing System:**
- ✅ Multi-stage parsing with graceful degradation (exact → weekday-tolerant → year-assumption)
- ✅ Rich ParseMetadata structure replacing primitive timezone strings
- ✅ Weekday tolerance ("Sun Jan 15" when actual date is Wednesday)
- ✅ Time mocking for deterministic tests (works in 2027!)
- ✅ Shared parsing utilities across all team scrapers

**⚡ Intelligent Validation System:**
- ✅ Date range validation (current year to 2 years future only)
- ✅ Historical fixture detection and filtering
- ✅ Data quality warnings for calendar integration
- ✅ Three-tier severity: Warning → Error → Critical
- ✅ ValidatedFixture wrapper with rich calendar descriptions

**🎯 Production-Ready CLI:**
- ✅ Beautiful colored output with professional formatting
- ✅ Direct parser→validator flow (eliminated JSON middleware complexity)
- ✅ Rich error messages and help system
- ✅ Arsenal scraper integrated with shared parsing system

### **Testing Excellence:**
- ✅ **25/26 tests passing** (only 1 ignored integration test)
- ✅ Time-independent tests using mocked dates
- ✅ Comprehensive coverage: parsing, validation, error handling
- ✅ AI guardrails preventing context drift and bad practices

## 🎉 BREAKTHROUGH ACHIEVED: Real Arsenal Data! ✅

### **Phase 3: Production Arsenal Scraper** ✅ COMPLETE
**🏆 MAJOR MILESTONE - We moved from "sophisticated architecture" to "actually works with 42 real Arsenal fixtures"!**

**Real Data Success:**
- ✅ **42 Arsenal Fixtures** scraped from live website (vs previous 4 teaser fixtures)
- ✅ **Real Venue Data**: Emirates Stadium, Old Trafford, Anfield, Stamford Bridge, international venues  
- ✅ **Complete Season Coverage**: Friendlies, Premier League, cup competitions through May 2026
- ✅ **Proper Opponent Names**: Newcastle United, Tottenham, Villarreal, Athletic Club, etc.
- ✅ **Headless Browser Integration**: chromiumoxide successfully handles Arsenal's dynamic accordion content

**Technical Breakthroughs:**
- ✅ **CSS Selector Mastery**: Fixed from `.fixture-teaser` (4 results) to `div.accordions article` (42 results)
- ✅ **Venue Extraction**: Added `.event-info__venue` selector for real venue data
- ✅ **Opponent Parsing**: Using `h3.visually-hidden` content for accurate team names
- ✅ **Dynamic Content Handling**: Browser successfully waits for JavaScript accordion loading

**CLI Success:**
- ✅ **Beautiful Output**: 42 fixtures displayed with professional formatting  
- ✅ **Real Venues**: National Stadium (Singapore), Kai Tak Sports Park (Hong Kong), Premier League stadiums
- ✅ **Production Ready**: Direct pipeline from scraping → parsing → validation → display

### **Phase 4: Documentation & Organization** ✅ COMPLETE

**Completed Tasks:**
- ✅ **CLAUDE.md Update**: Documented breakthrough achievements and current architecture
- ✅ **README.md Update**: Showcases working Arsenal scraper with 42 fixtures 
- ✅ **Nested Documentation**: Created subdirectory CLAUDE.md files for growing codebase organization
- ✅ **Code Cleanup**: Perfect code quality with zero warnings and 37/37 tests passing

### **Phase 5: Production-Ready Foundation** ✅ COMPLETE

**Achieved Excellence:**
- ✅ **37/37 Tests Passing** - Comprehensive coverage including integration tests
- ✅ **Zero Clippy Warnings** - Clean, idiomatic Rust code throughout
- ✅ **Zero Compiler Warnings** - No dead code or unused imports
- ✅ **Production Build Success** - Release compilation verified
- ✅ **Beautiful CLI** - Professional colored output showing real fixture data
- ✅ **Robust Error Handling** - Graceful browser fallbacks and detailed error messages

## Current Phase: Expansion & Polish
**Immediate Next Actions:**
1. **Springboks Rugby Scraper** - apply proven headless browser approach to rugby sources
2. **Calendar Integration** - ICS file generation for Google Calendar, Outlook integration  
3. **Automated Scraping** - GitHub Actions for continuous fixture updates
4. **Nested Documentation** - organize growing codebase with subdirectory CLAUDE.md files

**Phase After: User Interface**
- Leptos WASM frontend displaying 42 Arsenal fixtures with venue information
- Rich calendar integration with data quality indicators from validation system
- Friend management for organizing watching parties around fixture schedule

## Memory Management & Context for Claude Code

### **Current Architecture (July 2025)**
**🏗️ Sophisticated Multi-Layer System:**
- **fixture-scraper** package: Core parsing and validation with bleeding-edge patterns
- **DateTimeParser**: Multi-stage parsing with time mocking support
- **ParseMetadata**: Rich structured data replacing primitive strings
- **ValidatedFixture**: Three-tier validation system (Warning/Error/Critical)
- **Arsenal scraper**: Integrated with shared parsing utilities
- **CLI**: Beautiful colored output with direct parser→validator flow

**🧠 Key Design Decisions:**
- **Time independence**: All tests use mocked dates for deterministic behavior
- **No JSON middleware**: Direct scraping→parsing→validation→display flow
- **Rich metadata**: Structured weekday mismatch detection, not string parsing
- **Graceful degradation**: Multiple parsing strategies with fallbacks
- **Calendar-first**: Validation designed for organizing watching parties

**🎯 Current Status (25/26 tests passing):**
- Architecture complete and production-ready
- Arsenal scraper needs real HTML selectors
- Ready for Springboks scraper implementation
- Calendar integration (ICS) ready for implementation

### **Development Philosophy for Future Contributors:**
- **Test-driven**: AI guardrails prevent context drift and bad patterns
- **Teaching-first**: Code demonstrates sophisticated Rust patterns with clear comments
- **Bleeding-edge**: Always use latest stable Rust and crate versions
- **Time-agnostic**: Tests must work in 2027 (no hardcoded dates)
- **User-focused**: Every feature serves Ollie's goal of organizing friend watching parties
