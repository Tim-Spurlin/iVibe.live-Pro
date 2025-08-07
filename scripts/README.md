# Scripts

Automation and deployment helpers for the project. All scripts are written in Bash for Linux/macOS. Windows users may need to adapt them or run in a compatible shell.

## Execution Order and Details

| Order | Script | Purpose | Required Environment Variables |
|-------|--------|---------|--------------------------------|
| 1 | `setup/install-deps.sh` | Install system dependencies needed for development. | None |
| 2 | `setup/generate-keys.sh` | Generate development keys and certificates. | None |
| 3 | `setup/dev-setup.sh` | Configure the local development environment. | None |
| 4 | `build/build-frontend.sh` | Build frontend assets. | None |
| 5 | `build/build-rust.sh` | Build Rust backend components. | None |
| 6 | `build/build-all.sh` | Run all build steps sequentially. | None |
| 7 | `stripe/create-products.sh` | Create products in Stripe. | `STRIPE_API_KEY` |
| 8 | `stripe/setup-webhooks.sh` | Register Stripe webhooks. | `STRIPE_API_KEY`, `WEBHOOK_ENDPOINT` |
| 9 | `deploy/deploy-prod.sh` | Deploy the production stack. | Depends on target environment (e.g., `DOCKER_TAG`). |
| 10 | `deploy/rollback.sh` | Roll back to the previous production release. | Depends on target environment. |

