use anyhow::Result;
use clap::{Args, Subcommand};
use std::process::Command;

use crate::config::VigilConfig;

#[derive(Args)]
pub struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Set a configuration value
    Set(SetArgs),

    /// Get a configuration value
    Get(GetArgs),

    /// List all configuration values
    List,

    /// Edit configuration file in default editor
    Edit,

    /// Show configuration file path
    Path,
}

#[derive(Args)]
struct SetArgs {
    /// Configuration key (e.g., llm.provider)
    key: String,

    /// Configuration value
    value: String,
}

#[derive(Args)]
struct GetArgs {
    /// Configuration key
    key: String,
}

impl ConfigCommand {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            ConfigSubcommand::Set(args) => {
                let mut config = VigilConfig::load()?;
                config.set(&args.key, &args.value)?;
                config.save()?;

                println!("⚙️  Configuration Updated");
                println!("\n{} = {}", args.key, args.value);
                Ok(())
            }
            ConfigSubcommand::Get(args) => {
                let config = VigilConfig::load()?;
                let value = config.get(&args.key)?;

                println!("⚙️  Configuration Value");
                println!("\n{} = {}", args.key, value);
                Ok(())
            }
            ConfigSubcommand::List => {
                let config = VigilConfig::load()?;
                let toml_str = toml::to_string_pretty(&config)?;

                println!("⚙️  Configuration");
                println!("\n{}", toml_str);
                Ok(())
            }
            ConfigSubcommand::Edit => {
                let config_path = VigilConfig::config_path()?;

                // Ensure config exists
                let _ = VigilConfig::load()?;

                // Try to open in editor
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
                    if cfg!(target_os = "windows") {
                        "notepad".to_string()
                    } else {
                        "nano".to_string()
                    }
                });

                println!("⚙️  Opening Configuration in {}", editor);

                Command::new(editor)
                    .arg(&config_path)
                    .status()?;

                Ok(())
            }
            ConfigSubcommand::Path => {
                let config_path = VigilConfig::config_path()?;
                println!("📁 Configuration Path");
                println!("\n{}", config_path.display());
                Ok(())
            }
        }
    }
}
