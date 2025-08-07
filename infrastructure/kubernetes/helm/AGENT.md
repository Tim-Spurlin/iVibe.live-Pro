# Helm Charts

- **Criticality**: 7/10
- **Purpose**: Helm chart definitions for iVibe Kubernetes deployments
- **Contents**:
  - `ivibe/`
    - `Chart.yaml`
    - `templates/` – ConfigMaps, Secrets, Deployments, Services, Ingress rules, HorizontalPodAutoscalers, and RBAC objects
    - `values.yaml`
- **Namespace Isolation**: All templates scope resources to the `ivibe` namespace
- **RBAC**: Role and RoleBinding templates enforce least-privilege access
- **Deployment Strategy**: Follows the project's Kubernetes strategy using Helm for templated releases, enabling controlled upgrades and rollbacks
