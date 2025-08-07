# GitLab CI Agent Instructions

This directory hosts the GitLab CI/CD configuration for the project. Use the following guidelines when modifying files here.

## Pipeline Stages
1. `test`
2. `build`
3. `security`
4. `deploy`

## Runners
- Jobs run on Arch Linux runners so the environment matches the preferred stack.

## Security
- Every commit must run `cargo audit` and `npm audit`.

## Build
- Build Docker images with locked dependencies using `--locked`.

## Deployment
- Releases are applied via Kubernetes manifests and Helm charts.

## Artifacts
- Produce signed binaries and generate SBOMs for traceability.

