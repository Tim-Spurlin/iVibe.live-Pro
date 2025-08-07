# GitLab CI Jobs Agent

- **Criticality:** 8/10
- **Purpose:** Defines job templates for the GitLab CI/CD pipeline (Build → Test → Security → Deploy) described in the project's schema.
- **Files:**
  - `build.yml` – criticality: 8
  - `test.yml` – criticality: 8
  - `deploy.yml` – criticality: 9
  - `security.yml` – criticality: 9
- **Communication with Runners:** Jobs run on GitLab runners (Arch Linux) using Docker executors. Each job spins up a container, executes cargo/npm commands, and streams logs and status back to the pipeline so merge requests reflect success or failure.
- **Docker Builds:** The build job triggers Docker builds and pushes resulting images to the project registry.
- **Commands:**
  - `build.yml` compiles code and packages images.
  - `test.yml` runs `cargo test` and `npm test`.
  - `security.yml` calls `cargo audit` and `npm audit` via scripts in `../scripts`.
  - `deploy.yml` uses built artifacts or images to update deployment targets.
- **Artifacts & Cache:** Jobs upload binaries, reports, and container images to GitLab's artifact storage. Shared caches preserve Cargo and npm dependencies between stages, reducing pipeline time.
- **Status Reporting:** Pipeline results are posted back to merge requests through GitLab's CI status API.
- **Deployment Targets:** Deployments interact with environments such as Kubernetes clusters or Docker hosts defined by the infrastructure manifests.

