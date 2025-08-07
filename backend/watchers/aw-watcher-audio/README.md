# aw-watcher-audio

`aw-watcher-audio` captures short snippets from the microphone and turns them into privacy-preserving embeddings. The watcher only runs when activated by a hot word, a change in application context, or manual user action.

## Privacy

- Audio is recorded as **16kHz mono** for efficient processing.
- Samples are converted to vector embeddings on the device and the raw audio is discarded immediately.
- Only embeddings are stored in PostgreSQL using the `pgvector` extension.

## Vectorization Process

1. Detect trigger and capture a short audio segment.
2. Normalize and resample to 16kHz mono if necessary.
3. Generate an embedding for the audio segment.
4. Send the embedding to the server for storage and similarity search.

## Hot-word Configuration

The watcher listens for a configurable hot word that activates audio capture. Set the desired phrase and sensitivity in the configuration file or environment variables. When the phrase is spoken, the watcher records the following few seconds for vectorization.

## Triggers

- Hot-word detection
- Context changes (e.g., active application or project)
- Manual activation through the CLI
