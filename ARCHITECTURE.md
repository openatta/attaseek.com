# AttaSeek.com — 架构文档

## 1. 定位

**attaseek.com** 是 AttaSeek 产品族的官方对外网站，承载产品展示、技术介绍、文档入口、社区运营等职能。面向潜在用户、开发者、合作伙伴。

### 产品族全景

```
AttaSeek（主产品 — 桌面 AI Agent 工作台）
  ├── AttaCode   CLI AI Agent 引擎（Rust 重写，独立发布）
  ├── AttaGo     移动端（Flutter，远程操控）
  └── ClawPod     桌面端（Tauri + Rust，安装/管理/Bridge 接入）
```

> attaseek.com 是这 4 个产品的"官网"，不是功能平台。各产品有自己的分发/文档渠道，本网站做聚合入口。

## 2. 技术架构

```
┌─────────────────────────────────────────────────────────┐
│                    attaseek.com                         │
│                                                         │
│  Nginx (TLS: Let's Encrypt, auto-renew via certbot)     │
│  ├── /         → /opt/attaseek/frontend/dist/ (Vue 3)  │
│  └── /api/*    → http://127.0.0.1:3001 (Rust backend)  │
│                                                         │
│  /opt/attaseek/                                         │
│  ├── frontend/dist/     Vue 3 SPA (static)              │
│  ├── backend            Rust binary (axum)              │
│  ├── backend.env        runtime env                     │
│  └── backend.service    systemd unit                    │
└─────────────────────────────────────────────────────────┘
```

### 分层

| 层 | 技术 | 说明 |
|---|---|---|
| **TLS 终结** | Nginx + Let's Encrypt certbot | 与 jiantex.com / asvoxel.com 同机部署，共享 nginx 实例 |
| **静态前端** | Vue 3 + Vite | SPA，`npm run build` → `dist/`，nginx 直接 serve |
| **后端 API** | Rust (axum) | 轻量 HTTP server，serve 静态 + 少量 API（联系表单、新闻订阅） |
| **进程管理** | systemd | `attaseek-backend.service`，开机自启，crash 自动重启 |

### 不需要的东西

- **数据库**：纯展示站，无需持久化（联系表单可发邮件或写入本地 SQLite 作为轻量选择）
- **Docker**：不用。直接 systemd + nginx。
- **Redis / 队列**：不需要。
- **SSR / Nuxt**：不需要。纯静态 SPA 即可，SEO 关键页面可用预渲染或 nginx 静态 fallback。

## 3. 编译与构建

### Rust 后端

```
macOS (aarch64) 本机编译
       │
       ├── target: x86_64-unknown-linux-musl
       ├── linker: zig 0.16.0 (cargo-zigbuild 或 .cargo/config.toml)
       └── 产出: 单个静态二进制 → rsync 到 VPS
```

**zig 交叉编译配置** (`.cargo/config.toml`):

