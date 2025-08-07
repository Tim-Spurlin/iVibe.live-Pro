# Terraform Agent

- **Criticality:** 9/10
- **Purpose:** Provision cloud infrastructure defined by the repository schema using Terraform.
- **Files:**
  - `environments/` – environment-specific configs (.gitkeep)
  - `modules/` – reusable modules (.gitkeep)
  - `main.tf` – root configuration (criticality: 9)
- **Resources Provisioned:** VPC networking, security groups, RDS instances, S3 buckets, IAM roles.
- **State Management:** Use a remote backend with state locking (e.g., S3 with DynamoDB) to manage Terraform state.
- **Schema Reference:** Align with cloud infrastructure requirements in the root `AGENTS.md` schema, ensuring secure networking, least-privilege IAM, and encrypted storage.
- **Remote Backend:** Configure backend credentials before running `terraform init`.
