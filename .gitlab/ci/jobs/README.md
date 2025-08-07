# GitLab CI Jobs

This directory contains GitLab CI job definitions referenced by the root `.gitlab-ci.yml`. The jobs mirror the pipeline architecture described in the project schema: **Build → Test → Security → Deploy**.

## Job Files

| File | Criticality | Description |
| --- | --- | --- |
| `build.yml` | 8 | Compile sources and build Docker images |
| `test.yml` | 8 | Execute `cargo test` and `npm test` |
| `security.yml` | 9 | Run dependency and container security checks |
| `deploy.yml` | 9 | Publish artifacts or images to deployment targets |

Jobs run on GitLab runners using Docker. They leverage caches to reuse Cargo and npm dependencies and upload artifacts (binaries, reports, images) for later stages. Status for each job is reported back to merge requests so reviewers can track pipeline results.

