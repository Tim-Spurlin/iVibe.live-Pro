# Deployment Automation Guidelines

- **Criticality:** 9/10
- **Purpose:** Automate production deployments for iVibe.live.

## Files
- `deploy-prod.sh` – orchestrates blue‑green deployments (criticality: 9).
- `rollback.sh` – reverts to the previous stable version (criticality: 9).

## Deployment Strategy
- **Blue‑green deployments:** provision the new release in an idle environment and cut traffic over only after verification.
- **Health checks:** probe `/health` and dependent services before and after switch; failures abort promotion.
- **Database migrations:** execute migrations against the target environment prior to traffic cutover; scripts must be idempotent.
- **Service discovery updates:** update DNS and service registry entries once the new environment is healthy.
- **Automatic rollback:** failed checks or alerts trigger `rollback.sh` to restore the prior environment.

## Monitoring & Alerting
- Integrate with Prometheus metrics and Alertmanager (or equivalent) to watch deployments.
- Emit deployment events and notify the on‑call channel; watch for alerts during the post‑deploy window.

## Reference
These scripts implement the production deployment flow described in the project's schema and infrastructure documentation.
