# Kubernetes Deployment Resources

This directory contains deployment configurations for running iVibe on Kubernetes.

## Structure

- `helm/ivibe/` – Helm chart for templated releases.
- `manifests/` – Raw YAML manifests for direct `kubectl apply` usage.

## Helm Values

The Helm chart exposes configurable values in `values.yaml`.
Common parameters include:

```yaml
image:
  repository: ghcr.io/ivibe/backend
  tag: latest
replicaCount: 2
resources:
  requests:
    cpu: 250m
    memory: 256Mi
  limits:
    cpu: 500m
    memory: 512Mi
ingress:
  enabled: true
  className: nginx
  hosts:
    - host: ivibe.example.com
      paths:
        - path: /
          pathType: Prefix
```

Override values during install or upgrade:

```sh
helm upgrade --install ivibe helm/ivibe -n ivibe -f values.yaml -f values-prod.yaml
```

## Deployment Strategies

1. **Helm (recommended)** – Manage releases with Helm using the chart in `helm/ivibe`.
2. **Raw Manifests** – Apply files under `manifests/` directly:
   ```sh
   kubectl apply -n ivibe -f manifests/
   ```

## Production Configuration

- All resources deploy into the `ivibe` namespace.
- Secrets and configuration are separated via `Secrets` and `ConfigMaps`.
- Use a `HorizontalPodAutoscaler` for CPU and memory based scaling:
  ```sh
  kubectl autoscale deployment ivibe --cpu-percent=70 --min=2 --max=10
  ```
- Maintain separate `values-prod.yaml` for production overrides such as persistent storage, domain names, and resource limits.
