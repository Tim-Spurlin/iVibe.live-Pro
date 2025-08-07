# Raw Manifest Guidelines

- **Purpose**: Maintain raw Kubernetes resource files for direct application via `kubectl`.
- **Namespace Creation**: `namespace.yaml` must define and apply the `ivibe` namespace with appropriate labels before other resources.
- **ConfigMaps**: `configmaps.yaml` holds non‑secret configuration. Include keys such as `APP_CONFIG`, `DATABASE_URL`, and other application settings. Keep values env‑agnostic.
- **Secrets**: Store sensitive data in `secrets.yaml` using base64‑encoded values. Encrypt secrets with tools like `kubeseal` before committing and never store plain text credentials.
- **Deployments**: Place individual deployment YAMLs under `deployments/`. Each deployment should specify container images, ports, and health probes.
  - Define `resources.requests` and `resources.limits` for CPU and memory.
  - Use `securityContext` with `runAsNonRoot: true`, `readOnlyRootFilesystem: true`, and drop unnecessary capabilities.
- **Schema Compliance**: All manifests must conform to Kubernetes API schemas for container orchestration. Validate with `kubectl apply --dry-run=client -f <file>`.
