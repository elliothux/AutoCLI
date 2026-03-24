mod args;
mod execution;

use clap::{Arg, ArgAction, Command};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // 1. Initialize tracing with OPENCLI_VERBOSE / RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
                if std::env::var("OPENCLI_VERBOSE").is_ok() {
                    EnvFilter::new("debug")
                } else {
                    EnvFilter::new("warn")
                }
            }),
        )
        .init();

    // 2. Build clap App
    // For now, just a basic app with version, about, and global options
    // Later phases will dynamically add subcommands from discovered adapters
    let app = Command::new("opencli-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("AI-driven CLI tool — turns websites into command-line interfaces")
        .arg(
            Arg::new("format")
                .long("format")
                .short('f')
                .global(true)
                .default_value("table")
                .help("Output format: table, json, yaml, csv, md"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .global(true)
                .action(ArgAction::SetTrue)
                .help("Enable verbose output"),
        );

    // 3. Parse and route
    let matches = app.get_matches();
    let _format = matches.get_one::<String>("format").unwrap().clone();
    let verbose = matches.get_flag("verbose");

    if verbose {
        tracing::info!("Verbose mode enabled");
    }

    // At this stage, just print help if no subcommand
    // Future: dynamic subcommands from registry
    eprintln!("opencli-rs v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("No command specified. Use --help for usage.");
}
