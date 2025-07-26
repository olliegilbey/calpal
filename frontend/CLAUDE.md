# Frontend Package

## Overview
Future Leptos WebAssembly frontend for CalPal. Will provide a beautiful, reactive interface for browsing Arsenal fixtures, organizing watching parties, and integrating with calendar systems. Designed to showcase the 42 real Arsenal fixtures with rich validation metadata.

## Planned Architecture

### Leptos WASM Application
- **Full-stack Rust** with WebAssembly compilation
- **Reactive UI** with component-based architecture
- **Server-side rendering** support for SEO and performance
- **Type-safe API** communication with backend

### Core Components (Planned)
```
frontend/src/
├── lib.rs                  # Main app and routing
├── components/
│   ├── fixture_list.rs     # Arsenal fixture display with venues
│   ├── fixture_card.rs     # Individual fixture with quality indicators
│   ├── calendar_export.rs  # Calendar integration controls
│   ├── quality_badge.rs    # Validation status display
│   └── team_selector.rs    # Team navigation (Arsenal, Springboks)
├── pages/
│   ├── home.rs            # Landing page with feature overview
│   ├── arsenal.rs         # Arsenal fixture browsing
│   ├── springboks.rs      # Rugby fixture browsing (future)
│   └── calendar.rs        # Calendar integration page
└── api/
    └── client.rs          # Type-safe API client for backend
```

## User Experience Goals

### Primary Features
- **Fixture Browsing** - Beautiful display of 42 Arsenal fixtures with venues
- **Quality Indicators** - Visual representation of three-tier validation system
- **Calendar Integration** - One-click export to Google Calendar, Outlook
- **Watching Party Planning** - Friend coordination around fixture schedule

### Visual Design
- **Clean, Modern UI** with responsive design
- **Real Venue Information** prominently displayed (Emirates Stadium, etc.)
- **Competition Context** (Premier League, friendlies, cup matches)
- **Date/Time Clarity** with London timezone display

## Technical Approach

### Leptos Framework
- **Component-based** reactive UI with signals
- **Type-safe** communication with Rust backend
- **WebAssembly** for performance and shared code
- **SSR support** for better initial load times

### Data Integration
```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct FixtureDisplay {
    pub fixture: Fixture,
    pub validation: FixtureValidation,
    pub quality_badge: QualityBadge,
    pub calendar_ready: bool,
}

// API client integration
pub async fn fetch_arsenal_fixtures() -> Result<Vec<ValidatedFixture>, ApiError> {
    // Type-safe communication with Axum backend
}
```

### State Management
- **Leptos signals** for reactive fixture data
- **Local storage** for user preferences
- **API caching** for performance
- **Error boundaries** for graceful failure handling

## Feature Specifications

### Fixture Display
- **42 Arsenal Fixtures** in clean, scannable format
- **Venue Emphasis** - Emirates Stadium, Old Trafford, Anfield prominently shown
- **Competition Context** - Premier League, friendlies, cup matches clearly marked
- **Date/Time Display** - London timezone with BST/GMT awareness

### Quality Indicators
- **Three-Tier System** visualization (Warning/Error/Critical)
- **ParseMetadata** exposure for transparency
- **Confidence Levels** for calendar integration
- **Error Details** with helpful explanations

### Calendar Integration
- **One-Click Export** to ICS format
- **Google Calendar** direct integration (OAuth flow)
- **Outlook Support** with proper formatting
- **Quality Warnings** before calendar addition

### Friend Coordination (Future)
- **Watching Party** creation around fixtures
- **Friend Groups** for different types of matches
- **Location Suggestions** based on venue and match importance
- **Notification System** for fixture changes

## Component Design

### FixtureCard Component
```rust
#[component]
pub fn FixtureCard(fixture: ValidatedFixture) -> impl IntoView {
    view! {
        <div class="fixture-card">
            <div class="teams">
                <span class="team">{fixture.fixture.team}</span>
                <span class="vs">vs</span>
                <span class="opponent">{fixture.fixture.opponent}</span>
            </div>
            <div class="venue">{fixture.fixture.venue}</div>
            <div class="datetime">{fixture.fixture.to_london_time().format("%Y-%m-%d %H:%M")}</div>
            <QualityBadge validation={fixture.validation} />
            <CalendarButton fixture={fixture} />
        </div>
    }
}
```

### QualityBadge Component
```rust
#[component]
pub fn QualityBadge(validation: FixtureValidation) -> impl IntoView {
    let (status, color, tooltip) = match validation {
        FixtureValidation::Valid => ("✅ Ready", "green", "High quality - ready for calendar"),
        FixtureValidation::ValidWithWarnings(ref issues) => 
            ("⚠️ Warnings", "yellow", &format!("{} warnings - check details", issues.len())),
        FixtureValidation::Invalid(ref issues) => 
            ("❌ Issues", "red", &format!("{} critical issues", issues.len())),
        FixtureValidation::Historical(_) => 
            ("📅 Past", "gray", "Historical fixture - filtered from calendar"),
    };
    
    view! {
        <span class="quality-badge" style=format!("color: {}", color) title={tooltip}>
            {status}
        </span>
    }
}
```

## Development Plan

### Phase 1: Core Display
1. Basic Leptos app with routing
2. Arsenal fixture display from API
3. Responsive design with venue emphasis
4. Quality indicator visualization

### Phase 2: Calendar Integration
1. ICS export functionality
2. Google Calendar OAuth flow
3. Calendar preview and customization
4. Quality warnings before export

### Phase 3: Enhanced UX
1. Advanced filtering and sorting
2. Fixture search and favorites
3. Watching party planning features
4. Springboks rugby integration

## Integration Points

### Backend API
- **Type-safe client** generated from API schemas
- **Real-time updates** for fixture changes
- **Error handling** with user-friendly messages
- **Caching strategy** for performance

### Calendar Systems
- **ICS generation** with proper timezone handling
- **OAuth integration** for Google Calendar
- **Outlook integration** with appropriate formatting
- **Quality metadata** in calendar descriptions

## Styling & Design

### CSS Framework
- **Modern CSS** with CSS Grid and Flexbox
- **Responsive design** for mobile and desktop
- **Color scheme** reflecting quality status
- **Typography** optimized for fixture data scanning

### Visual Hierarchy
- **Fixture prominence** with clear team vs opponent display
- **Venue emphasis** since location affects watching party planning
- **Competition context** for match importance assessment
- **Quality indicators** prominently but not obtrusively displayed

## Testing Strategy

### Component Testing
- **Individual component** behavior and rendering
- **Props handling** and state management
- **User interaction** simulation
- **Error boundary** testing

### Integration Testing
- **API communication** with backend
- **Calendar export** functionality
- **Quality indicator** accuracy
- **Responsive design** across devices

### User Experience Testing
- **Fixture browsing** workflow
- **Calendar integration** flow
- **Error handling** user experience
- **Performance** on various devices

## Dependencies

### Core Framework
- **leptos 0.8.5** - Reactive web framework
- **wasm-bindgen 0.2.100** - WebAssembly bindings
- **web-sys** - Browser API bindings
- **serde-wasm-bindgen** - Serialization for WASM

### UI & Styling
- **leptos_dom** - DOM manipulation
- **leptos_router** - Client-side routing
- **console_log** - Browser console integration for debugging

### API Integration
- **reqwest** - HTTP client (WASM-compatible)
- **serde_json** - API response deserialization
- **chrono** - Date/time handling in UI