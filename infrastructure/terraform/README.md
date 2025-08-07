# Terraform Infrastructure

This directory contains infrastructure as code for provisioning iVibe.live cloud resources using Terraform.

## Contents

- `environments/` – environment-specific state and variables (.gitkeep)
- `modules/` – reusable Terraform modules (.gitkeep)
- `main.tf` – root module entry point (criticality: 9)

## Usage

1. Configure the remote backend as described in `AGENT.md`.
2. Initialize the workspace:

```bash
terraform init
```
3. Review infrastructure changes:

```bash
terraform plan
```
4. Apply changes:

```bash
terraform apply
```

## Resources

Configurations provision VPC networking, security groups, RDS instances, S3 buckets, IAM roles, and other components defined in the project's cloud infrastructure schema.

## State Management

Terraform state is stored in a remote backend to enable collaboration and locking. Ensure backend credentials are configured before running commands.
