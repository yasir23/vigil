use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod detect;
mod doctor;
mod hunt;
mod ir;
mod onboard;
mod playbook;
mod source;
mod vuln;

#[derive(Parser)]
#[command(
    name = "vigil",
    version,
    about = "Your own personal threat intelligence agent. Any OS. Any Platform. The security-first way.",
    long_about = "Vigil Bot is an autonomous threat intelligence agent you run on your own machine. \
                  It hunts threats, generates detection rules, investigates incidents, and pivots on \
                  indicators of compromise — all from your terminal, with no cloud dependency required."
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress output (only errors)
    #[arg(short, long, global = true)]
    silent: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the onboarding wizard
    Onboard(onboard::OnboardCommand),

    /// Hunt for threats and investigate IOCs
    Hunt(hunt::HuntCommand),

    /// Generate and test detection rules
    Detect(detect::DetectCommand),

    /// Incident response workflows
    Ir(ir::IrCommand),

    /// Vulnerability analysis
    Vuln(vuln::VulnCommand),

    /// Manage data sources and threat intel feeds
    Source(source::SourceCommand),

    /// Manage playbooks
    Playbook(playbook::PlaybookCommand),

    /// Manage configuration
    Config(config::ConfigCommand),

    /// Health check and diagnostics
    Doctor(doctor::DoctorCommand),
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Onboard(cmd) => cmd.execute().await,
            Commands::Hunt(cmd) => cmd.execute().await,
            Commands::Detect(cmd) => cmd.execute().await,
            Commands::Ir(cmd) => cmd.execute().await,
            Commands::Vuln(cmd) => cmd.execute().await,
            Commands::Source(cmd) => cmd.execute().await,
            Commands::Playbook(cmd) => cmd.execute().await,
            Commands::Config(cmd) => cmd.execute().await,
            Commands::Doctor(cmd) => cmd.execute().await,
        }
    }
}
