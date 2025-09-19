# Community Donation Handler

## Overview
This service processes community donations and supporter acknowledgments. It tracks community funding without restricting any features, as all functionality is universally available.

## Setup
1. Create donation webhook endpoints and point them at the handler's `/webhook` route.
2. Configure webhook signing secrets via environment variables.
3. Deploy the service to track community support and generate acknowledgments.

## Community Support Logic
- Supported events: donation confirmations and supporter acknowledgments.
- All users have identical access regardless of donation status.
- Community supporters receive acknowledgment but no additional features.
- All functionality remains universally available to every user.
