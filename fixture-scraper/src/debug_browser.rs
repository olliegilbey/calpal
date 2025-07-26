//! Debug utility to fetch and examine Arsenal's rendered HTML structure

use crate::browser::BrowserScraper;
use crate::ScrapeError;

/// Debug function to fetch Arsenal's fixtures page and save the rendered HTML
pub async fn debug_arsenal_html() -> Result<(), ScrapeError> {
    let browser = BrowserScraper::new().await?;
    let html = browser
        .get_rendered_html("https://www.arsenal.com/fixtures")
        .await?;

    // Save to a file for analysis
    std::fs::write("/tmp/arsenal_rendered.html", &html)
        .map_err(|e| ScrapeError::Parse(format!("Failed to write debug file: {e}")))?;

    println!("Arsenal rendered HTML saved to /tmp/arsenal_rendered.html");
    println!("HTML length: {} characters", html.len());

    // Look for fixture-related patterns
    if html.contains("fixture") {
        println!("✓ Found 'fixture' in HTML");
    }
    if html.contains("match") {
        println!("✓ Found 'match' in HTML");
    }
    if html.contains("vs") || html.contains("v ") {
        println!("✓ Found opponent indicators in HTML");
    }
    if html.contains("Premier League") {
        println!("✓ Found 'Premier League' in HTML");
    }

    // Count fixture elements
    use scraper::{Html, Selector};
    let document = Html::parse_document(&html);

    // Check teaser fixtures
    let fixture_selector = Selector::parse(".fixture-teaser").unwrap();
    let fixture_count = document.select(&fixture_selector).count();
    println!(
        "📊 Found {fixture_count} .fixture-teaser elements (teasers)"
    );

    // Check accordion fixtures
    let accordion_selector = Selector::parse("div.accordions").unwrap();
    let accordion_count = document.select(&accordion_selector).count();
    println!("📊 Found {accordion_count} .accordions containers");

    // Check article fixtures inside accordions
    let article_selector = Selector::parse("div.accordions article").unwrap();
    let article_count = document.select(&article_selector).count();
    println!(
        "📊 Found {article_count} article elements inside accordions"
    );

    // Check for /fixture/arsenal URLs
    let fixture_url_count = html.matches("/fixture/arsenal").count();
    println!("📊 Found {fixture_url_count} '/fixture/arsenal' URLs");

    // Test the new selector we're using in the scraper
    let new_fixture_selector = Selector::parse("div.accordions article.card").unwrap();
    let new_fixture_count = document.select(&new_fixture_selector).count();
    println!(
        "📊 Found {new_fixture_count} 'div.accordions article.card' elements (our target)"
    );

    // Look for venue patterns
    if html.contains("Emirates Stadium") {
        println!("✓ Found 'Emirates Stadium' in HTML");
    }
    if html.contains("venue") {
        println!("✓ Found 'venue' in HTML");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_debug_arsenal_html() {
        let result = debug_arsenal_html().await;
        match result {
            Ok(_) => println!("Debug successful"),
            Err(e) => println!("Debug failed: {e}"),
        }
    }
}
