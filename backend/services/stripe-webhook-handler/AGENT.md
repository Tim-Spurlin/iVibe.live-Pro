# AGENT Instructions

- Criticality: 9/10
- Purpose: Subscription management and billing
- Files:
  - src/main.rs
  - src/webhook.rs
  - src/events/subscription.rs
  - src/events/invoice.rs
  - src/database.rs
- Webhook events: subscription.created, subscription.updated, invoice.paid
- Updates: users.tier, subscription_id, seats
- Signature verification: Stripe webhook secret
- Tier enforcement: Adjusts retention_days, integration_limits
