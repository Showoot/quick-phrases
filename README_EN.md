# Quick Phrases
Desktop Snippet Manager — Save, search and copy your frequently used text snippets in one click.

## ✨ Features
- 📋 **Snippet Management** – Organize snippets by categories, with support for title, content text and tags
- 🔍 **Instant Search** – Open command palette via Ctrl+K for instant lookup
- 📋 **One-Click Copy** – Copy text to clipboard by clicking the snippet card or pressing Enter
- ⭐ **Favorites** – Star frequently used snippets for quick access
- 🖼️ **Image & Text Support** – Attach images alongside text content in snippets
- 🌙 **Dark Mode** – Toggle between light and dark color schemes
- 🌍 **Multi-Language Support** – Simplified Chinese, Traditional Chinese and English
- 📌 **System Tray Minimize** – Minimize app to system tray and run in background
- 🚀 **Auto Startup** – Option to launch the app automatically when system boots
- 💾 **Local File Storage** – All data saved in local JSON files; you retain full control of your data

## 🖥️ Screenshots

![screenshot](https://github.com/Showoot/quick-phrases/blob/main/README/js1.png)
![screenshot](https://github.com/Showoot/quick-phrases/blob/main/README/js2.png)

## 🏗️ Tech Stack
| Layer | Technology |
|-------|------------|
| Desktop Framework | Tauri v2 |
| Frontend | Vue 3 + Vite + TypeScript |
| UI Library | Naive UI |
| Icon Set | Iconify (Remix Icon) |
| State Management | Pinia |
| Internationalization | vue-i18n |
| Data Persistence | Local JSON files + Rust file I/O |

## 📦 Development
### Prerequisites
- Rust >= 1.70 ([Official Download](https://www.rust-lang.org/))
- Node.js >= 18 ([Official Download](https://nodejs.org/))
- pnpm >= 8 ([Official Download](https://pnpm.io/))
- Windows 10 or newer

### Quick Start

```bash
# Clone repository
git clone https://github.com/Showoot/quick-phrases.git
cd quick-phrases

# Install frontend dependencies
pnpm install

# Launch development environment
cargo tauri dev

# Build production release installer
cargo tauri build
```

## 📁 Project Structure

```
quick-phrases/
├── src/                        # Vue Frontend Source Code
│   ├── components/             # Reusable UI Components
│   │   ├── CategorySidebar.vue # Category Sidebar
│   │   ├── PhraseCard.vue      # Snippet Display Card
│   │   ├── PhraseEditor.vue    # Snippet Edit Panel
│   │   ├── SearchBar.vue       # Global Search Input
│   │   ├── CommandPalette.vue  # Ctrl+K Search Popup
│   │   ├── TitleBar.vue        # Custom Window Title Bar
│   │   └── CopyToast.vue       # Copy Success Notification
│   ├── views/                  # Main Page Views
│   │   ├── HomeView.vue        # Main Snippet List Page
│   │   └── SettingsView.vue    # Application Settings Page
│   ├── stores/                 # Pinia Global State Stores
│   ├── composables/            # Vue Composables / Reusable Hooks
│   ├── locales/                # Multilingual Translation Files
│   └── types/                  # TypeScript Type Definitions
├── src-tauri/                  # Rust Backend Source Code
│   ├── src/
│   │   ├── commands/           # Tauri Interop Commands
│   │   ├── models/             # Core Data Models
│   │   └── services/           # Backend Business Logic
│   └── icons/                  # Application Icon Assets
└── package.json
```

## 📄 License

[MIT](LICENSE)