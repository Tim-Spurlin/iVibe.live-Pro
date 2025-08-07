# Stripe Webhook Handler

## Overview
This service processes Stripe webhook events to manage subscription tiers and billing state. It listens for subscription and invoice events and updates user records accordingly.

## Setup
1. Create a webhook endpoint in the Stripe dashboard and point it at the handler's `/webhook` route.
2. Retrieve the webhook signing secret and expose it to the service via the `STRIPE_WEBHOOK_SECRET` environment variable.
3. Deploy the service and ensure Stripe can reach it over HTTPS. The handler verifies each request using the secret before processing events.

## Subscription Tier Logic
- Supported events: `subscription.created`, `subscription.updated`, and `invoice.paid`.
- On subscription creation or update, the handler updates `users.tier` and `users.subscription_id` and adjusts retention days and integration limits based on the subscribed tier.
- When an invoice is paid, the handler confirms the subscription is active and records the seat count.

## Seat Counting for Team Plans
Team and Business plans are billed per seat. The webhook extracts the subscription quantity and stores it in `users.seats`. This count is used for reconciliation and enforcement of per-seat limits.
