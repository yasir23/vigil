# vigil bot 🦅

Your own personal threat intelligence agent. Any OS. Any Platform. The security-first way. 🛡️

[![Crates.io](https://img.shields.io/crates/v/vigil?style=flat-square&color=dc2626)](https://crates.io/crates/vigil)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/yasir23/vigil/ci.yml?style=flat-square)](https://github.com/yasir23/vigil/actions)
[![Stars](https://img.shields.io/github/stars/yasir23/vigil?style=flat-square&color=f59e0b)](https://github.com/yasir23/vigil/stargazers)

---

Vigil Bot is an autonomous threat intelligence agent you run on your own machine. It hunts threats, generates detection rules, investigates incidents, and pivots on indicators of compromise — all from your terminal, with no cloud dependency required.

The CLI is just the control plane — the product is the agent. If you want a fast, local-first, always-on threat hunter that feels like a senior SOC analyst in your terminal, this is it.

Single binary. No Python. No vendor lock-in. Runs anywhere, including air-gapped environments.

---

## Getting started

**Cargo**
```bash
cargo install vigil
```

Then run the onboarding wizard:
```bash
vigil onboard
```

The wizard walks you through everything: LLM provider, log sources, threat intel feeds, and your first hunt.

---

## Building from source

```bash
git clone https://github.com/yasir23/vigil
cd vigil
cargo build --release
./target/release/vigil --version
```

---

## What vigil bot does

Vigil Bot is a local-first AI agent for cybersecurity workflows. It reasons over your data, uses your tools, and works the way a real analyst works — not just answering questions, but taking autonomous action.

- **Threat hunting** — investigate IOCs across your log sources, pivot on related indicators, map to MITRE ATT&CK, generate reports
- **Detection engineering** — describe a behavior in plain English, get production-ready Sigma, YARA, or KQL rules back
- **Incident response** — feed an alert or artifact, get a full investigation timeline and draft IR report
- **Vulnerability analysis** — analyze CVEs against your environment, check exposure, get remediation steps
- **Playbooks** — YAML-defined agent workflows you can save, share, and reuse across your team

---

## Quick demo

```
$ vigil hunt --ioc "185.220.101.47"

⠿ Querying Elasticsearch...        12 events found
⠿ Pivoting on related IOCs...       3 related IPs
⠿ Enriching via VirusTotal...       Malicious (67/90)
⠿ Correlating MITRE ATT&CK...       T1071.001, T1090

╔══════════════════════════════════════════════════╗
║  THREAT SUMMARY                                  ║
╠══════════════════════════════════════════════════╣
║  IOC         185.220.101.47                      ║
║  Verdict     MALICIOUS — High Confidence         ║
║  Actor       Tor exit node / C2 relay            ║
║  Technique   T1071.001 · T1090                   ║
║  Related     185.220.101.48, 185.220.101.51      ║
╚══════════════════════════════════════════════════╝

→ Report saved: ./reports/hunt-20250127-143201.md
```

---

## LLM setup

Vigil Bot works with any OpenAI-compatible endpoint.

```bash
# Anthropic Claude (recommended)
vigil config set llm.provider anthropic
vigil config set llm.api_key sk-ant-...

# OpenAI
vigil config set llm.provider openai
vigil config set llm.api_key sk-...

# Local / fully offline (Ollama)
vigil config set llm.provider ollama
vigil config set llm.model llama3.1:70b
```

---

## Usage

### Threat hunting

```bash
vigil hunt --ioc "185.220.101.47"
vigil hunt --ioc "evil.example.com" --sources elastic,splunk
vigil hunt --hypothesis "lateral movement via RDP from workstations to servers"
```

### Detection rule generation

```bash
vigil detect generate --behavior "PowerShell downloading base64 payload" --format sigma
vigil detect generate --technique T1059.001 --format yara
```

### Incident response

```bash
vigil ir investigate --alert-id "ALERT-2025-0042" --source splunk
vigil ir analyze --file ./suspicious.exe
```

### Health check

```bash
vigil doctor
```

Run `vigil doctor` to surface misconfigured sources, missing API keys, and connectivity issues before they affect a real investigation.

---

## Configuration

Config lives at `~/.config/vigil/config.toml` (macOS/Linux) or `%APPDATA%\vigil\config.toml` (Windows).

Run `vigil config edit` to open it, or `vigil config path` to see the location.

```toml
[llm]
provider    = "anthropic"
model       = "claude-opus-4-5"
temperature = 0.1

[agent]
max_iterations = 10
timeout_secs   = 120

[output]
default_format = "markdown"
```

---

## Current status

🚧 **Early development** - Core infrastructure is in place. Feature implementation is ongoing.

### ✅ Implemented
- CLI framework with all commands
- Configuration system
- Project structure

### 🚧 In Progress
- LLM client integration
- Threat hunting engine
- Detection rule generation
- Source integrations (Elasticsearch, Splunk, VirusTotal, etc.)

### 📋 Planned
- Playbook system
- Incident response workflows
- Vulnerability analysis
- Community playbooks

---

## Contributing

All contributions welcome — new source integrations, playbooks, bug reports, and docs.

```bash
git clone https://github.com/yasir23/vigil
cd vigil
cargo build
cargo test
```

---

## License

MIT © 2025 [Nayaflow](https://nayaflow.com)

Built for defensive security. Users are responsible for compliance with applicable laws and policies.
