# GitLab CI Agent

- **Criticality:** 7/10
- **Purpose:** GitLab CI/CD pipeline configuration (alternative to GitHub Actions)
- **Files:**
  - `.gitlab-ci.yml` – criticality: 8
- **Subdirectories:**
  - `jobs/` – test, build, deploy, security configs
  - `scripts/` – `cargo-audit.sh`, `npm-audit.sh`
- **Communication:** GitLab runners on Arch Linux
- **Triggers:** Push events, merge requests, scheduled pipelines
- **Integration:** HashiCorp Vault for secrets, Docker Registry for images

