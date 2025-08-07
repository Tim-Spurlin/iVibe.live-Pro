# Helm Chart Definitions

This directory contains Helm chart definitions for deploying iVibe on Kubernetes.

## Contents
- `ivibe/`
  - `Chart.yaml`
  - `templates/`
  - `values.yaml`

The templates define ConfigMaps, Secrets, Deployments, Services, Ingress rules, HorizontalPodAutoscalers, and RBAC objects. Resources are isolated in the `ivibe` namespace. This setup follows the repository's Kubernetes deployment strategy of managing releases with Helm for repeatable upgrades and rollbacks.
