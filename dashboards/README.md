# Dashboards

This directory contains Grafana dashboard JSON definitions for productivity, emotions, AI usage, social vibes, team metrics, and developer analytics.

## Importing Dashboards
1. Open your Grafana instance.
2. Navigate to **Dashboards → Import**.
3. Upload one of the JSON files from this folder or paste its contents.
4. Select the PostgreSQL with TimescaleDB data source when prompted.

## Customizing Panels
- Open a dashboard and click **Edit** on any panel.
- Adjust the query, visualization type, thresholds, and refresh interval.
- Use the built-in variables `User`, `project`, and `date range` to filter data.

## Creating New Visualizations
1. From an existing dashboard, choose **Add panel** or start a new dashboard.
2. Build queries against the PostgreSQL TimescaleDB data source.
3. Set appropriate refresh rates: 30 seconds for real-time panels, hourly for aggregate panels.
4. Save the dashboard and export its JSON into this directory to version control it.

