# GitLab CI Scripts Agent

- **Criticality:** 9/10
- **Purpose:** Shell utilities that enforce supply chain security in GitLab CI jobs.
- **Files:**
  - `cargo-audit.sh` – criticality: 9
  - `npm-audit.sh` – criticality: 9
- **Communication:**
  - `cargo-audit.sh` reaches the crates.io index and RustSec advisory database.
  - `npm-audit.sh` queries the npm registry and audit database.
- **Behaviour:**
  - Emit audit reports to job logs and attach them as CI artifacts.
  - Exit with a non-zero status to fail pipelines when vulnerabilities are found.
- **SupplyChainRiskMitigation:** Implements schema requirements such as `UseCargoAudit`, `AuditInCI`, `CommitLockfile`, and `UseVersionResolutions` to detect dependency risks early.
