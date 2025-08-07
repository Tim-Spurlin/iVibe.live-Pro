# IntelliJ Plugin Metadata

- **Criticality:** 9/10
- **Purpose:** IntelliJ IDEA and JetBrains IDE integration for iVibe.live

## Structure
- `src/main/kotlin/` – Kotlin sources
- `src/main/resources/` – resource files including `META-INF/plugin.xml`
- `README.md` – overview
- `build.gradle.kts` – Gradle build

## IntelliJ Platform SDK Usage
- Built on the IntelliJ Platform SDK for JetBrains IDE support
- Registers project, file, and editor listeners
- Analyses the PSI tree to capture structural information
- Monitors run configurations to detect execution events
- Batches events before forwarding them to the platform API

## Plugin Configuration
- `src/main/resources/META-INF/plugin.xml` configures plugin id, vendor, and actions
- Compatibility range: 2022.3–2024.* JetBrains IDEs

## Event Model
- Emits events aligned with the repository's schema for JetBrains IDE support
