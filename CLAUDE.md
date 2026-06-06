# attaseek.com

AttaSeek 产品族官方网站 —— 后端 Rust (axum) + 前端 Vue 3 SPA，部署于 attaseek.com VPS。

## 边界

- 本项目是 **独立网站项目**，不是 Atta 主 monorepo 的一部分
- 本项目的角色是**产品展示/运营官网**，不承载产品功能（产品功能在各自的代码仓库）
- 上级目录 `/Users/xbits/Workspace/Atta/` 下的其他项目（AttaCode、AttaGo、ClawPod、AttaSeek）的内容仅供**文案/品牌参考**，本项目不依赖它们的代码
- 产品族关系：
  - **AttaSeek** — 桌面 AI Agent 工作台（Electron + React，主产品，位于 `../AttaSeek/`）
  - **AttaCode** — CLI AI Agent 引擎（Rust 重写 Claude Code，位于 `../AttaCode/`）
  - **AttaGo** — 移动端远程操控（Flutter，位于 `../AttaGo/`）
  - **ClawPod** — 桌面端管理工具（Tauri + Rust，位于 `../ClawPod/`）

## 技术栈

| 层 | 选型 |
|---|---|
| 后端 | Rust 1.95+ (stable), axum 0.8, tokio |
| 前端 | Vue 3, Vite, Vue Router, Tailwind CSS |
| 编译 | macOS aarch64 本机 zig 0.16.0 交叉编译 → Linux x86_64 musl |
| 部署 | rsync → VPS `/opt/attaseek/`，Nginx + Let's Encrypt，systemd |
| 样式 | Tailwind CSS（前端），无 CSS 预处理器 |

> 禁止使用 Docker 编译或部署。编译在本机完成，部署用 rsync。

## 项目结构

```
attaseek.com/
├── CLAUDE.md                     ← 本文件
├── ARCHITECTURE.md               ← 架构文档（详细）
├── README.md                     ← 项目概览
├── Cargo.toml                    ← Rust workspace root
├── .cargo/
│   └── config.toml               ← zig linker 配置
├── crates/
│   └── attaseek-web/             ← axum HTTP server
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           ← entrypoint
│           ├── lib.rs            ← app state, router mount
│           ├── config.rs         ← env / config
│           └── routes/
│               ├── mod.rs
│               ├── health.rs     ← /health
│               ├── contact.rs    ← POST /api/contact
│               └── newsletter.rs ← POST /api/newsletter
├── frontend/                     ← Vue 3 SPA（create-vue 脚手架）
│   ├── package.json
│   ├── vite.config.ts
│   ├── index.html
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── router/index.ts      ← Vue Router 配置
│   │   ├── views/               ← 页面组件
│   │   │   ├── HomeView.vue
│   │   │   ├── ProductsView.vue
│   │   │   ├── ProductDetailView.vue
│   │   │   ├── TechnologyView.vue
│   │   │   ├── DocsView.vue
│   │   │   ├── CommunityView.vue
│   │   │   └── AboutView.vue
│   │   ├── components/          ← 通用组件
│   │   │   ├── NavBar.vue
│   │   │   ├── Footer.vue
│   │   │   ├── ProductCard.vue
│   │   │   └── HeroSection.vue
│   │   └── assets/              ← 图片、图标等
│   └── dist/                    ← 构建产物 (gitignored)
├── scripts/
│   ├── build.sh                 ← 编译后端 (zig cross) + 构建前端
│   └── deploy.sh                ← rsync 部署
├── deploy/
│   ├── attaseek.com.nginx       ← nginx vhost config
│   └── attaseek-backend.service ← systemd unit
└── .gitignore
```

## 本地开发

```sh
# 后端
cd /Users/xbits/Workspace/Atta/attaseek.com
cargo run -p attaseek-web

# 前端（另开 terminal）
cd /Users/xbits/Workspace/Atta/attaseek.com/frontend
npm run dev
```

**本地开发时**，前端 `npm run dev` 默认监听 `localhost:5173`（Vite dev server），后端监听 `localhost:3001`。前端 Vite config 应配置 proxy 将 `/api` 转发到后端。

## 交叉编译

```sh
# 添加 target（一次性）
rustup target add x86_64-unknown-linux-musl

# 使用 cargo-zigbuild 交叉编译
cargo zigbuild --release --target x86_64-unknown-linux-musl -p attaseek-web
```

> 本机已安装 cargo-zigbuild (`~/.cargo/bin/cargo-zigbuild`) 和 zig 0.16.0 (`/opt/homebrew/bin/zig`)。