```toml
[target.x86_64-unknown-linux-musl]
linker = "zig"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

**编译命令**:

```sh
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl -p attaseek-web
```

### Vue 3 前端

```sh
cd frontend
npm install
npm run build    # → dist/
```

### 部署

```sh
# 一次性脚本
rsync -avz target/x86_64-unknown-linux-musl/release/attaseek-web root@attaseek.com:/opt/attaseek/backend
rsync -avz frontend/dist/ root@attaseek.com:/opt/attaseek/frontend/dist/
ssh root@attaseek.com "systemctl restart attaseek-backend && systemctl reload nginx"
```

## 4. Nginx 配置（规划）

参考同机 jiantex.com 和 asvoxel.com 的模式：

```nginx
server {
    server_name attaseek.com www.attaseek.com;

    root /opt/attaseek/frontend/dist;
    index index.html;

    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Static assets with long cache
    location /assets/ {
        add_header Cache-Control "public, max-age=31536000, immutable";
        try_files $uri =404;
    }

    # API proxy
    location /api/ {
        proxy_pass http://127.0.0.1:3001;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Health check
    location /health {
        proxy_pass http://127.0.0.1:3001;
    }

    listen [::]:443 ssl ipv6only=on;
    listen 443 ssl;
    ssl_certificate /etc/letsencrypt/live/attaseek.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/attaseek.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}

server {
    if ($host = www.attaseek.com) {
        return 301 https://$host$request_uri;
    }
    if ($host = attaseek.com) {
        return 301 https://$host$request_uri;
    }

    listen 80;
    listen [::]:80;
    server_name attaseek.com www.attaseek.com;
    return 404;
}
```

## 5. 网站结构

### 导航

```
首页 | 产品 | 技术 | 文档 | 社区 | 关于
```

### 页面路由

| 路由 | 内容 |
|---|---|
| `/` | 首页 Hero + 产品卡片 + 特性亮点 + CTA |
| `/products/` | 产品总览页 |
| `/products/attaseek` | AttaSeek 桌面工作台详情 |
| `/products/attacode` | AttaCode CLI 引擎详情 |
| `/products/attago` | AttaGo 移动端详情 |
| `/products/clawpod` | ClawPod 桌面端详情 |
| `/technology` | 技术栈、协议架构、安全模型 |
| `/docs` | 文档入口（链接到各产品文档） |
| `/community` | 社区（GitHub / Discord / 博客） |
| `/about` | 团队、愿景、联系方式 |

### 首页内容规划

```
Hero:
  标题: "AttaSeek — AI Agent 工作台"
  副标题: "在自己的设备上运行、审查、引导 AI Agent。"
  CTA: [探索产品] [查看文档]

三大支柱:
  ┌──────────────┬──────────────┬──────────────┐
  │  AttaSeek    │  AttaCode    │  AttaGo      │
  │  桌面工作台   │  CLI 引擎    │  移动操控     │
  │  Electron    │  Rust 重写   │  Flutter     │
  │  React + TS  │  42+ 工具    │  iOS/Android │
  └──────────────┴──────────────┴──────────────┘

特性区:
  - 本地优先，隐私安全
  - 多 Agent 协同
  - 跨端远程操控
  - 开放协议栈

技术栈展示:
  Rust · Flutter · Tauri · Electron · WebRTC · Proto
```

## 6. 目录结构

```
attaseek.com/
├── ARCHITECTURE.md          ← 本文件
├── README.md                ← 项目概览 + 本地开发指南
├── CLAUDE.md                ← Claude 项目指令
├── Cargo.toml               ← Rust workspace
├── crates/
│   └── attaseek-web/        ← axum HTTP server
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs      ← entrypoint (serve static + API routes)
│           ├── config.rs    ← env/config loading
│           ├── routes/
│           │   ├── mod.rs
│           │   ├── health.rs
│           │   ├── contact.rs
│           │   └── newsletter.rs
│           └── static/      ← embed frontend dist at build time (optional)
├── frontend/                ← Vue 3 project (create-vue scaffold)
│   ├── package.json
│   ├── vite.config.ts
│   ├── index.html
│   ├── src/
│   │   ├── main.ts
│   │   ├── App.vue
│   │   ├── router/
│   │   ├── components/
│   │   ├── views/
│   │   └── assets/
│   └── dist/                ← build output (gitignored)
├── scripts/
│   ├── build.sh             ← zig cross-compile backend + npm build frontend
│   └── deploy.sh            ← rsync to /opt/attaseek
├── deploy/
│   ├── attaseek.com.nginx   ← nginx vhost config
│   └── attaseek-backend.service  ← systemd unit
└── .github/
    └── workflows/           ← CI (optional)
```

## 7. systemd 服务

```ini
[Unit]
Description=AttaSeek Web Backend
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/attaseek
EnvironmentFile=/opt/attaseek/backend.env
ExecStart=/opt/attaseek/backend
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

## 8. 服务器信息

| 项 | 值 |
|---|---|
| **OS** | Ubuntu 22.04 LTS (Jammy) |
| **架构** | x86_64 |
| **Web Server** | Nginx (shared with jiantex.com, asvoxel.com) |
| **TLS** | Let's Encrypt via certbot (auto-renew timer) |
| **部署路径** | `/opt/attaseek/` |
| **同机站点** | `/opt/jiandu/` (jiantex.com), `/var/www/asvoxel.com/` (asvoxel.com) |
