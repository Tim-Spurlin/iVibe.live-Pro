# Ingestion Module

This folder contains the batching logic for `aw-server-rust`. It collects incoming events, validates them, and forwards batches to the storage layer.

## Files
| File | Description | Criticality |
| --- | --- | --- |
| `batch.rs` | Implements the accumulator with a **1000 event** buffer and a **5s** flush interval. | 9 |
| `mod.rs` | Module glue that wires the ingestion components together. | 8 |
| `validator.rs` | Validates events before batching and ensures schema compliance. | 9 |

## Behavior
- Handles event batch processing and communication with storage backends.
- Ensures events conform to the [EventSchema v0.9](../../../../../AGENTS.md#eventschema) prior to persistence.
