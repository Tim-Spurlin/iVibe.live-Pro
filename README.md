# iVibe.live 🌟

<div align="center">
  
  [![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
  [![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue?style=for-the-badge&logo=typescript)](https://www.typescriptlang.org/)
  [![PostgreSQL](https://img.shields.io/badge/PostgreSQL-15+-336791?style=for-the-badge&logo=postgresql)](https://www.postgresql.org/)
  [![Docker](https://img.shields.io/badge/Docker-24.0+-2496ED?style=for-the-badge&logo=docker)](https://www.docker.com/)
  [![License](https://img.shields.io/badge/License-Proprietary-red?style=for-the-badge)](LICENSE)
  
  **Redefining Social Media Through Genuine Human Connection**
  
  *Replace algorithmic feeds and superficial likes with proximity-based growth, user-controlled privacy, and meaningful relationships*
  
  [🚀 Quick Start](#-quick-start) • [📚 Documentation](#-documentation) • [💰 Pricing](#-pricing-plans) • [🛠️ Architecture](#%EF%B8%8F-system-architecture) • [🔒 Security](#-security--privacy)
  
</div>

---

## 📑 Table of Contents

- [🎯 Executive Summary](#-executive-summary)
- [✨ Core Features](#-core-features)
  - [Feature Matrix](#feature-matrix)
  - [Platform Capabilities](#platform-capabilities)
- [🛠️ System Architecture](#%EF%B8%8F-system-architecture)
  - [Four-Plane Architecture](#four-plane-architecture)
  - [Component Overview](#component-overview)
  - [Data Flow Diagram](#data-flow-diagram)
- [💻 Technology Stack](#-technology-stack)
  - [Core Technologies](#core-technologies)
  - [Infrastructure Services](#infrastructure-services)
- [📊 Module Specifications](#-module-specifications)
  - [Capture Modules](#capture-modules)
  - [Processing Services](#processing-services)
  - [API Services](#api-services)
- [💰 Pricing Plans](#-pricing-plans)
  - [Subscription Tiers](#subscription-tiers)
  - [Feature Comparison](#feature-comparison)
  - [Data Retention Matrix](#data-retention-matrix)
- [🚀 Quick Start](#-quick-start)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Configuration](#configuration)
- [🔧 Development Setup](#-development-setup)
  - [Arch Linux (Zen Kernel)](#arch-linux-zen-kernel)
  - [Windows](#windows)
  - [macOS](#macos)
- [🔌 Integrations](#-integrations)
  - [IDE/Editor Support](#ideeditor-support)
  - [External Services](#external-services)
  - [SDK Development](#sdk-development)
- [📈 Analytics & Dashboards](#-analytics--dashboards)
  - [Grafana Configuration](#grafana-configuration)
  - [Metric Categories](#metric-categories)
- [🔒 Security & Privacy](#-security--privacy)
  - [Privacy Architecture](#privacy-architecture)
  - [Compliance & Consent](#compliance--consent)
- [🧪 Testing & Quality](#-testing--quality)
  - [Test Strategy](#test-strategy)
  - [Supply Chain Security](#supply-chain-security)
- [📅 Development Roadmap](#-development-roadmap)
- [🤝 Contributing](#-contributing)
- [📖 Glossary](#-glossary)
- [📄 License](#-license)

---

## 🎯 Executive Summary

iVibe.live represents a paradigm shift in social media and productivity analytics. By combining **proximity-based social connections**, **comprehensive activity tracking**, and **emotion analytics**, we create a platform that fosters genuine human connections while providing unprecedented insights into personal and team productivity.

### Key Differentiators

| Traditional Social Media | iVibe.live |
|-------------------------|------------|
| Algorithmic feeds | Proximity-based discovery |
| Likes & follower counts | Vibe scores (1-1000) based on real compatibility |
| Superficial engagement | In-person interaction rewards |
| Data harvesting | Local-first, privacy-respecting |
| Isolated metrics | Holistic well-being tracking |

### Platform Philosophy

```mermaid
mindmap
  root((iVibe.live))
    Human Connection
      Proximity Detection
      Vibe Scoring
      Real Interactions
    Productivity Intelligence
      Activity Tracking
      AI Usage Monitoring
      Error Analysis
    Emotional Wellness
      Mood Patterns
      Trigger Analysis
      Context Capture
    Privacy First
      Local Storage
      Audio Vectorization
      Selective Sharing
```

---

## ✨ Core Features

### Feature Matrix

| Feature | Free | Essential | Elite | Team | Business |
|---------|------|-----------|-------|------|----------|
| **Activity Tracking** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Terminal Intelligence** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Browser Monitoring** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Vibe Social System** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Advanced Analytics** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **AI Token Tracking** | ❌ | ✅ | ✅ | ✅ | ✅ |
| **Mobile Suite** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Emotion Analytics** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Location Intelligence** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Custom Integrations** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Export Data** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Team Dashboards** | ❌ | ❌ | ❌ | ✅ | ✅ |
| **SSO/Enterprise** | ❌ | ❌ | ❌ | ❌ | ✅ |

### Platform Capabilities

```mermaid
graph LR
    subgraph "Desktop Tracking"
        DT[Window Focus]
        DF[File Tracking]
        DB[Branch Detection]
        DL[Language Analysis]
    end
    
    subgraph "Terminal Intelligence"
        TC[Command Capture]
        TE[Error Classification]
        TF[Fix Detection]
        TS[Success Rates]
    end
    
    subgraph "Browser Monitoring"
        BU[URL Tracking]
        BT[Tab Duration]
        BA[AI Chat Detection]
        BR[Research Classification]
    end
    
    subgraph "Mobile Features"
        MA[App Usage]
        ML[Location Context]
        ME[Emotion Detection]
        MV[Audio Vectorization]
    end
    
    subgraph "Social Features"
        VP[Proximity Detection]
        VS[Vibe Scoring]
        VR[Friend Requests]
        VL[Viber Links]
    end
```

---

## 🛠️ System Architecture

### Four-Plane Architecture

```mermaid
graph TB
    subgraph "1. Capture Plane"
        subgraph "Desktop Watchers"
            WW[aw-watcher-window<br/>Rust]
            KW[aw-watcher-kdevelop<br/>Rust]
            TW[aw-watcher-terminal<br/>Rust]
        end
        
        subgraph "Browser Extension"
            BW[aw-watcher-browser<br/>TypeScript]
        end
        
        subgraph "Mobile Watchers"
            MW[aw-watcher-mobile<br/>Kotlin + Rust FFI]
            AW[aw-watcher-audio<br/>Rust]
        end
        
        subgraph "Intelligence Services"
            ED[EmotionDetectionService<br/>Rust + WASM]
            VS[VibeSocialEngine<br/>Rust]
        end
    end
    
    subgraph "2. Transport Plane"
        EG[EventGateway<br/>Rust/gRPC]
        KC[Keycloak<br/>OIDC/OAuth2]
        TLS[TLS 1.3<br/>mTLS Internal]
    end
    
    subgraph "3. Processing Plane"
        IS[aw-server-rust<br/>Event Ingestion]
        KF[(Kafka<br/>Event Stream)]
        VEC[VectoriserService<br/>OpenAI Embeddings]
        SUM[SummariserJob<br/>GPT-4o Summaries]
        ETL[ETLOrchestrator<br/>Materialised Views]
    end
    
    subgraph "4. Presentation Plane"
        API[REST/GraphQL API<br/>Rust/Actix]
        WEB[React Frontend<br/>TypeScript]
        MOB[Mobile Apps<br/>Kotlin/Swift]
        GRAF[Grafana Dashboards<br/>TimescaleDB]
        PUB[Public Profiles<br/>Privacy Filtered]
    end
    
    subgraph "Storage Layer"
        PG[(PostgreSQL<br/>TimescaleDB<br/>pgvector)]
        MIN[(MinIO<br/>Object Store)]
        RED[(Redis<br/>Cache)]
    end
    
    WW --> EG
    KW --> EG
    TW --> EG
    BW --> EG
    MW --> EG
    AW --> EG
    ED --> EG
    VS --> EG
    
    EG --> KC
    KC --> TLS
    TLS --> IS
    
    IS --> PG
    IS --> KF
    KF --> VEC
    KF --> SUM
    VEC --> PG
    SUM --> PG
    ETL --> PG
    
    API --> PG
    WEB --> API
    MOB --> API
    GRAF --> PG
    PUB --> API
    
    style WW fill:#ff9999
    style KW fill:#ff9999
    style TW fill:#ff9999
    style BW fill:#99ccff
    style MW fill:#ffcc99
    style AW fill:#ff9999
    style ED fill:#ff9999
    style VS fill:#ff9999
    style EG fill:#ff9999
    style IS fill:#ff9999
    style VEC fill:#ff9999
    style SUM fill:#ff9999
    style ETL fill:#ff9999
    style API fill:#ff9999
    style WEB fill:#99ccff
    style MOB fill:#ffcc99
```

### Component Overview

| Component | Language | Purpose | Communication |
|-----------|----------|---------|---------------|
| **aw-watcher-window** | Rust | Desktop window tracking | gRPC → Gateway |
| **aw-watcher-kdevelop** | Rust | KDevelop IDE integration | gRPC → Gateway |
| **aw-watcher-browser** | TypeScript | Browser activity monitoring | WebSocket → Gateway |
| **aw-watcher-terminal** | Rust | Shell command tracking | gRPC → Gateway |
| **aw-watcher-mobile** | Kotlin/Rust | Mobile app tracking | HTTPS → Gateway |
| **aw-watcher-audio** | Rust | Audio vectorization | gRPC → Gateway |
| **EmotionDetectionService** | Rust/WASM | Facial emotion analysis | gRPC → Gateway |
| **VibeSocialEngine** | Rust | Proximity & scoring | gRPC → Gateway |
| **EventGateway** | Rust | API gateway & auth | gRPC/HTTP2 |
| **aw-server-rust** | Rust | Event ingestion | Kafka/Postgres |
| **VectoriserService** | Rust | Text/audio embeddings | Kafka consumer |
| **SummariserJob** | Rust | Daily AI summaries | Cron/Postgres |
| **ETLOrchestrator** | Rust | Data aggregation | Postgres/Cron |

### Data Flow Diagram

```mermaid
sequenceDiagram
    participant W as Watcher
    participant G as Gateway
    participant K as Keycloak
    participant I as Ingestion
    participant Q as Kafka
    participant V as Vectorizer
    participant S as Summarizer
    participant P as Postgres
    participant A as API
    participant U as User
    
    W->>G: Event Batch (gRPC/TLS)
    G->>K: Validate JWT
    K-->>G: User Claims
    G->>I: Authenticated Event
    I->>P: Store Raw Event
    I->>Q: Publish to Stream
    
    par Vectorization
        Q->>V: Consume Event
        V->>V: Generate Embeddings
        V->>P: Store Vectors
    and Summarization
        S->>P: Fetch 24h Events
        S->>S: Call OpenAI GPT-4o
        S->>P: Store Summary
    end
    
    U->>A: Request Dashboard
    A->>P: Query Metrics
    P-->>A: Aggregated Data
    A-->>U: Render Dashboard
```

---

## 💻 Technology Stack

### Core Technologies

| Layer | Technology | Purpose | Version |
|-------|------------|---------|---------|
| **Backend Language** | Rust | Core services, watchers, APIs | 1.75+ |
| **Frontend Language** | TypeScript | Web UI, browser extension, SDK | 5.0+ |
| **Mobile** | Kotlin | Android app | 1.9+ |
| **Database** | PostgreSQL | Primary data store | 15+ |
| **Time Series** | TimescaleDB | Event storage | 2.13+ |
| **Vector Store** | pgvector | Embeddings | 0.5+ |
| **Message Queue** | Kafka | Event streaming | 3.6+ |
| **Cache** | Redis | Session & data cache | 7.2+ |
| **Object Storage** | MinIO | Audio metadata | Latest |
| **Analytics** | Grafana | Dashboards | 10.2+ |
| **Identity** | Keycloak | Authentication | 23.0+ |
| **Proxy** | Nginx | Reverse proxy | 1.25+ |
| **AI Proxy** | Helicone | OpenAI monitoring | Latest |
| **Container** | Docker | Deployment | 24.0+ |

### Infrastructure Services

```mermaid
graph LR
    subgraph "External Services"
        OAI[OpenAI API]
        STR[Stripe API]
        GH[GitHub/GitLab]
    end
    
    subgraph "Proxy Layer"
        HEL[Helicone]
        NGX[Nginx]
    end
    
    subgraph "Core Services"
        PG[(PostgreSQL)]
        KF[Kafka]
        RED[Redis]
        MIN[MinIO]
    end
    
    subgraph "Identity & Monitoring"
        KC[Keycloak]
        GRF[Grafana]
        PRO[Prometheus]
        LOK[Loki]
    end
    
    OAI --> HEL
    HEL --> VEC[Vectorizer]
    HEL --> SUM[Summarizer]
    STR --> WH[Webhook Handler]
    GH --> INT[Integrations]
```

---

## 📊 Module Specifications

### Capture Modules

#### Desktop Watchers

| Module | Platform | Capture Data | Update Frequency |
|--------|----------|--------------|------------------|
| **aw-watcher-window** | Linux/Win/Mac | Window title, app name, file path | 1-5 seconds |
| **aw-watcher-kdevelop** | Linux | Project, branch, language, errors | On change |
| **aw-watcher-terminal** | All | Commands, exit codes, duration | Per command |

#### Browser Extension

| Feature | Data Captured | Privacy |
|---------|---------------|---------|
| **URL Tracking** | Full URL, domain, path | Local filtering |
| **Tab Focus** | Active duration, switches | Aggregated |
| **AI Detection** | ChatGPT, Claude, Gemini usage | Token counting |
| **Research Mode** | Academic vs casual browsing | ML classification |

#### Mobile Tracking

| Component | Android | iOS | Data Type |
|-----------|---------|-----|-----------|
| **App Usage** | ✅ | 🔜 | Foreground time, switches |
| **Location** | ✅ | 🔜 | GPS, WiFi positioning |
| **Emotion** | ✅ | 🔜 | Face detection, mood |
| **Audio** | ✅ | 🔜 | Vectorized embeddings |
| **Vibe Detection** | ✅ | 🔜 | Bluetooth LE, proximity |

### Processing Services

```mermaid
graph LR
    subgraph "Event Processing Pipeline"
        I[Ingestion<br/>10k events/sec]
        V[Vectorization<br/>OpenAI API]
        S[Summarization<br/>GPT-4o]
        E[ETL<br/>Hourly/Daily]
    end
    
    subgraph "Data Transformations"
        R[Raw Events]
        EM[Embeddings]
        SU[Summaries]
        MV[Material Views]
    end
    
    I --> R
    R --> V
    V --> EM
    R --> S
    S --> SU
    R --> E
    E --> MV
```

### API Services

| Endpoint | Method | Purpose | Auth Required |
|----------|--------|---------|---------------|
| `/api/events` | POST | Submit event batch | JWT |
| `/api/dashboard` | GET | Fetch user metrics | JWT |
| `/api/vibe/nearby` | GET | Find nearby vibers | JWT + Location |
| `/api/profile/{id}` | GET | Public profile | Optional |
| `/api/export` | POST | Generate data export | JWT + Tier |
| `/api/integrations` | GET/POST | Manage integrations | JWT |
| `/webhook/stripe` | POST | Billing updates | Signature |

---

## 💰 Pricing Plans

### Subscription Tiers

| Plan | Monthly | Annual (per month) | Users | Best For |
|------|---------|-------------------|-------|----------|
| **Free** | $0 | $0 | Individual | Trying iVibe, basic tracking |
| **Essential** | $4.50 | $4.00 | Individual | Professionals, AI tracking |
| **Elite** | $10.00 | $8.00 | Individual | Power users, all features |
| **Team** | $15.50/user | $13.50/user | 2-50 | Small teams, shared insights |
| **Business** | $18.25/user | $16.25/user | 50+ | Enterprise, compliance tools |

### Feature Comparison

| Feature | Free | Essential | Elite | Team | Business |
|---------|------|-----------|-------|------|----------|
| **Dashboard History** | 1 week | 2 weeks | Unlimited | 365 days | Unlimited |
| **Email Reports** | Weekly | Daily + Weekly | All | All | All |
| **Programming Goals** | 1 | 3 | Unlimited | Unlimited | Unlimited |
| **Leaderboard Type** | Public | Private | Private | Private | Private |
| **Leaderboard Size** | Unlimited | 5 users | 50 users | 100 users | 1000 users |
| **Core Integrations** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Custom Integrations** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Mobile Features** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Audio Intelligence** | ❌ | ❌ | ✅ | ✅ | ✅ |
| **Team Dashboards** | ❌ | ❌ | ❌ | ✅ | ✅ |
| **SSO Support** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Support Level** | Community | Email | Priority Chat | Priority | Dedicated |

### Data Retention Matrix

| Data Type | Free | Essential | Elite | Team | Business |
|-----------|------|-----------|-------|------|----------|
| **Raw Events** | 7 days | 14 days | Infinite | 365 days | Policy-driven |
| **Summaries** | 90 days | 180 days | Infinite | Infinite | Infinite |
| **Audio Objects** | 30 days | 90 days | 365 days | Configurable | Configurable |
| **Embeddings** | 90 days | 180 days | Infinite | Infinite | Infinite |

---

## 🚀 Quick Start

### Prerequisites

```bash
# System Requirements
- CPU: 4+ cores recommended
- RAM: 8GB minimum, 16GB recommended
- Storage: 50GB+ available
- OS: Linux (preferred), Windows 10+, macOS 12+

# Software Requirements
- Rust 1.75+ with cargo
- Node.js 20+ with pnpm
- Docker 24.0+ & Docker Compose 2.23+
- PostgreSQL 15+ (or via Docker)
- Git with 2FA enabled
```

### Installation

#### 1. Clone Repository

```bash
git clone https://github.com/yourusername/ivibe-live.git
cd ivibe-live
```

#### 2. Install Dependencies

```bash
# Rust dependencies (with lock file)
cargo build --locked

# TypeScript dependencies (with frozen lockfile)
pnpm install --frozen-lockfile

# System dependencies (Arch Linux)
sudo pacman -S postgresql timescaledb postgresql-libs docker docker-compose nginx redis
```

#### 3. Environment Setup

```bash
# Copy example environment
cp .env.example .env

# Edit with your configuration
vim .env
```

Required environment variables:
```env
# Database
DATABASE_URL=postgresql://user:pass@localhost/ivibe
REDIS_URL=redis://localhost:6379

# OpenAI (via Helicone)
OPENAI_API_KEY=sk-...
HELICONE_API_KEY=...

# Stripe
STRIPE_SECRET_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Keycloak
KEYCLOAK_URL=http://localhost:8080
KEYCLOAK_REALM=ivibe
KEYCLOAK_CLIENT_ID=ivibe-backend
KEYCLOAK_CLIENT_SECRET=...

# Security
JWT_SECRET=... # Generate with: openssl rand -hex 32
ENCRYPTION_KEY=... # Generate with: openssl rand -hex 32
```

#### 4. Start Services

```bash
# Start infrastructure services
docker-compose up -d postgres redis kafka minio keycloak grafana

# Run database migrations
cargo run --bin migrate

# Start core services
cargo run --bin event-gateway &
cargo run --bin ingestion-service &
cargo run --bin vectorizer &
cargo run --bin api-server &

# Start frontend development server
cd frontend && pnpm dev
```

### Configuration

#### Database Setup

```sql
-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS timescaledb;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pg_stat_statements;

-- Create hypertables for time-series data
SELECT create_hypertable('events_raw', 'timestamp');
SELECT create_hypertable('emotion_events', 'timestamp');
SELECT create_hypertable('location_history', 'timestamp');
```

#### Grafana Dashboard Import

```bash
# Import pre-built dashboards
curl -X POST http://admin:admin@localhost:3000/api/dashboards/db \
  -H "Content-Type: application/json" \
  -d @dashboards/productivity.json
```

---

## 🔧 Development Setup

### Arch Linux (Zen Kernel)

```bash
# Install Zen kernel and headers
sudo pacman -S linux-zen linux-zen-headers

# Install NVIDIA drivers (for GPU-accelerated emotion detection)
sudo pacman -S nvidia-open-dkms

# Wayland tools (avoid X11)
sudo pacman -S wayland wlroots wlr-randr wl-clipboard

# KDE Plasma 6 development
sudo pacman -S plasma-framework kdevelop kate

# Rust development
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup component add rustfmt clippy rust-analyzer

# Node.js via fnm (Fast Node Manager)
curl -fsSL https://fnm.vercel.app/install | bash
fnm install 20
fnm use 20

# pnpm package manager
npm install -g pnpm

# Docker
sudo pacman -S docker docker-compose
sudo systemctl enable --now docker
sudo usermod -aG docker $USER
```

### Windows

```powershell
# Install Chocolatey package manager
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install dependencies
choco install git nodejs rust postgresql docker-desktop -y

# Install pnpm
npm install -g pnpm

# Clone and setup
git clone https://github.com/yourusername/ivibe-live.git
cd ivibe-live
cargo build --locked
pnpm install --frozen-lockfile
```

### macOS

```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rust node postgresql@15 redis docker docker-compose

# Install pnpm
npm install -g pnpm

# Start services
brew services start postgresql@15
brew services start redis

# Clone and setup
git clone https://github.com/yourusername/ivibe-live.git
cd ivibe-live
cargo build --locked
pnpm install --frozen-lockfile
```

---

## 🔌 Integrations

### IDE/Editor Support

#### Tier 1 Support (Official Plugins)
| IDE | Language | Status | Installation |
|-----|----------|--------|--------------|
| VS Code | TypeScript | ✅ Ready | Marketplace |
| IntelliJ IDEA | Rust/TS | ✅ Ready | Plugin Repository |
| Neovim | Lua | ✅ Ready | `:Lazy ivibe.nvim` |
| KDevelop | C++ | ✅ Ready | Built-in |
| Sublime Text | Python | ✅ Ready | Package Control |

#### Full IDE List (100+ Supported)
<details>
<summary>Click to expand full list</summary>

- Android Studio
- AppCode
- Aptana
- Arduino IDE
- Azure Data Studio
- Blender
- Brackets
- CLion
- Cloud9
- Cursor
- DataGrip
- DataSpell
- DBeaver
- Eclipse
- Emacs
- GoLand
- Helix
- JetBrains Fleet
- Jupyter
- Kate
- Komodo
- Light Table
- Micro
- NetBeans
- Notepad++
- Nova
- Obsidian
- PhpStorm
- PyCharm
- Rider
- RubyMine
- RustRover
- TextMate
- Unity
- Vim
- Visual Studio
- WebStorm
- Windsurf
- Wing
- Xcode
- Zed
- [... and 60+ more]

</details>

### External Services

```mermaid
graph LR
    subgraph "Version Control"
        GH[GitHub]
        GL[GitLab]
        BB[Bitbucket]
    end
    
    subgraph "Project Management"
        JI[Jira]
        AS[Asana]
        TR[Trello]
        NO[Notion]
    end
    
    subgraph "Communication"
        SL[Slack]
        DI[Discord]
        TE[Teams]
    end
    
    subgraph "Calendar"
        GC[Google Calendar]
        MC[MS Calendar]
        ZO[Zoom]
    end
    
    subgraph "Time Tracking"
        PM[Paymo]
        ZU[Zube]
        BM[Beeminder]
    end
```

### SDK Development

```typescript
// TypeScript SDK Example
import { iVibe } from '@ivibe/sdk';

// Initialize client
const client = new iVibe({
  apiKey: process.env.IVIBE_API_KEY,
  endpoint: 'https://api.ivibe.live'
});

// Capture custom event
await client.capture.customEvent({
  name: 'deployment',
  payload: {
    service: 'api-server',
    version: '2.1.0',
    environment: 'production'
  },
  tags: ['ci/cd', 'release']
});

// Subscribe to real-time events
client.subscribe('emotion.detected', (event) => {
  console.log(`Emotion: ${event.emotion} at ${event.timestamp}`);
});
```

```rust
// Rust SDK Example
use ivibe_sdk::{Client, Event};

#[tokio::main]
async fn main() {
    // Initialize client
    let client = Client::new(
        std::env::var("IVIBE_API_KEY").unwrap(),
        "https://api.ivibe.live"
    );
    
    // Capture custom event
    client.capture_event(Event {
        name: "build_complete".to_string(),
        payload: json!({
            "duration_ms": 45000,
            "success": true,
            "artifacts": 12
        }),
        tags: vec!["rust", "release"],
    }).await?;
}
```

---

## 📈 Analytics & Dashboards

### Grafana Configuration

```mermaid
graph TB
    subgraph "User Dashboards"
        PD[Productivity Dashboard]
        ED[Emotion Dashboard]
        TD[Token Usage Dashboard]
        VD[Vibe Score Dashboard]
    end
    
    subgraph "Team Dashboards"
        TM[Team Metrics]
        PR[Project Reports]
        AI[AI Spending]
    end
    
    subgraph "Data Sources"
        TS[(TimescaleDB)]
        PG[(PostgreSQL)]
        PR2[Prometheus]
    end
    
    PD --> TS
    ED --> PG
    TD --> PG
    VD --> PG
    TM --> TS
    PR --> TS
    AI --> PG
```

### Metric Categories

| Category | Metrics | Update Frequency | Tiers |
|----------|---------|------------------|-------|
| **Productivity** | Time by app/language, idle time, focus sessions | Real-time | All |
| **Terminal** | Commands run, errors fixed, success rate | Per command | All |
| **Browser** | Sites visited, research time, AI chat usage | Real-time | All |
| **Emotions** | Mood patterns, triggers, happiness index | Hourly | Elite+ |
| **AI Usage** | Tokens consumed, cost per project, provider breakdown | Daily | Essential+ |
| **Vibe Social** | Score trends, nearby vibers, connection strength | Real-time | All |
| **Location** | Places visited, time at locations, travel patterns | Hourly | Elite+ |

---

## 🔒 Security & Privacy

### Privacy Architecture

```mermaid
graph LR
    subgraph "User Device"
        LD[Local Data]
        AE[Audio Engine]
        EE[Emotion Engine]
    end
    
    subgraph "Privacy Processing"
        VEC[Vectorization]
        GEO[Geofencing]
        MASK[Data Masking]
    end
    
    subgraph "Storage"
        ENC[Encrypted DB]
        EMB[Embeddings Only]
    end
    
    subgraph "Sharing"
        SEL[Selective Export]
        PUB[Public Profile]
    end
    
    LD --> VEC
    AE --> VEC
    EE --> VEC
    VEC --> EMB
    LD --> GEO
    GEO --> MASK
    MASK --> ENC
    ENC --> SEL
    MASK --> PUB
    
    style VEC fill:#90EE90
    style GEO fill:#90EE90
    style MASK fill:#90EE90
    style EMB fill:#90EE90
```

### Compliance & Consent

| Aspect | Implementation | Compliance |
|--------|---------------|------------|
| **Data Collection** | Consent on first launch | GDPR Article 6 |
| **Audio Processing** | On-device vectorization only | CCPA compliant |
| **Location Tracking** | Geofencing for privacy zones | Location privacy laws |
| **Data Export** | Full export in standard formats | GDPR Article 20 |
| **Data Deletion** | Complete removal within 30 days | Right to be forgotten |
| **Encryption** | AES-256 at rest, TLS 1.3 in transit | Industry standard |
| **Access Control** | Row-level security in database | Zero-trust model |
| **Audit Logging** | All actions logged with timestamp | SOC 2 ready |

### Android Permissions

| Permission | Purpose | Required |
|------------|---------|----------|
| **RECORD_AUDIO** | Emotion detection, audio vectorization | Elite+ |
| **CAMERA** | Facial emotion analysis | Elite+ |
| **ACCESS_FINE_LOCATION** | Proximity detection, place intelligence | Elite+ |
| **READ_PHONE_STATE** | Call metadata for productivity | Elite+ |
| **PACKAGE_USAGE_STATS** | App usage tracking | All tiers |
| **BLUETOOTH/BLUETOOTH_ADMIN** | Vibe detection via BLE | All tiers |

---

## 🧪 Testing & Quality

### Test Strategy

```mermaid
graph LR
    subgraph "Test Types"
        UT[Unit Tests<br/>80% coverage]
        IT[Integration Tests<br/>API & Services]
        E2E[End-to-End<br/>User flows]
        PT[Performance<br/>Load testing]
        ST[Security<br/>Penetration]
    end
    
    subgraph "Test Tools"
        CARGO[cargo test]
        JEST[Jest/Vitest]
        CYP[Cypress]
        K6[k6 Load]
        OWASP[OWASP ZAP]
    end
    
    UT --> CARGO
    UT --> JEST
    E2E --> CYP
    PT --> K6
    ST --> OWASP
```

### Supply Chain Security

| Measure | Rust | TypeScript | Frequency |
|---------|------|------------|-----------|
| **Dependency Audit** | `cargo audit` | `npm audit` | Every commit |
| **License Check** | `cargo deny` | `license-checker` | Weekly |
| **Lock Files** | `Cargo.lock` | `pnpm-lock.yaml` | Always committed |
| **Vulnerability Scan** | Snyk | Snyk | Daily |
| **2FA Enforcement** | crates.io | npm registry | Required |
| **Signed Releases** | Sigstore | npm provenance | All releases |
| **SBOM Generation** | `cargo sbom` | `npm sbom` | Per release |

---

## 📅 Development Roadmap

### Milestone Timeline

```mermaid
gantt
    title iVibe.live Development Timeline
    dateFormat YYYY-MM-DD
    section Phase 1
    M1 Capture Core    :2024-01-01, 4w
    M2 Storage & ETL   :2024-01-29, 2w
    M3 Dashboards      :2024-02-12, 2w
    section Phase 2
    M4 Stripe Billing  :2024-02-26, 1w
    M5 Privacy Controls:2024-03-05, 1w
    M6 Integration SDK :2024-03-12, 3w
    section Phase 3
    M7 Mobile & Plugins:2024-04-02, 4w
    M8 Production      :2024-04-30, 2w
```

### Milestone Details

| Milestone | Duration | Key Deliverables | Team Size |
|-----------|----------|------------------|-----------|
| **M1 - Capture Core** | 4 weeks | All watchers operational, event gateway, Vibe engine | 3-4 devs |
| **M2 - Storage & ETL** | 2 weeks | Database schema, ingestion, vectorization, summaries | 2-3 devs |
| **M3 - Dashboards** | 2 weeks | Grafana setup, panels, exports, public profiles | 2 devs |
| **M4 - Stripe Billing** | 1 week | Products, prices, webhooks, portal | 1-2 devs |
| **M5 - Privacy Controls** | 1 week | Consent flow, geofencing, encryption | 2 devs |
| **M6 - Integration SDK** | 3 weeks | SDK release, 50+ integrations | 3 devs |
| **M7 - Mobile & Plugins** | 4 weeks | Android app, IDE plugins | 4 devs |
| **M8 - Production** | 2 weeks | CI/CD, monitoring, security audit | 2-3 devs |

---

## 🤝 Contributing

### Development Workflow

```mermaid
graph LR
    F[Fork Repo] --> B[Create Branch]
    B --> C[Make Changes]
    C --> T[Run Tests]
    T --> L[Run Linters]
    L --> CO[Commit]
    CO --> P[Push]
    P --> PR[Pull Request]
    PR --> R[Review]
    R --> M[Merge]
```

### Contribution Guidelines

1. **Code Style**
   - Rust: Follow `rustfmt` and `clippy` recommendations
   - TypeScript: ESLint with Prettier
   - Commits: Conventional Commits format

2. **Testing Requirements**
   - Unit tests for all new functions
   - Integration tests for API endpoints
   - 80% code coverage minimum

3. **Security**
   - Run `cargo audit` before commits
   - No hardcoded secrets
   - Follow OWASP guidelines

4. **Documentation**
   - Update relevant docs
   - Add JSDoc/RustDoc comments
   - Include examples

---

## 📖 Glossary

| Term | Definition |
|------|------------|
| **Vibe Score** | Compatibility rating (1-1000) between users based on shared interests, emotions, and activities |
| **Vibe-by** | A proximity detection event when two iVibe users are near each other |
| **Viber** | An iVibe user who has the app installed and proximity detection enabled |
| **My Vibers** | Your list of connected iVibe users (friends) |
| **Vectorization** | Converting audio/text to mathematical embeddings for privacy-preserving analysis |
| **Watcher** | A background service that captures specific types of activity data |
| **Event Gateway** | Central API endpoint that receives and authenticates all capture events |
| **Materialised View** | Pre-computed database view for fast dashboard queries |
| **Row-Level Security** | Database feature ensuring users can only access their own data |
| **Geofencing** | Creating geographic boundaries where tracking is automatically disabled |
| **Token** | Unit of AI API usage (input/output) tracked for cost analysis |
| **Embedding** | High-dimensional vector representation of text or audio |
| **Hypertable** | TimescaleDB table optimized for time-series data |

---

## 📄 License

This software is proprietary and confidential. All rights reserved.

© 2024 iVibe.live - Redefining Social Connection

For licensing inquiries, contact: license@ivibe.live

---

<div align="center">
  
  **Built with ❤️ using Rust and TypeScript**
  
  [Website](https://ivibe.live) • [Documentation](https://docs.ivibe.live) • [Support](https://support.ivibe.live)
  
  [![Twitter](https://img.shields.io/twitter/follow/ivibelive?style=social)](https://twitter.com/ivibelive)
  [![Discord](https://img.shields.io/discord/123456789?style=social)](https://discord.gg/ivibe)
  [![GitHub Stars](https://img.shields.io/github/stars/ivibe/ivibe-live?style=social)](https://github.com/ivibe/ivibe-live)
  
</div>
