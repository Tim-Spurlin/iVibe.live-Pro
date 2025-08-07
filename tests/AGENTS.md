# Tests Directory Instructions

This directory contains all automated test suites for the project. These tests are critical (criticality level 7) and run on every commit via CI.

## Structure
- `e2e/`
  - `cypress/`: End-to-end tests covering full user flows using Cypress.
  - `playwright/`: Cross-browser end-to-end tests implemented with Playwright.
- `integration/`
  - `api/`: API endpoint tests.
  - `services/`: Tests verifying inter-service communication.
- `load/`
  - `k6/`: API load scripts executed with 100 virtual users for 10 minutes.
  - `locust/`: Complex scenario load tests defined in `locustfile.py`.
- `security/`
  - `zap/`: Automated OWASP ZAP scanning.
  - `burp/`: Manual and automated Burp Suite testing assets.

## Guidelines
- End-to-end tests model realistic user journeys through Cypress or Playwright.
- Integration tests validate API endpoints and service interactions.
- Load testing uses k6 (`k6 run --vus 100 --duration 10m`) for standard API load and Locust for complex scenarios.
- Security testing leverages OWASP ZAP and Burp Suite to detect vulnerabilities.
- Unit test suites across the codebase must maintain at least 80% coverage.
- All tests are executed in CI on every commit.
