# Frontend

This directory contains the TypeScript/React frontend applications for the project. It is structured as a pnpm monorepo with three packages:

- **web/** – main web client
- **browser-extension/** – browser watcher and UI
- **desktop/** – desktop shell

## Development

1. Install dependencies with a frozen lockfile:

   ```bash
   pnpm install --frozen-lockfile
   ```

2. Start a development server for a package:

   ```bash
   pnpm --filter <package> dev
   ```

   Replace `<package>` with `web`, `browser-extension`, or `desktop`.

## Build for Production

Build any package using Vite:

```bash
pnpm --filter <package> build
```

The compiled assets will be written to the respective `dist/` directories.
