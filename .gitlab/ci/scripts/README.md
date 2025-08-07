# GitLab CI Scripts

This directory contains shell scripts executed by GitLab CI jobs to audit dependencies and enforce the project's supply chain risk mitigation policy. Each script talks to external registries and vulnerability databases, reports findings back to the pipeline, and halts the build if issues are detected. Audit logs are preserved as CI artifacts for traceability.

## Files

| File | Criticality | Description |
| --- | --- | --- |
| `cargo-audit.sh` | 9 | Runs `cargo audit` against crates.io and the RustSec advisory database. |
| `npm-audit.sh` | 9 | Runs `npm audit --production` against the npm registry and its vulnerability database. |

These checks implement the **SupplyChainRiskMitigation** schema by verifying Rust and TypeScript dependencies during continuous integration.
