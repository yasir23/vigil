use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct OnboardCommand {}

impl OnboardCommand {
    pub async fn execute(self) -> Result<()> {
        println!("🦅 Welcome to Vigil Bot!");
        println!("\nThe onboarding wizard will guide you through:");
        println!("  • Setting up your LLM provider");
        println!("  • Configuring log sources");
        println!("  • Adding threat intel feeds");
        println!("  • Running your first hunt");
        println!("\n⚠️  Onboarding wizard not yet implemented");
        Ok(())
    }
}
