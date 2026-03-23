# 🐦 BetterPigeon

> **Stay connected. Leave the cage.**

BetterPigeon is a free, open-source messaging hub for Signal, WhatsApp, Instagram, Facebook, and Matrix. One calm, customizable interface — no algorithms, no ads, no data harvesting. Your chats stay private, in a healthy environment. Stay connected without sacrificing your privacy, energy, or time to profit-driven platforms.

---

## 💡 Why BetterPigeon?

Your friends are scattered. Signal, WhatsApp, Instagram, Facebook, Matrix — each platform a walled garden, each app engineered not for communication, but for capturing your attention and monetizing your data.

**BetterPigeon tears down those walls.**

Inspired by the carrier pigeons used by soldiers for discreet, long-range communication, BetterPigeon brings that same philosophy to modern messaging: reliable, private, and entirely under your control.

- **One app** to reach everyone, wherever they are
- **One interface**, calm and fully customizable — designed for humans, not engagement metrics
- **Zero compromise** on security, privacy, or freedom

Because staying in touch with the people you love shouldn't mean surrendering your mind and your data to profit-driven platforms.

---

## ✨ Features (Roadmap)

### Core
- [ ] Unified inbox — all your conversations in one place
- [ ] Fully customizable interface (themes, layout, fonts)
- [ ] Real-time notifications across all platforms
- [ ] Cross-platform support: Linux, Windows, macOS

### Messaging Integrations
- [ ] Signal
- [ ] Matrix
- [ ] WhatsApp
- [ ] Instagram Direct
- [ ] Facebook Messenger

### Privacy & Security
- [ ] 100% local — no cloud, no third-party servers
- [ ] AES-256 encrypted session storage
- [ ] Automatic session reconnection (encrypted local store)
- [ ] No telemetry, no analytics, no tracking — ever
- [ ] Secure credential management (no plaintext tokens)

### Developer
- [ ] Modular architecture (one folder per platform)
- [ ] Plugin system for future integrations
- [ ] Full test coverage (unit + integration)
- [ ] Audit-ready codebase

---

## 🔒 Security Principles

BetterPigeon is built with security as a first-class citizen, not an afterthought.

| Principle | Implementation |
|---|---|
| **Local-first** | All data lives on your device. Nothing leaves it. |
| **Encrypted storage** | Sessions and messages encrypted with AES-256 + PBKDF2 key derivation |
| **No proprietary deps** | 100% open-source dependency chain |
| **Auditable** | Every line of code is readable, documented, and reviewable |
| **Minimal attack surface** | Strict Tauri sandbox, no unnecessary permissions |
| **No plaintext secrets** | Tokens and credentials never appear in logs or code |

---

## 🛠️ Tech Stack

| Layer | Technology | Role |
|---|---|---|
| **Shell** | [Tauri](https://tauri.app) (Rust) | Secure cross-platform desktop shell |
| **Frontend** | [Svelte](https://svelte.dev) + TypeScript | UI — minimal, fast, auditable |
| **Styling** | Tailwind CSS | Customizable design system |
| **Storage** | SQLite + SQLCipher | Encrypted local database |
| **Signal** | [signal-cli](https://github.com/AsamK/signal-cli) | Signal bridge (bundled) |
| **Matrix** | [matrix-js-sdk](https://github.com/matrix-org/matrix-js-sdk) | Matrix official SDK |
| **WhatsApp** | [Baileys](https://github.com/WhiskeySockets/Baileys) | WhatsApp Web reverse-engineered |
| **Instagram** | [instagram-private-api](https://github.com/dilame/instagram-private-api) | Instagram private API |
| **Facebook** | [facebook-chat-api](https://github.com/Schmavery/facebook-chat-api) | Facebook Messenger bridge |

---

## 🏗️ Architecture

```
betterpigeon/
├── src-tauri/              # Rust shell (Tauri)
│   └── src/
│       ├── main.rs         # Entry point
│       ├── crypto.rs       # AES-256 encryption module
│       └── storage.rs      # SQLite interface
│
├── frontend/               # Svelte UI
│   └── src/
│       ├── App.svelte      # Root component
│       ├── lib/
│       │   ├── inbox/      # Unified inbox view
│       │   ├── sidebar/    # Platform switcher
│       │   └── settings/   # User preferences
│       └── stores/         # Svelte state management
│
├── bridge/                 # Messaging platform modules
│   ├── signal/             # signal-cli wrapper
│   ├── matrix/             # matrix-js-sdk integration
│   ├── whatsapp/           # Baileys integration
│   ├── instagram/          # instagram-private-api
│   └── facebook/           # facebook-chat-api
│
└── docs/                   # Documentation
```

### Data flow

```
User action (Svelte UI)
      │
      ▼ IPC (Tauri secure bridge)
Rust core (auth, crypto, storage)
      │
      ▼
Platform bridge (JS/TS module)
      │
      ▼
External platform (Signal, Matrix, etc.)
```

---

## ⚠️ Legal Notice

BetterPigeon uses **official APIs** for Matrix and Signal (via signal-cli).

For WhatsApp, Instagram, and Facebook, BetterPigeon uses **unofficial, reverse-engineered APIs**. These are not endorsed by Meta. Using them may violate those platforms' Terms of Service.

- This software is primarily intended for **personal use**
- You use these integrations **at your own risk**
- Account bans, while rare for personal use, are possible
- BetterPigeon contributors are not liable for any consequences

---

## 🚀 Getting Started

> 🚧 **Not yet available** — project is in early development.

Installation instructions will be added as the project matures. Planned distribution formats:

- `.deb` for Debian/Ubuntu
- `.AppImage` for Linux (universal)
- `.exe` installer for Windows
- `.dmg` for macOS

---

## 🧪 Development

### Prerequisites

- Rust + Cargo (`rustup`)
- Node.js 20+ (via `nvm`)
- Tauri CLI (`cargo install tauri-cli`)
- Debian/Ubuntu: `libwebkit2gtk-4.1-dev`, `libssl-dev`

### Setup

```bash
git clone https://github.com/YOUR_USERNAME/betterpigeon.git
cd betterpigeon
cd frontend && npm install
cd ../bridge && npm install
cargo tauri dev
```

---

## 🤝 Contributing

BetterPigeon is built in the open. Contributions, feedback, and audits are welcome.

- **Report bugs** → [Issues](https://github.com/YOUR_USERNAME/betterpigeon/issues)
- **Suggest features** → [Discussions](https://github.com/YOUR_USERNAME/betterpigeon/discussions)
- **Security vulnerabilities** → Please report privately via GitHub Security Advisories

### Commit convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
feat:      New feature
fix:       Bug fix
security:  Security patch
docs:      Documentation
chore:     Maintenance
```

---

## 📄 License

**GNU General Public License v3.0**

BetterPigeon is free software: you can redistribute it and/or modify it under the terms of the GPL-3.0. See [LICENSE](LICENSE) for details.

This license guarantees that BetterPigeon will always remain free, open, and auditable — and that no one can ever take this code and make it proprietary.

---

<div align="center">

**🐦 BetterPigeon**

*Stay connected. Leave the cage.*

[Report a bug](https://github.com/YOUR_USERNAME/betterpigeon/issues) · [Request a feature](https://github.com/YOUR_USERNAME/betterpigeon/discussions) · [Read the docs](docs/)

</div>


#README creat by an AI, It will be remade by hand once the project is further along.

