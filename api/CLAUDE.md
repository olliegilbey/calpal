# API Package

## Overview
Future Axum-based REST API server for CalPal. Will expose fixture data and validation information through clean JSON endpoints, supporting the Leptos frontend and potential third-party integrations.

## Planned Architecture

### REST Endpoints (Ready for Implementation)
```
GET  /api/fixtures/{team}           # Get validated fixtures for team
GET  /api/fixtures                  # Get all fixtures across teams
GET  /api/teams                     # List supported teams and metadata
GET  /api/health                    # Health check and system status
POST /api/scrape/{team}             # Trigger scraping for specific team
GET  /api/calendar/{team}.ics       # Export fixtures as ICS calendar
```

### Response Format
All endpoints return structured JSON with validation metadata:

```rust
#[derive(Serialize)]
pub struct FixtureResponse {
    pub fixtures: Vec<ValidatedFixture>,
    pub metadata: ScrapeMetadata,
    pub quality_summary: QualitySummary,
}

#[derive(Serialize)]
pub struct ScrapeMetadata {
    pub scraped_at: DateTime<Utc>,
    pub source_url: String,
    pub team_name: String,
    pub parsing_strategy: String,
}
```

## Integration Points

### fixture-scraper Package
- **Direct ValidatedFixture usage** from scraper system
- **Rich validation data** exposed through API responses
- **ScrapeError handling** converted to appropriate HTTP status codes
- **Real-time scraping** triggered via POST endpoints

### Frontend Communication
- **JSON serialization** of all fixture data structures
- **Quality indicators** from three-tier validation system
- **Error responses** with user-friendly messages
- **CORS support** for local development and production

## Technical Approach

### Axum Framework
- **Modern async** Rust web framework
- **Type-safe routing** with extractors
- **Middleware support** for logging, CORS, rate limiting
- **JSON serialization** with serde integration

### Error Handling Strategy
```rust
#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// HTTP status mapping:
// ScrapeError::Network -> 502 Bad Gateway
// ScrapeError::Parse -> 502 Bad Gateway  
// ScrapeError::InvalidDateTime -> 502 Bad Gateway
// ScrapeError::MissingElement -> 404 Not Found
```

### Performance Considerations
- **Response caching** for recently scraped data
- **Background scraping** with cached results
- **Rate limiting** to prevent abuse of scraping endpoints
- **Streaming responses** for large fixture datasets

## Future Features

### Authentication & Authorization
- **API keys** for third-party access
- **Rate limiting** per user/key
- **Usage analytics** and monitoring
- **CORS policies** for different environments

### Advanced Endpoints
- **Fixture filtering** by date range, competition, venue
- **Data quality reports** with detailed validation results
- **Historical data** tracking changes over time
- **Webhook support** for fixture updates

### Calendar Integration
- **ICS file generation** with proper timezone handling
- **Google Calendar** OAuth integration
- **Outlook integration** with proper formatting
- **Custom calendar** formats for different clients

## Implementation Plan

### Phase 1: Core API
1. Basic Axum server setup with health endpoint
2. Integration with fixture-scraper for Arsenal data
3. JSON endpoints for fixture retrieval
4. Error handling and logging middleware

### Phase 2: Enhanced Features  
1. Real-time scraping endpoints
2. ICS calendar export functionality
3. Quality reporting and validation details
4. Caching layer for performance

### Phase 3: Production Ready
1. Authentication and rate limiting
2. Comprehensive monitoring and logging
3. API documentation with OpenAPI/Swagger
4. Deployment configuration for cal.ollie.gg

## Development Guidelines

### API Design Principles
- **RESTful conventions** with consistent endpoint naming
- **JSON-first** with proper content-type headers
- **Semantic HTTP status codes** for different error types
- **Comprehensive error messages** with actionable guidance

### Data Serialization
- **ValidatedFixture** exposed with all validation metadata
- **Timezone consistency** (UTC in JSON, display conversion in frontend)
- **Quality indicators** clearly marked in responses
- **Parsing metadata** included for debugging and transparency

### Testing Strategy
- **Integration tests** with real fixture-scraper data
- **API contract tests** ensuring response format stability
- **Error scenario testing** for various failure modes
- **Performance testing** for scraping and caching behavior

### Security Considerations
- **Input validation** for all request parameters
- **Rate limiting** to prevent scraping abuse
- **CORS policies** appropriate for deployment environment
- **No sensitive data exposure** in error messages or logs

## Dependencies

### Core Framework
- **axum 0.8.4** - Modern async web framework
- **tower 0.5.2** - Middleware and service abstractions
- **hyper** - HTTP implementation (via axum)
- **tokio** - Async runtime

### Serialization & Data
- **serde_json** - JSON serialization of fixture data
- **chrono** - Date/time handling in API responses
- **uuid** - Request ID generation for tracing

### Production Features
- **tower-http** - HTTP middleware (CORS, logging, compression)
- **tracing** - Structured logging and observability
- **anyhow** - Error handling integration with scraper