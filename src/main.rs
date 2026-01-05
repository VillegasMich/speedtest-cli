mod cli;
mod error;
mod output;
mod speedtest;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(if cli.verbose { "debug" } else { "warn" }),
    )
    .init();

    log::info!("Speed Test CLI started");

    if let Err(e) = cli.execute().await {
        log::error!("Application error: {}", e);
        std::process::exit(1);
    }
}
