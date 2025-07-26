use anyhow::{Context, Result};
use chrono::{Timelike, Utc};
use clap::{Args, Parser, Subcommand};
use colored::*;
use fixture_scraper::{arsenal::ArsenalScraper, Fixture, FixtureScraper, validation::ValidatedFixture};
use serde_json;
use std::path::PathBuf;
use tokio;

#[derive(Parser)]
#[command(name = "calpal")]
#[command(about = "CalPal - Sports Calendar Scraper")]
#[command(version = "0.1.0")]
#[command(author = "Ollie <ollie@example.com>")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(global = true, long, help = "Enable verbose output")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Scrape fixtures from team websites")]
    Scrape(ScrapeArgs),
    
    #[command(about = "Show information about supported teams")]
    Teams,
}

#[derive(Args)]
struct ScrapeArgs {
    #[arg(short, long, help = "Team to scrape (arsenal, springboks, all)")]
    team: String,
    
    #[arg(short, long, help = "Output file for JSON data")]
    output: Option<PathBuf>,
    
    #[arg(long, help = "Pretty print JSON output")]
    pretty: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Scrape(args) => scrape_command(args, cli.verbose).await,
        Commands::Teams => teams_command(),
    }
}

async fn scrape_command(args: ScrapeArgs, verbose: bool) -> Result<()> {
    print_banner();
    
    if verbose {
        println!("{}", "🔍 Verbose mode enabled".dimmed());
    }
    
    let team_name = args.team.to_lowercase();
    
    match team_name.as_str() {
        "arsenal" => {
            scrape_team_fixtures(ArsenalScraper::new(), &args, verbose).await
        }
        "all" => {
            println!("{}", "🌟 Scraping all supported teams...".bright_blue().bold());
            scrape_team_fixtures(ArsenalScraper::new(), &args, verbose).await?;
            println!("{}", "✅ All teams scraped successfully!".bright_green().bold());
            Ok(())
        }
        _ => {
            eprintln!("{}", format!("❌ Unsupported team: {}", team_name).bright_red());
            eprintln!("{}", "💡 Supported teams: arsenal, all".bright_yellow());
            std::process::exit(1);
        }
    }
}

async fn scrape_team_fixtures<T: FixtureScraper>(
    scraper: T, 
    args: &ScrapeArgs, 
    verbose: bool
) -> Result<()> {
    let team_name = scraper.team_name();
    let source_url = scraper.source_url();
    
    println!("{}", format!("🚀 Scraping {} fixtures...", team_name).bright_blue().bold());
    
    if verbose {
        println!("{}", format!("📡 Source: {}", source_url).dimmed());
        println!("{}", format!("⏰ Started at: {}", Utc::now().format("%Y-%m-%d %H:%M:%S UTC")).dimmed());
    }
    
    // Scrape fixtures with error handling
    let fixtures = match scraper.scrape().await {
        Ok(fixtures) => {
            println!("{}", format!("✅ Successfully scraped {} fixtures", fixtures.len()).bright_green());
            fixtures
        }
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to scrape {}: {}", team_name, e).bright_red());
            return Err(anyhow::anyhow!("Scraping failed: {}", e));
        }
    };
    
    // Display fixtures in a beautiful table format
    print_fixtures_table(&fixtures, verbose);
    
    // Save to file if requested
    if let Some(output_path) = &args.output {
        save_fixtures_to_file(&fixtures, output_path, args.pretty)
            .context("Failed to save fixtures to file")?;
        
        println!("{}", format!("💾 Saved {} fixtures to {}", 
            fixtures.len(), 
            output_path.display()).bright_green());
    }
    
    Ok(())
}

fn print_fixtures_table(fixtures: &[ValidatedFixture], verbose: bool) {
    if fixtures.is_empty() {
        println!("{}", "⚠️  No fixtures found".bright_yellow());
        return;
    }
    
    println!("\n{}", "📅 Upcoming Fixtures".bright_magenta().bold().underline());
    println!();
    
    // Table header
    println!("{:<4} {:<12} {:<20} {:<15} {:<25} {:<15}", 
        "#".dimmed(),
        "Date".bright_cyan().bold(),
        "Match".bright_white().bold(), 
        "Time".bright_yellow().bold(),
        "Venue".bright_green().bold(),
        "Competition".bright_blue().bold()
    );
    
    println!("{}", "─".repeat(95).dimmed());
    
    // Table rows
    for (i, validated_fixture) in fixtures.iter().enumerate() {
        let fixture = &validated_fixture.fixture;
        let london_time = fixture.to_london_time();
        let date_str = london_time.format("%a %b %d").to_string();
        let time_str = london_time.format("%H:%M").to_string();
        let match_str = format!("{} vs {}", fixture.team, fixture.opponent);
        
        // Color coding for teams
        let colored_match = if fixture.team.contains("Arsenal") {
            match_str.bright_red()
        } else {
            match_str.bright_white()
        };
        
        println!("{:<4} {:<12} {:<20} {:<15} {:<25} {:<15}",
            format!("{}", i + 1).dimmed(),
            date_str.bright_cyan(),
            colored_match,
            time_str.bright_yellow(),
            fixture.venue.bright_green(),
            fixture.competition.bright_blue()
        );
        
        if verbose {
            println!("     {}", format!("UTC: {} | Parsing: {}", 
                fixture.datetime.format("%Y-%m-%d %H:%M UTC"),
                fixture.parse_metadata.timezone_assumptions).dimmed());
        }
    }
    
    println!();
}

fn save_fixtures_to_file(fixtures: &[ValidatedFixture], path: &PathBuf, pretty: bool) -> Result<()> {
    let json_data = if pretty {
        serde_json::to_string_pretty(fixtures)
    } else {
        serde_json::to_string(fixtures)
    }.context("Failed to serialize fixtures to JSON")?;
    
    std::fs::write(path, json_data)
        .context("Failed to write JSON data to file")?;
    
    Ok(())
}


fn teams_command() -> Result<()> {
    print_banner();
    
    println!("{}", "🏈 Supported Teams".bright_magenta().bold().underline());
    println!();
    
    println!("{:<15} {:<30} {:<40}", 
        "Team".bright_cyan().bold(),
        "Sport".bright_green().bold(),
        "Source".bright_blue().bold()
    );
    println!("{}", "─".repeat(85).dimmed());
    
    println!("{:<15} {:<30} {:<40}",
        "Arsenal".bright_red(),
        "Football (Premier League)".bright_green(),
        "https://www.arsenal.com/fixtures".bright_blue()
    );
    
    println!("{:<15} {:<30} {:<40}",
        "Springboks".bright_green(),
        "Rugby (International)".bright_green(),
        "https://planetrugby.com/...".bright_blue().dimmed()
    );
    
    println!();
    println!("{}", "💡 Use 'calpal scrape --team <team>' to scrape fixtures".bright_yellow());
    println!("{}", "💡 Use 'calpal scrape --team all' to scrape all teams".bright_yellow());
    
    Ok(())
}

fn print_banner() {
    println!("{}", r#"
    ╔═══════════════════════════════════════╗
    ║              🏈 CalPal 🏈              ║
    ║        Sports Calendar Scraper        ║
    ║      Built with Rust 🦀 & Love       ║
    ╚═══════════════════════════════════════╝
    "#.bright_magenta().bold());
}