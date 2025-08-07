# Deployment Automation

This directory contains scripts that automate production deployments for the iVibe.live platform.

## Scripts

| File | Criticality | Purpose |
| --- | --- | --- |
| `deploy-prod.sh` | 9 | Perform blue‑green deployment to production, including migrations and health checks. |
| `rollback.sh` | 9 | Restore the previous production environment if a deployment fails. |

See `AGENT.md` for operational guidelines.
