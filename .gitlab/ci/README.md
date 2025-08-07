# GitLab CI

This directory contains templates and helper scripts for running the project on GitLab CI/CD. It is intended for teams that prefer GitLab while maintaining parity with the GitHub Actions setup.

Pipelines execute on runners using Arch Linux so the build and test environment matches the developers' machines. The root `.gitlab-ci.yml` file references job definitions in `jobs/` and uses utility scripts in `scripts/` for tasks such as dependency audits.

Secrets are pulled from HashiCorp Vault and container images are published to a Docker Registry.

