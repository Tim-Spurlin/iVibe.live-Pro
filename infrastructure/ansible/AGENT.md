# Ansible Agent

- **Criticality:** 6/10
- **Purpose:** Configuration management for servers and services
- **Server provisioning playbooks:** Define and provision hosts according to the deployment automation schema.
- **Application deployment tasks:** Deploy application components consistently across environments.
- **Configuration templating:** Use Jinja2 templates for idempotent configuration files.
- **Secret management:** Protect credentials and variables with Ansible Vault.
- **Idempotency:** Ensure tasks can be rerun safely and use handlers to apply changes only when needed.
- **Role dependencies:** Declare dependencies in role `meta/main.yml` files to maintain proper execution order.
- **Subdirectories:** `inventory/`, `playbooks/`
