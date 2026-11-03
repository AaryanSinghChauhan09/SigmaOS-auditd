use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use auditd::auditd;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = auditd::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = auditd::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-auditd operational");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
