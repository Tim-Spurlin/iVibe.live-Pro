# Test Strategy and Execution

This folder aggregates all automated test suites for the project with a criticality level of 7. The strategy spans unit, integration, end-to-end, load, and security testing to ensure robust coverage and reliability.

## Directory Structure
- `e2e/`
  - `cypress/`
  - `playwright/`
- `integration/`
  - `api/`
  - `services/`
- `load/`
  - `k6/scenarios/`
  - `locust/locustfile.py`
- `security/`
  - `zap/`
  - `burp/`

## Running Tests
### Unit Tests
Run the unit test suites from their respective packages. A minimum of 80% coverage is required.

### End-to-End
```bash
npm run test:e2e:cypress
npm run test:e2e:playwright
```

### Integration
```bash
npm run test:integration:api
npm run test:integration:services
```

### Load
```bash
k6 run --vus 100 --duration 10m load/k6/<script.js>
locust -f load/locust/locustfile.py
```

### Security
```bash
zap-cli quick-scan http://localhost:8080
# Burp Suite tests are executed manually or via configured automation
```

## CI/CD Integration
All test suites are executed in the continuous integration pipeline for every commit and pull request. Coverage thresholds are enforced to guarantee at least 80% unit test coverage.
