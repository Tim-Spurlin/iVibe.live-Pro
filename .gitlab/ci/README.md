# GitLab CI/CD

The `.gitlab/ci` directory defines the GitLab pipeline for iVibe. The pipeline is considered criticality level 7 and runs through four stages: `test`, `build`, `security`, and `deploy`.

## Workflow
1. **Test** – execute project tests to validate changes.
2. **Build** – produce Docker images with locked dependencies (`--locked`).
3. **Security** – run dependency scans using `cargo audit` and `npm audit` on every commit.
4. **Deploy** – apply Kubernetes manifests and Helm charts to release to the cluster.

Artifacts produced in the build stage are signed and accompanied by SBOMs for traceability.

## Runner Setup
Runners are based on Arch Linux to mirror the development stack. Each runner must have Docker, Rust tooling, Node.js, and Helm installed.

## Deployment Strategy
Deployments use Kubernetes with Helm charts. Generated artifacts are pulled into the cluster and applied via manifests to roll out new versions.

