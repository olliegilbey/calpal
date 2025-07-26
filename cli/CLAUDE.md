# CLI Package

## Overview
Beautiful command-line interface for CalPal with professional colored output. Showcases the 42 real Arsenal fixtures with rich formatting and provides development/testing workflow for the fixture scraper system.

## Current Features

### Commands Implemented
- `calpal --help` - Show available commands and options
- `calpal teams` - List supported teams and their source URLs
- `calpal scrape --team <team>` - Scrape fixtures for specific team
- `calpal scrape --all` - Scrape all teams (Arsenal + future Springboks)

### Beautiful Output
- **Colored formatting** using `colored` crate
- **Professional layout** with clear headers and sections
- **Real fixture data** showing 42 Arsenal fixtures with venues
- **Validation indicators** from three-tier system
- **Error handling** with helpful user messages

## Architecture

### CLI Structure
```
cli/src/
└── main.rs              # Command-line interface with clap integration
```

### Integration Points
- **fixture-scraper** package for core functionality
- **ValidatedFixture** display with quality indicators
- **ScrapeError** handling with user-friendly messages
- **clap** for argument parsing and help generation

## Design Philosophy

### Purpose: Development & Testing
- **System operations** for fixture data management
- **Testing workflow** for scraper development
- **Data validation** and quality assessment
- **No calendar integration** (that's for the web app)

### User Experience
- **Beautiful terminal output** with colors and formatting
- **Clear error messages** with actionable guidance
- **Progress indicators** for long-running operations
- **Helpful defaults** and validation

## Output Examples

### Successful Arsenal Scraping
```
🎯 CalPal - Sports Calendar Scraper

✅ Arsenal Fixtures (42 found)
────────────────────────────────────

📅 Arsenal vs Newcastle United
   🏟️  Emirates Stadium
   📊 Premier League
   ⏰ 2025-08-15 14:30 UTC (15:30 BST)
   ✅ High Quality - Ready for calendar

📅 Arsenal vs Tottenham
   🏟️  Emirates Stadium  
   📊 Premier League
   ⏰ 2025-09-12 16:30 UTC (17:30 BST)
   ✅ High Quality - Ready for calendar

[... 40 more fixtures ...]

📊 Summary: 42 fixtures found, 41 high quality, 1 with warnings
```

### Error Handling
```
❌ Scraping failed for Arsenal
🔍 Error: Network error: Failed to fetch Arsenal fixtures: Connection timeout

💡 Suggestions:
   • Check your internet connection
   • Try again in a few moments
   • Use --verbose for detailed debugging
```

## Implementation Notes

### Command Structure
- **clap** for robust argument parsing
- **Subcommands** for different operations (scrape, teams, validate)
- **Global options** like --verbose, --output, --format
- **Help system** with examples and usage patterns

### Output Formatting
- **Colored output** with semantic meaning (green=success, red=error, yellow=warning)
- **Unicode symbols** for visual hierarchy (✅❌🎯📅🏟️📊⏰)
- **Structured layout** with clear sections and spacing
- **Terminal-friendly** design for various screen sizes

### Error Handling Strategy
- **User-friendly messages** avoiding technical jargon
- **Actionable suggestions** for common failure modes
- **Debug information** available with --verbose flag
- **Graceful degradation** for non-color terminals

## Future Enhancements

### Additional Commands
- `calpal validate <file>` - Validate fixture JSON files
- `calpal export --format ics` - Export to calendar format
- `calpal debug --team <team>` - Debug scraper issues
- `calpal watch --team <team>` - Watch for fixture updates

### Output Improvements
- **JSON output** option for programmatic use
- **Table formatting** for fixture listings
- **Progress bars** for long operations
- **Interactive prompts** for configuration

### Integration Features
- **Configuration file** support for user preferences
- **Output caching** for faster repeated commands
- **Fixture comparison** showing changes over time
- **Quality reports** with detailed validation results

## Development Guidelines

### Adding New Commands
1. Define clap subcommand structure
2. Implement command handler function
3. Add appropriate output formatting
4. Include error handling and user guidance
5. Write tests for command behavior

### Output Formatting Standards
- Use semantic colors consistently
- Provide both colored and plain output modes
- Include unicode symbols for visual hierarchy
- Maintain consistent spacing and alignment
- Support various terminal widths

### Error Message Guidelines
- Use clear, non-technical language
- Provide specific suggestions for resolution
- Include relevant context and debugging information
- Maintain helpful tone without being condescending
- Use consistent formatting across all error types

## Testing Strategy

### CLI Testing
- **Command parsing** verification
- **Output formatting** validation
- **Error handling** scenarios
- **Integration tests** with real scraper data

### User Experience Testing
- **Terminal compatibility** across different environments
- **Color support** detection and fallbacks
- **Command completion** and help system
- **Error recovery** and user guidance