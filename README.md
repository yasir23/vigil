# vigil bot 🦅

![Whisk_a8b2ecacf3b94b7969940979aa5b931edr](https://github.com/user-attachments/assets/a44e2c24-1f6f-440a-b5c9-f0d965dc4edc)


Your own personal threat intelligence agent. Any OS. Any Platform. The security-first way. 🛡️

[![Crates.io](https://img.shields.io/crates/v/vigil?style=flat-square&color=dc2626)](https://crates.io/crates/vigil)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/yasir23/vigil/ci.yml?style=flat-square)](https://github.com/yasir23/vigil/actions)
[![Stars](https://img.shields.io/github/stars/yasir23/vigil?style=flat-square&color=f59e0b)](https://github.com/yasir23/vigil/stargazers)
[![Discord](https://img.shields.io/discord/000000000?style=flat-square&color=5865f2&label=discord)](https://discord.gg/nayaflow)

[Website](https://nayaflow.com) · [Docs](https://docs.nayaflow.com) · [Getting Started](#getting-started) · [Discord](https://discord.gg/nayaflow)

---

Vigil Bot is an autonomous threat intelligence agent you run on your own machine. It hunts threats, generates detection rules, investigates incidents, and pivots on indicators of compromise — all from your terminal, with no cloud dependency required.

The CLI is just the control plane — the product is the agent. If you want a fast, local-first, always-on threat hunter that feels like a senior SOC analyst in your terminal, this is it.

Single binary. No Python. No vendor lock-in. Runs anywhere, including air-gapped environments.

Preferred setup: run the onboarding wizard (`vigil onboard`) in your terminal. The wizard guides you step by step through setting up your LLM, log sources, and threat intel feeds.

---

## Getting started

**macOS / Linux**
```bash
curl -fsSL https://nayaflow.com/install.sh | bash
```

**Windows (PowerShell)**
```powershell
iwr -useb https://nayaflow.com/install.ps1 | iex
```

**Cargo**
```bash
cargo install vigil
```

**Homebrew**
```bash
brew tap yasir23/vigil && brew install vigil
```

Then run the onboarding wizard:
```bash
vigil onboard
```

The wizard walks you through everything: LLM provider, log sources, threat intel feeds, and your first hunt.

---

## What vigil bot does

Vigil Bot is a local-first AI agent for cybersecurity workflows. It reasons over your data, uses your tools, and works the way a real analyst works — not just answering questions, but taking autonomous action.

- **Threat hunting** — investigate IOCs across your log sources, pivot on related indicators, map to MITRE ATT&CK, generate reports
- **Detection engineering** — describe a behavior in plain English, get production-ready Sigma, YARA, or KQL rules back
- **Incident response** — feed an alert or artifact, get a full investigation timeline and draft IR report
- **Vulnerability analysis** — analyze CVEs against your environment, check exposure, get remediation steps
- **Recon & red team** — passive recon pipelines, tool chaining, automated pentest report drafts
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
# Anthropic Claude (recommended — best reasoning, strongest prompt-injection resistance)
vigil config set llm.provider anthropic
vigil config set llm.api_key sk-ant-...

# OpenAI
vigil config set llm.provider openai
vigil config set llm.api_key sk-...

# Local / fully offline (Ollama)
vigil config set llm.provider ollama
vigil config set llm.model llama3.1:70b
```

**Air-gapped / offline mode:** Ollama support means no data ever leaves your machine. Vigil Bot runs in air-gapped SOCs, government networks, and compliance-restricted environments (FedRAMP, PCI-DSS, HIPAA) out of the box.

---

## Connecting sources

```bash
# Log sources
vigil source add elastic --url http://localhost:9200 --index logs-*
vigil source add splunk  --url https://splunk:8089 --token YOUR_TOKEN
vigil source add loki    --url http://localhost:3100

# Threat intel
vigil source add virustotal --api-key YOUR_KEY
vigil source add shodan     --api-key YOUR_KEY
vigil source add abuseipdb  --api-key YOUR_KEY
vigil source add misp       --url https://misp.internal --key YOUR_KEY
```

Run `vigil source list` to see all connected sources and their health status.

---

## Usage

### Threat hunting

```bash
vigil hunt --ioc "185.220.101.47"
vigil hunt --ioc "evil.example.com" --sources elastic,splunk
vigil hunt --hypothesis "lateral movement via RDP from workstations to servers"
vigil hunt --ioc-file ./iocs.txt --parallel 5 --output json
```

### Detection rule generation

```bash
vigil detect generate --behavior "PowerShell downloading base64 payload" --format sigma
vigil detect generate --technique T1059.001 --format yara
vigil detect generate --behavior "Suspicious scheduled task creation" --format kql --platform sentinel
vigil detect test ./rules/my-rule.yml --source elastic --dry-run
```

### Incident response

```bash
vigil ir investigate --alert-id "ALERT-2025-0042" --source splunk
vigil ir analyze --file ./suspicious.exe
vigil ir timeline --start "2025-01-27T00:00:00Z" --entity "workstation-42"
vigil ir report --incident ./incidents/inc-001.json --format markdown
```

### Vulnerability analysis

```bash
vigil vuln analyze --cve CVE-2024-12345 --inventory ./assets.json
vigil vuln exposure --product nginx --version 1.18.0
```

### Health check

```bash
vigil doctor
```

Run `vigil doctor` to surface misconfigured sources, missing API keys, and connectivity issues before they affect a real investigation.

---

## Playbooks

Playbooks are YAML-defined agent workflows. Save, share, and reuse common investigation patterns across your team.

```yaml
# playbooks/ransomware-hunt.yml
name: Ransomware Indicator Hunt
description: Hunt for common ransomware TTPs across endpoints and network logs
author: nayaflow
tags: [ransomware, T1486]

steps:
  - name: Hunt file encryption indicators
    action: hunt
    params:
      behavior: "mass file modification with extension changes"

  - name: Check known ransomware IOCs
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
vigil playbook run ./playbooks/ransomware-hunt.yml
vigil playbook list          # browse community playbooks
vigil playbook pull ransomware-hunt
```

Community playbooks live in [`/playbooks`](./playbooks). Contributions welcome.

---

## Pipeline / automation

Exit codes are machine-readable: `0` clean · `1` suspicious · `2` malicious · `3` error

```bash
# JSON output for scripting
vigil hunt --ioc "$ALERT_IP" --output json --silent | jq .verdict

# Bulk scan
cat daily-iocs.txt | xargs -I{} vigil hunt --ioc {} --output json >> results.ndjson

# GitHub Actions
- name: Scan extracted domains
  run: |
    vigil hunt --ioc-file domains.txt --output json \
      | jq 'select(.verdict == "MALICIOUS")' \
      | tee malicious.json
```

---

## Integrations

**Log sources** — Elasticsearch · OpenSearch · Splunk · Loki · Microsoft Sentinel (beta) · Chronicle (beta) · AWS CloudTrail (planned) · Azure Monitor (planned)

**Threat intel** — VirusTotal · Shodan · AbuseIPDB · MISP · AlienVault OTX · MITRE ATT&CK

**Output** — Markdown · JSON/NDJSON · PDF · STIX 2.1 · Sigma · YARA · KQL · SPL

---

## Configuration

Config lives at `~/.config/vigil/config.toml`. Run `vigil config edit` to open it.

```toml
[llm]
provider    = "anthropic"
model       = "claude-opus-4-6"
api_key_env = "ANTHROPIC_API_KEY"
temperature = 0.1

[agent]
max_iterations = 10
timeout_secs   = 120

[output]
default_format = "markdown"
report_dir     = "~/.vigil/reports"

[sources.elastic]
url   = "http://localhost:9200"
index = "logs-*"
```

---

## Built on

Vigil Bot is powered by three best-in-class Rust agent frameworks, each chosen for a specific role.

### [Agentor](https://github.com/agentor-ai/agentor) — Investigative IR & sandboxing

Handles the high-risk, file-touching parts of vigil — malware analysis, artifact inspection, and sandboxed execution. WASM-based isolation means a malicious file can't escape the agent's reasoning loop and compromise your machine.

Best for: `vigil ir analyze`, `vigil recon`

### [GraphBit](https://github.com/graphbit) — Deterministic SOC orchestration

Runs the multi-step pivoting logic. When vigil hunts an IOC across Elasticsearch, VirusTotal, and Shodan in parallel, GraphBit's graph-based orchestration engine ensures the results are consistent and repeatable every run — no hallucinated pivots, no infinite loops. Also handles built-in secret management for your API keys (Shodan, MISP, VT).

Best for: `vigil hunt`, `vigil playbook run`

### [Rig](https://github.com/0xPlaygrounds/rig) — Modular LLM chains

The lightweight backbone for `vigil detect` and `vigil vuln`. Rig lets us define each command as a composable LLM chain without bloating the binary — the same philosophy as vigil itself: fast, modular, single binary.

Best for: `vigil detect`, `vigil vuln`

| | Agentor | GraphBit | Rig |
|---|---|---|---|
| Primary focus | Security & sandboxing | Reliability & determinism | Modularity & performance |
| Sandboxing | WASM-based isolation | Resource-type safety | Basic Rust safety |
| Workflow | Task-based agents | Graph-based orchestration | Modular LLM chains |
| Best use case | Investigative IR tools | High-scale SOC orchestration | General CLI-based agents |

---

## Architecture

```
CLI (clap)
  └── Agent engine (Rust)
        ├── LLM client       — Anthropic · OpenAI · Ollama
        ├── Tool registry    — log query · threat intel · rule gen · file analysis
        └── Memory / state   — local (sled) · session context
              └── Data sources
                    Elastic · Splunk · Loki · VT · Shodan · MISP · OTX
```

---

## Contributing

All contributions welcome — new source integrations, playbooks, bug reports, and docs.

```bash
git clone https://github.com/yasir23/vigil
cd vigil
cargo build
cargo test
cargo run -- hunt --ioc "1.2.3.4" --mock   # no live sources needed
```

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines. Join [Discord](https://discord.gg/nayaflow) before starting large changes. AI/vibe-coded PRs welcome! 🤖

High-value areas right now: AWS CloudTrail · Azure Monitor · Chronicle · community playbooks · Windows CI

---

## Security

Do not open public issues for vulnerabilities. Email [yasir@nayaflow.com](mailto:yasir@nayaflow.com) or use [GitHub private reporting](https://github.com/yasir23/vigil/security/advisories/new).

Run `vigil doctor` to surface risky or misconfigured setups before exposing anything. Review [SECURITY.md](./SECURITY.md) before exposing vigil bot to a network.

---

## Roadmap

- [x] Core agent engine
- [x] Elastic · Splunk · Loki log sources
- [x] VirusTotal · Shodan · AbuseIPDB · MISP enrichment
- [x] Sigma · YARA · KQL rule generation
- [x] Ollama / local LLM support
- [x] Playbook system
- [x] `vigil doctor` health checks
- [ ] Microsoft Sentinel (stable)
- [ ] Chronicle integration
- [ ] AWS CloudTrail source
- [ ] Azure Monitor source
- [ ] STIX 2.1 output
- [ ] MCP server mode
- [ ] VSCode extension
- [ ] Web UI companion

Vote on features in [GitHub Discussions](https://github.com/yasir23/vigil/discussions).

---

## License

MIT © 2025 [Nayaflow](https://nayaflow.com)

Built for defensive security. Users are responsible for compliance with applicable laws and policies.

Vigil Bot was built by Yasir and the community. 🦅 by [Nayaflow](https://nayaflow.com).

---

<div align="center">

[nayaflow.com](https://nayaflow.com) · [Docs](https://docs.nayaflow.com) · [Discord](https://discord.gg/nayaflow) · [GitHub](https://github.com/yasir23/vigil)

</div>
