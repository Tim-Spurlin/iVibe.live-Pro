# Raw Kubernetes Manifests

This directory provides untemplated Kubernetes YAML files for deploying iVibe resources without Helm.

## Contents

- `deployments/` – container deployment specifications (.gitkeep placeholder).
- `configmaps.yaml` – shared ConfigMap definitions (criticality: 8).
- `namespace.yaml` – creation of the `ivibe` namespace (criticality: 9).
- `secrets.yaml` – sensitive configuration as Kubernetes Secrets (criticality: 10).

Apply manifests in the above order to ensure the namespace and configuration exist before workloads.