## 部署

```sh
# 全量构建 + 部署
./scripts/build.sh && ./scripts/deploy.sh
```

部署到 `root@attaseek.com:/opt/attaseek/`（免密 SSH）。部署后重启 backend 和 reload nginx。

## 服务器环境

| 项 | 值 |
|---|---|
| **域名** | attaseek.com / www.attaseek.com |
| **OS** | Ubuntu 22.04 LTS x86_64 |
| **Web Server** | Nginx (shared with jiantex.com, asvoxel.com) |
| **部署路径** | `/opt/attaseek/` |
| **后端端口** | `127.0.0.1:3001` |
| **TLS** | Let's Encrypt via certbot (auto-renew systemd timer) |
| **同机参考** | jiantex.com (`/opt/jiandu/`, proxy to 8082), asvoxel.com (`/var/www/asvoxel.com/`, proxy to 8080) |

### 获取服务器 nginx 配置参考

```sh
ssh root@attaseek.com "cat /etc/nginx/sites-enabled/jiantex.com"
ssh root@attaseek.com "cat /etc/nginx/sites-enabled/asvoxel.com"
```

### 新增 TLS 证书

```sh
ssh root@attaseek.com "certbot --nginx -d attaseek.com -d www.attaseek.com"
```

### systemd 管理

```sh
# 安装 service 文件后
ssh root@attaseek.com "systemctl daemon-reload && systemctl enable --now attaseek-backend"
ssh root@attaseek.com "systemctl status attaseek-backend"
ssh root@attaseek.com "journalctl -u attaseek-backend -f"
```

## 网站内容

### 导航栏

```
首页 | 产品 | 技术 | 文档 | 社区 | 关于
```

### 路由表

| 路径 | 视图 | 说明 |
|---|---|---|
| `/` | HomeView | Hero + 产品卡片 + 特性 |
| `/products` | ProductsView | 产品总览 |
| `/products/attaseek` | ProductDetailView | AttaSeek 桌面工作台 |
| `/products/attacode` | ProductDetailView | AttaCode CLI 引擎 |
| `/products/attago` | ProductDetailView | AttaGo 移动端 |
| `/products/clawpod` | ProductDetailView | ClawPod 桌面端 |
| `/technology` | TechnologyView | 技术栈、架构、安全 |
| `/docs` | DocsView | 文档入口 |
| `/community` | CommunityView | GitHub / Discord / 博客 |
| `/about` | AboutView | 团队、愿景、联系 |

### 产品文案来源

- AttaSeek：`../AttaSeek/CLAUDE.md`, `../AttaSeek/AGENTS.md`
- AttaCode：`../AttaCode/CLAUDE.md`, `../AttaCode/ATTA.md`
- AttaGo：`../AttaGo/CLAUDE.md`
- ClawPod：`../ClawPod/CLAUDE.md`
- 整体品牌：`../CLAUDE.md`, `../docs/ARCHITECTURE.md`

> 文案编写时参考上述文件以保持品牌一致性，但本项目独立，不引用其代码。

## 代码风格

### Rust

- `rustfmt` + `clippy -D warnings`
- 错误用 `thiserror`，应用层用 `anyhow`
- 异步统一 `tokio`
- 日志用 `tracing` + `tracing-subscriber`（JSON 格式）
- 配置用 `figment`（TOML + env）
- **不记录明文业务内容或密钥**

### Vue 3 / TypeScript

- Composition API + `<script setup>` 语法
- Vue Router 4，lazy loading 路由
- Tailwind CSS 原子类为主
- ESLint + Prettier
- TypeScript 严格模式
- 组件命名 PascalCase，文件名 PascalCase

### 通用

- 提交信息英文，格式：`attaseek.com: <verb> <subject>`
- 不上传 `dist/`、`target/`、`node_modules/`
- API 路由前缀 `/api/v1/`

## 设计约束

1. **不引入数据库**：展示站不需要。联系表单可写本地 SQLite（better-sqlite3 同模式）或直接发邮件。
2. **不引入 auth 体系**：纯公开站，无用户系统。后台管理（如有需要）后续再议。
3. **SPA 为主，不做 SSR**：SEO 关键页通过预渲染或 nginx 静态 fallback 覆盖。
4. **响应式**：移动端友好，Tailwind 断点 `sm/md/lg/xl`。
5. **暗色模式**：支持（Tailwind `dark:` variant），默认跟随系统。
6. **轻量优先**：图片压缩、字体子集化、无重型第三方脚本。
