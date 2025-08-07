# Infrastructure

This directory contains deployment and configuration resources for the platform.

## Development: Docker Compose

Use the files in `docker/` to launch a local stack:

```bash
cd docker
docker compose up -d
```

This environment is intended for local testing and mirrors production services on a smaller scale.

## Production: Kubernetes

Kubernetes manifests live in `kubernetes/`. Provision cloud resources with Terraform before applying the manifests:

```bash
cd terraform
terraform init
terraform apply
cd ../kubernetes
kubectl apply -k .
```

Helm or Kustomize may be used to manage production deployments.

## Infrastructure as Code

- **Terraform** configurations in `terraform/` provision cloud infrastructure.
- **Ansible** playbooks in `ansible/` manage configuration for servers and services.

## Scaling Considerations

- Use Kubernetes Horizontal Pod Autoscalers to scale application pods based on load.
- Ensure stateful services use managed offerings or persistent volumes that support scaling.
- Review resource requests and limits in manifests to avoid overcommitment.
- For development, Docker Compose is not intended for high availability or scaling.
