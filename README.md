# ⚡ Quick Phrases 快捷话术

桌面话术管理工具 — 快速存储、搜索、一键复制常用话术。

## ✨ 功能

- 📋 **话术管理** — 分类管理话术，支持标题 + 文本内容 + 标签
- 🔍 **快速搜索** — Ctrl+K 唤起命令面板，即搜即用
- 📋 **一键复制** — 点击卡片或回车即可复制到剪贴板
- ⭐ **收藏** — 常用话术收藏，快速访问
- 🖼️ **图文混排** — 支持话术附带图片
- 🌙 **深色模式** — 浅色 / 深色主题切换
- 🌍 **多语言** — 简体中文 / 繁體中文 / English
- 📌 **系统托盘** — 最小化至托盘，后台常驻
- 🚀 **开机自启** — 可设置随系统启动
- 💾 **本地存储** — JSON 文件存储，数据完全可控

## 🖥️ 截图

![screenshot](README\js1.png)
![screenshot](README\js2.png)

## 🏗️ 技术栈

| 层 | 技术 |
|----|------|
| 桌面框架 | Tauri v2 |
| 前端 | Vue 3 + Vite + TypeScript |
| UI 组件 | Naive UI |
| 图标 | Iconify (Remix Icon) |
| 状态管理 | Pinia |
| 国际化 | vue-i18n |
| 存储 | JSON 文件 + Rust 文件操作 |

## 📦 开发

### 环境要求

- [Rust](https://www.rust-lang.org/) >= 1.70
- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- Windows 10+

### 快速开始

```bash
# 克隆项目
git clone https://github.com/yourname/quick-phrases.git
cd quick-phrases

# 安装依赖
pnpm install

# 启动开发模式
cargo tauri dev

# 生产构建
cargo tauri build
```

## 📁 项目结构

```
quick-phrases/
├── src/                        # Vue 前端
│   ├── components/             # 通用组件
│   │   ├── CategorySidebar.vue # 侧边栏
│   │   ├── PhraseCard.vue      # 话术卡片
│   │   ├── PhraseEditor.vue    # 话术编辑器
│   │   ├── SearchBar.vue       # 搜索栏
│   │   ├── CommandPalette.vue  # 命令面板 (Ctrl+K)
│   │   ├── TitleBar.vue        # 自定义标题栏
│   │   └── CopyToast.vue       # 复制提示
│   ├── views/                  # 页面视图
│   │   ├── HomeView.vue        # 主页
│   │   └── SettingsView.vue    # 设置页
│   ├── stores/                 # Pinia 状态管理
│   ├── composables/            # 组合式函数
│   ├── locales/                # 国际化语言文件
│   └── types/                  # TypeScript 类型
├── src-tauri/                  # Rust 后端
│   ├── src/
│   │   ├── commands/           # Tauri 命令
│   │   ├── models/             # 数据模型
│   │   └── services/           # 业务逻辑
│   └── icons/                  # 应用图标
└── package.json
```

## 📄 开源协议

[MIT](LICENSE)
