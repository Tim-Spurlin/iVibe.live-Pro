# AGENT Instructions

- Criticality: main.rs 10/10, webhook.rs 10/10, database.rs 9/10.
- Webhook signature verification: `webhook.rs` must validate the `Stripe-Signature` header using the `STRIPE_WEBHOOK_SECRET` before any processing.
- Events:
  - `subscription.created` and `subscription.updated`: update `users.tier`, `users.subscription_id`, and `users.seats` while adjusting retention days and integration limits per tier.
  - `invoice.paid`: confirm payment status and reconcile seat counts to keep subscriptions active.
- Seat count management: extract the subscription quantity from events and store it in `users.seats`.
- Retention policy adjustments: modify `retention_days` and related limits when tiers change.
- PostgreSQL updates: persist changes through `database.rs`, adhering to the billing integration schema for `users` and related tables.
- Stripe API callbacks: invoke Stripe APIs as needed to fetch subscription or invoice details and acknowledge processed events.
