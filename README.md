# attaseek.com

AttaSeek 产品族官方网站。展示 AttaSeek（桌面 AI Agent 工作台）、AttaCode（CLI 引擎）、AttaGo（移动端）、ClawPod（桌面端）。

## 技术栈

| 层 | 选型 |
|---|---|
| 后端 | Rust + axum (HTTP server + API) |
| 前端 | Vue 3 + Vite (SPA) |
| 样式 | Tailwind CSS |
| 编译 | macOS 本机 zig 交叉编译 → Linux x86_64 musl |
| 部署 | Nginx + Let's Encrypt → `/opt/attaseek/` |
| 进程管理 | systemd |

## 本地开发

```sh
# 后端
cd /Users/xbits/Workspace/Atta/attaseek.com
cargo run -p attaseek-web

# 前端（另开 terminal）
cd frontend
npm install
npm run dev
```

## 构建

```sh
# 全量构建（后端 zig 交叉编译 + 前端）
./scripts/build.sh
```

## 部署

```sh
# 一键部署到 attaseek.com VPS
./scripts/deploy.sh
```

## 目录结构

```
attaseek.com/
├── ARCHITECTURE.md          # 架构文档
├── README.md                # 本文件
├── CLAUDE.md                # Claude 项目指令
├── Cargo.toml               # Rust workspace
├── crates/
│   └── attaseek-web/        # axum HTTP server
├── frontend/                # Vue 3 SPA
│   ├── src/
│   │   ├── views/           # 页面组件（Home / Products / Technology / …）
│   │   ├── components/      # 通用组件（NavBar / Footer / ProductCard / …）
│   │   └── router/          # Vue Router 配置
│   └── dist/                # 构建产物（gitignored）
├── scripts/
│   ├── build.sh             # 编译脚本
│   └── deploy.sh            # 部署脚本
└── deploy/
    ├── attaseek.com.nginx   # Nginx 虚拟主机配置
    └── attaseek-backend.service  # systemd unit
```

## 网站结构

```
首页           /                    Hero + 产品卡片 + 特性
产品总览       /products            AttaSeek / AttaCode / AttaGo / ClawPod
产品详情       /products/:slug      单个产品详情页
技术           /technology          技术栈、架构、安全模型
文档           /docs                文档入口（链接到各产品文档站）
社区           /community           GitHub / Discord / 博客
关于           /about               团队、愿景、联系
```

## 服务器

| 项 | 值 |
|---|---|
| **域名** | attaseek.com / www.attaseek.com |
| **OS** | Ubuntu 22.04 LTS x86_64 |
| **部署路径** | `/opt/attaseek/` |
| **后端端口** | `127.0.0.1:3001` |
| **TLS** | Let's Encrypt (certbot auto-renew) |
| **同机站点** | jiantex.com, asvoxel.com |
