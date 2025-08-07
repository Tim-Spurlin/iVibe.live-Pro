# Ingestion Module Guidelines

This directory coordinates event batch ingestion for `aw-server-rust`. The rules below govern contributors working on this module.

## Batch accumulation
- Use an in-memory buffer that holds up to **1000 events**.
- Flush the buffer every **5 seconds** or when full, whichever comes first.
- Preserve arrival order within a batch and avoid partial writes.

## Event validation
- Validate every event against **EventSchema v0.9** before it enters the buffer.
- Reject or quarantine events that do not conform to the schema.
- Reference the ingestion service specifications to maintain compatibility.

## Tenant policies
- Extract the tenant identifier from request metadata.
- Apply tenant **row-level security** when persisting to storage so events remain isolated per tenant.

## Duplicate detection
- Use the `event_id` field to detect duplicates.
- Ensure writes are idempotent by skipping or updating already ingested events.

## Retry and backoff
- On transient storage failures, retry writes using **exponential backoff**.
- Abort after a bounded number of retries and surface errors to the caller.

## Storage communication
- After validation, forward batches asynchronously to storage backends:
  - Insert JSON payloads into TimescaleDB/ Postgres tables.
  - Store audio metadata and blobs via MinIO.
- Propagate backpressure signals from storage to the batch accumulator.

