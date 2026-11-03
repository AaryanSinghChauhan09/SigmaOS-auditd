use anyhow::Result;
use clap::Parser;
use auditd::auditd;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = auditd::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = auditd::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-auditd operational");
    Ok(())
}
