<div align="center">

```
██╗   ██╗██╗ ██████╗ ██╗██╗     
██║   ██║██║██╔════╝ ██║██║     
██║   ██║██║██║  ███╗██║██║     
╚██╗ ██╔╝██║██║   ██║██║██║     
 ╚████╔╝ ██║╚██████╔╝██║███████╗
  ╚═══╝  ╚═╝ ╚═════╝ ╚═╝╚══════╝
```

**Autonomous threat intelligence, built in Rust.**

[![Crates.io](https://img.shields.io/crates/v/vigil?style=flat-square&color=dc2626)](https://crates.io/crates/vigil)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/yasir23/vigil/ci.yml?style=flat-square)](https://github.com/yasir23/vigil/actions)
[![Stars](https://img.shields.io/github/stars/yasir23/vigil?style=flat-square&color=f59e0b)](https://github.com/yasir23/vigil/stargazers)
[![Discord](https://img.shields.io/discord/000000000?style=flat-square&color=5865f2&label=discord)](https://discord.gg/nayaflow)

[Install](#installation) · [Quick Start](#quick-start) · [Use Cases](#use-cases) · [Docs](#documentation) · [Contributing](#contributing)

</div>

---

`vigil` is an open-source AI agent CLI for cybersecurity professionals. It autonomously hunts threats, generates detection rules, investigates incidents, and pivots on indicators — all from your terminal, with no cloud dependency required.

**One binary. No Python. No vendor lock-in. Runs anywhere.**

```
$ vigil hunt --ioc "185.220.101.47" --sources elastic,virustotal,shodan

⠿ Querying Elasticsearch...       ████████████░░░ 12 events found
⠿ Pivoting on related IOCs...      ████████████████ 3 related IPs
⠿ Enriching via VirusTotal...      ████████████████ Malicious (67/90)
⠿ Correlating MITRE ATT&CK...      ████████████████ T1071.001, T1090

╔══════════════════════════════════════════════════╗
║  THREAT SUMMARY                                  ║
╠══════════════════════════════════════════════════╣
║  IOC         185.220.101.47                      ║
║  Verdict     MALICIOUS — High Confidence         ║
║  Actor       Likely Tor exit node / C2 relay     ║
║  Technique   T1071.001 — Application Layer Proto ║
║  First Seen  2024-11-03 · Last Seen 2025-01-17   ║
║  Related     185.220.101.48, 185.220.101.51      ║
╚══════════════════════════════════════════════════╝

→ Full report saved to ./reports/hunt-20250127-143201.md
```

---

## Why vigil?

Most "AI security tools" are ChatGPT wrappers with a dashboard. `vigil` is different:

| | vigil | Python-based tools | Vendor platforms |
|---|---|---|---|
| Single binary deploy | ✅ | ❌ | ❌ |
| Air-gapped / offline support | ✅ | ⚠️ | ❌ |
| Local LLM (Ollama) support | ✅ | ⚠️ | ❌ |
| Pipeline / CI-CD friendly | ✅ | ⚠️ | ❌ |
| Open source & auditable | ✅ | ✅ | ❌ |
| Memory safe (Rust) | ✅ | ❌ | unknown |
| No vendor lock-in | ✅ | ✅ | ❌ |

---

## Installation

### Cargo (recommended)

```bash
cargo install vigil
```

### Homebrew

```bash
brew tap yasir23/vigil
brew install vigil
```

### Pre-built binaries

Download from [Releases](https://github.com/yasir23/vigil/releases) for Linux, macOS, and Windows.

```bash
# Linux x86_64
curl -sSL https://github.com/yasir23/vigil/releases/latest/download/vigil-linux-x86_64 \
  -o /usr/local/bin/vigil && chmod +x /usr/local/bin/vigil
```

### Verify installation

```bash
vigil --version
# vigil 0.1.0
```

---

## Quick Start

### 1. Configure your LLM

```bash
# OpenAI
vigil config set llm.provider openai
vigil config set llm.api_key sk-...

# Anthropic Claude
vigil config set llm.provider anthropic
vigil config set llm.api_key sk-ant-...

# Local (Ollama) — fully offline, no data leaves your machine
vigil config set llm.provider ollama
vigil config set llm.model llama3.1:70b
```

### 2. Connect your sources

```bash
# Elasticsearch / OpenSearch
vigil source add elastic --url http://localhost:9200 --index logs-*

# Splunk
vigil source add splunk --url https://splunk:8089 --token your-token

# Threat intel
vigil source add virustotal --api-key your-key
vigil source add shodan --api-key your-key
```

### 3. Run your first hunt

```bash
vigil hunt --ioc "evil.example.com"
```

---

## Use Cases

### 🔴 Threat Hunting

Autonomously investigate indicators of compromise across your log sources, pivot on related artifacts, and generate structured reports.

```bash
# Hunt by IOC
vigil hunt --ioc "185.220.101.47"
vigil hunt --ioc "evil.example.com" --sources elastic,splunk
vigil hunt --ioc "d41d8cd98f00b204e9800998ecf8427e" --type md5

# Hunt by hypothesis
vigil hunt --hypothesis "lateral movement via RDP from workstations"

# Hunt from a file of IOCs
vigil hunt --ioc-file ./iocs.txt --parallel 5

# Output options
vigil hunt --ioc "..." --output json | jq .verdict
vigil hunt --ioc "..." --report pdf --out ./reports/
```

### 🔴 Detection Rule Generation

Describe a threat behavior. Get production-ready detection rules.

```bash
# Generate a Sigma rule
vigil detect generate \
  --behavior "PowerShell downloading and executing base64-encoded payload" \
  --format sigma

# Generate from a MITRE technique
vigil detect generate --technique T1059.001 --format yara

# Generate KQL for Microsoft Sentinel
vigil detect generate \
  --behavior "Suspicious scheduled task creation" \
  --format kql --platform sentinel

# Test a rule against your logs
vigil detect test ./rules/my-rule.yml --source elastic --dry-run
```

### 🟠 Incident Response

Feed an alert or artifact. Get a full investigation timeline with MITRE mapping.

```bash
# Investigate an alert
vigil ir investigate --alert-id "ALERT-2025-0042" --source splunk

# Analyze a suspicious file
vigil ir analyze --file ./suspicious.exe

# Build an incident timeline
vigil ir timeline --start "2025-01-27T00:00:00Z" --entity "workstation-42"

# Generate IR report
vigil ir report --incident ./incidents/inc-001.json --format markdown
```

### 🟡 Vulnerability & Attack Surface Analysis

```bash
# Analyze a CVE against your environment
vigil vuln analyze --cve CVE-2024-12345 --inventory ./assets.json

# Scan and assess
vigil vuln scan --target 10.0.0.0/24 --assess

# Check exposure
vigil vuln exposure --product "nginx" --version "1.18.0"
```

### 🟡 Autonomous Recon (Red Team)

```bash
# Recon pipeline
vigil recon --target example.com --passive

# Chain tools automatically
vigil recon --target example.com \
  --tools nmap,nuclei,ffuf \
  --report ./pentest-report.md
```

---

## Playbooks

Playbooks are YAML-defined agent workflows you can share and reuse.

```yaml
# playbooks/ransomware-hunt.yml
name: Ransomware Indicator Hunt
description: Hunt for common ransomware TTPs across endpoints and network logs
author: nayaflow
tags: [ransomware, hunting, T1486]

steps:
  - name: Hunt file encryption indicators
    action: hunt
    params:
      behavior: "mass file modification with extension changes"
      sources: [elastic, splunk]

  - name: Check for known ransomware IOCs
    action: hunt
    params:
      ioc_list: ./iocs/ransomware-known.txt

  - name: Generate detection rules
    action: detect.generate
    params:
      technique: T1486
      formats: [sigma, yara]

  - name: Produce report
    action: report
    params:
      format: markdown
      mitre_mapping: true
```

```bash
# Run a playbook
vigil playbook run ./playbooks/ransomware-hunt.yml

# List community playbooks
vigil playbook list

# Pull a community playbook
vigil playbook pull ransomware-hunt
```

Community playbooks live in [`/playbooks`](./playbooks) — contributions welcome.

---

## Local LLM / Air-Gapped Mode

`vigil` works fully offline with [Ollama](https://ollama.ai). No data leaves your environment.

```bash
# Install Ollama and pull a model
ollama pull llama3.1:70b

# Configure vigil to use it
vigil config set llm.provider ollama
vigil config set llm.model llama3.1:70b

# Run normally — everything stays local
vigil hunt --ioc "185.220.101.47"
```

This makes `vigil` viable in:
- Air-gapped SOC environments
- Government and defense networks
- Compliance-restricted organizations (HIPAA, PCI-DSS, FedRAMP)
- Any environment where sending log data to external APIs is prohibited

---

## Integrations

### Log Sources
| Source | Status |
|---|---|
| Elasticsearch / OpenSearch | ✅ Stable |
| Splunk | ✅ Stable |
| Loki / Grafana | ✅ Stable |
| Microsoft Sentinel (KQL) | 🚧 Beta |
| Chronicle | 🚧 Beta |
| AWS CloudTrail | 🔜 Planned |
| Azure Monitor | 🔜 Planned |

### Threat Intelligence
| Source | Status |
|---|---|
| VirusTotal | ✅ Stable |
| Shodan | ✅ Stable |
| AbuseIPDB | ✅ Stable |
| MISP | ✅ Stable |
| AlienVault OTX | 🚧 Beta |
| Recorded Future | 🔜 Planned |
| MITRE ATT&CK | ✅ Stable |

### Output Formats
- Markdown reports
- JSON / NDJSON (pipeline-friendly)
- PDF (via headless rendering)
- STIX 2.1
- Sigma, YARA, KQL, SPL rules

---

## Pipeline Integration

`vigil` is designed for automation. Use it in CI/CD, SOAR playbooks, or cron jobs.

```bash
# Exit codes: 0 = clean, 1 = suspicious, 2 = malicious, 3 = error
vigil hunt --ioc "$ALERT_IP" --output json --silent
echo "Exit: $?"

# Use in a shell pipeline
cat daily-iocs.txt | xargs -I{} vigil hunt --ioc {} --output json >> results.ndjson

# GitHub Actions example
- name: Scan PR for suspicious domains
  run: |
    vigil hunt --ioc-file extracted-domains.txt --output json \
      | jq 'select(.verdict == "MALICIOUS")' \
      | tee malicious-findings.json
```

---

## Configuration

Full config reference at `~/.config/vigil/config.toml`:

```toml
[llm]
provider = "anthropic"         # openai | anthropic | ollama | azure
model = "claude-3-5-sonnet"
api_key_env = "ANTHROPIC_API_KEY"
max_tokens = 4096
temperature = 0.1              # Low temp for consistent, factual output

[agent]
max_iterations = 10            # Max reasoning steps per task
timeout_secs = 120
parallel_tools = 3

[output]
default_format = "markdown"
report_dir = "~/.vigil/reports"
color = true

[sources.elastic]
url = "http://localhost:9200"
index = "logs-*"
username_env = "ELASTIC_USER"
password_env = "ELASTIC_PASS"

[sources.virustotal]
api_key_env = "VT_API_KEY"
```

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    CLI Layer (clap)                  │
│         hunt | detect | ir | vuln | recon            │
└────────────────────────┬────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────┐
│                  Agent Engine (Rust)                 │
│     Task planning → Tool selection → Reasoning loop  │
└──────┬──────────────┬──────────────────┬────────────┘
       │              │                  │
┌──────▼──────┐ ┌─────▼──────┐ ┌────────▼───────┐
│  LLM Client │ │Tool Registry│ │ Memory / State │
│  OpenAI     │ │  log query  │ │  sled (local)  │
│  Anthropic  │ │  threat intel│ │  session ctx  │
│  Ollama     │ │  rule gen   │ └────────────────┘
└─────────────┘ │  file anal  │
                └─────┬───────┘
                      │
       ┌──────────────┴──────────────┐
       │         Data Sources        │
       │  Elastic · Splunk · Loki    │
       │  VT · Shodan · MISP · OTX   │
       └─────────────────────────────┘
```

---

## Documentation

- [Getting Started Guide](./docs/getting-started.md)
- [Configuration Reference](./docs/configuration.md)
- [Tool Integrations](./docs/integrations.md)
- [Playbook Authoring](./docs/playbooks.md)
- [API / JSON Output](./docs/api-output.md)
- [Air-Gapped Deployment](./docs/air-gapped.md)
- [Contributing Guide](./CONTRIBUTING.md)

---

## Contributing

Contributions are what make this tool worth using. All skill levels welcome.

**High-value contributions right now:**
- New log source integrations (AWS CloudTrail, Azure Monitor, Chronicle)
- Additional threat intel source connectors
- Community playbooks for common threat scenarios
- Testing and bug reports

```bash
git clone https://github.com/yasir23/vigil
cd vigil
cargo build
cargo test

# Run against a local test environment
cargo run -- hunt --ioc "1.2.3.4" --mock
```

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full guide. Join us on [Discord](https://discord.gg/nayaflow) to discuss ideas before building.

---

## Security

`vigil` handles sensitive security data. Please report vulnerabilities responsibly.

**Do not** open public issues for security vulnerabilities. Instead, email `yasir@nayaflow.com` or use [GitHub's private vulnerability reporting](https://github.com/yasir23/vigil/security/advisories/new).

See [SECURITY.md](./SECURITY.md) for our full disclosure policy.

---

## Roadmap

- [x] Core agent engine
- [x] IOC hunting with Elastic, Splunk, Loki
- [x] VirusTotal, Shodan, AbuseIPDB enrichment
- [x] Sigma, YARA, KQL rule generation
- [x] Ollama / local LLM support
- [x] Playbook system
- [ ] Microsoft Sentinel integration
- [ ] STIX 2.1 report output
- [ ] Chronicle integration
- [ ] AWS CloudTrail source
- [ ] Web UI (optional companion)
- [ ] MCP server mode (use as a tool from other agents)
- [ ] VSCode extension

Track progress and vote on features in [GitHub Discussions](https://github.com/yasir23/vigil/discussions).

---

## License

MIT © 2025 [Nayaflow](https://nayaflow.com)

This software is provided for **defensive security purposes**. Users are responsible for ensuring their use complies with applicable laws and organizational policies.

---

<div align="center">

**Built by the security community, for the security community.**

[⭐ Star on GitHub](https://github.com/yasir23/vigil) · [💬 Join Discord](https://discord.gg/nayaflow) · [🌐 nayaflow.com](https://nayaflow.com)

</div>
