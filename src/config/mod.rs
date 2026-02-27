use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VigilConfig {
    #[serde(default)]
    pub llm: LlmConfig,

    #[serde(default)]
    pub agent: AgentConfig,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    pub sources: SourcesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmConfig {
    #[serde(default = "default_provider")]
    pub provider: String,

    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default)]
    pub api_key_env: Option<String>,

    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: default_model(),
            api_key_env: None,
            temperature: default_temperature(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConfig {
    #[serde(default = "default_max_iterations")]
    pub max_iterations: usize,

    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_iterations: default_max_iterations(),
            timeout_secs: default_timeout_secs(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    #[serde(default = "default_output_format")]
    pub default_format: String,

    #[serde(default)]
    pub report_dir: Option<String>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_format: default_output_format(),
            report_dir: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SourcesConfig {
    // Will be populated with configured sources
}

// Default value functions
fn default_provider() -> String {
    "anthropic".to_string()
}

fn default_model() -> String {
    "claude-opus-4-5".to_string()
}

fn default_temperature() -> f32 {
    0.1
}

fn default_max_iterations() -> usize {
    10
}

fn default_timeout_secs() -> u64 {
    120
}

fn default_output_format() -> String {
    "markdown".to_string()
}

impl VigilConfig {
    /// Get the path to the configuration file
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;

        let vigil_config_dir = config_dir.join("vigil");
        fs::create_dir_all(&vigil_config_dir)?;

        Ok(vigil_config_dir.join("config.toml"))
    }

    /// Load configuration from file, or create default if not exists
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            let config: VigilConfig = toml::from_str(&contents)?;
            Ok(config)
        } else {
            // Create default config
            let config = VigilConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        Ok(())
    }

    /// Set a configuration value by key path (e.g., "llm.provider")
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        let parts: Vec<&str> = key.split('.').collect();

        match parts.as_slice() {
            ["llm", "provider"] => self.llm.provider = value.to_string(),
            ["llm", "model"] => self.llm.model = value.to_string(),
            ["llm", "api_key_env"] => self.llm.api_key_env = Some(value.to_string()),
            ["llm", "temperature"] => {
                self.llm.temperature = value.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid temperature value"))?;
            }
            ["agent", "max_iterations"] => {
                self.agent.max_iterations = value.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid max_iterations value"))?;
            }
            ["agent", "timeout_secs"] => {
                self.agent.timeout_secs = value.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid timeout_secs value"))?;
            }
            ["output", "default_format"] => self.output.default_format = value.to_string(),
            ["output", "report_dir"] => self.output.report_dir = Some(value.to_string()),
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        }

        Ok(())
    }

    /// Get a configuration value by key path
    pub fn get(&self, key: &str) -> Result<String> {
        let parts: Vec<&str> = key.split('.').collect();

        let value = match parts.as_slice() {
            ["llm", "provider"] => self.llm.provider.clone(),
            ["llm", "model"] => self.llm.model.clone(),
            ["llm", "api_key_env"] => self.llm.api_key_env.clone().unwrap_or_default(),
            ["llm", "temperature"] => self.llm.temperature.to_string(),
            ["agent", "max_iterations"] => self.agent.max_iterations.to_string(),
            ["agent", "timeout_secs"] => self.agent.timeout_secs.to_string(),
            ["output", "default_format"] => self.output.default_format.clone(),
            ["output", "report_dir"] => self.output.report_dir.clone().unwrap_or_default(),
            _ => anyhow::bail!("Unknown configuration key: {}", key),
        };

        Ok(value)
    }
}
