# Contributing to Vigil Bot

Thanks for your interest in contributing to Vigil Bot! This guide will help you get started.

## Development Setup

1. **Install Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**
   ```bash
   git clone https://github.com/yasir23/vigil
   cd vigil
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Run the CLI**
   ```bash
   cargo run -- --help
   cargo run -- doctor
   ```

## Project Structure

```
vigil/
├── src/
│   ├── main.rs              # Entry point
│   ├── commands/            # CLI command implementations
│   │   ├── mod.rs           # Command routing
│   │   ├── hunt.rs          # Threat hunting
│   │   ├── detect.rs        # Detection rule generation
│   │   ├── ir.rs            # Incident response
│   │   ├── vuln.rs          # Vulnerability analysis
│   │   ├── source.rs        # Source management
│   │   ├── playbook.rs      # Playbook system
│   │   ├── config.rs        # Configuration
│   │   ├── doctor.rs        # Health checks
│   │   └── onboard.rs       # Onboarding wizard
│   ├── config/              # Configuration system
│   ├── agent/               # Agent engine (planned)
│   ├── sources/             # Data source integrations (planned)
│   ├── output/              # Report generation (planned)
│   └── utils/               # Utilities (planned)
├── Cargo.toml
└── README.md
```

## Adding a New Command

1. Create a new file in `src/commands/` (e.g., `mycommand.rs`)
2. Define the command structure:
   ```rust
   use anyhow::Result;
   use clap::Args;

   #[derive(Args)]
   pub struct MyCommand {
       #[arg(long)]
       my_arg: String,
   }

   impl MyCommand {
       pub async fn execute(self) -> Result<()> {
           // Implementation here
           Ok(())
       }
   }
   ```

3. Add the module and command to `src/commands/mod.rs`
4. Test your command: `cargo run -- mycommand --my-arg test`

## Adding a Source Integration

Source integrations should be added in `src/sources/` (to be created). Each source should implement:

- Connection/authentication
- Query interface
- Result parsing
- Error handling

## Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Run clippy: `cargo clippy`
- Add documentation for public APIs
- Write tests for new functionality

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Areas Needing Help

High-priority areas for contribution:

1. **LLM Client Integration**
   - Anthropic Claude API
   - OpenAI API
   - Ollama local models

2. **Source Integrations**
   - Elasticsearch
   - Splunk
   - Loki
   - VirusTotal
   - Shodan
   - MISP

3. **Detection Rule Generation**
   - Sigma rule templates
   - YARA rule generation
   - KQL/SPL conversion

4. **Playbook System**
   - YAML parser
   - Execution engine
   - Community playbooks

5. **Documentation**
   - Usage examples
   - Integration guides
   - Playbook templates

## Questions?

- Open an issue for bugs or feature requests
- Join our Discord: https://discord.gg/nayaflow
- Email: yasir@nayaflow.com

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
