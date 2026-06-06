#!/bin/bash
# attaseek.com — 部署（参考 asfly.io 方案）
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Homebrew Rust 跟 rustup 冲突 — 必须优先用 rustup 的 cargo/rustc
RUSTUP_BIN="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin"
export PATH="$RUSTUP_BIN:$PATH"

SSH_HOST="root@attaseek.com"
REMOTE_DIR="/opt/attaseek"
TARGET="x86_64-unknown-linux-musl"
BINARY="$ROOT/target/$TARGET/release/attaseek-web"

echo "==> [1/5] 预检..."
if ! command -v zig &>/dev/null; then
    echo "ERROR: zig not found. Install: brew install zig"
    exit 1
fi

echo "==> [2/5] 传输文件到 $SSH_HOST:$REMOTE_DIR"
ssh "$SSH_HOST" "mkdir -p $REMOTE_DIR/frontend/dist"

# binary via gzip pipe
gzip -c "$BINARY" | ssh "$SSH_HOST" "gunzip -c > $REMOTE_DIR/backend && chmod +x $REMOTE_DIR/backend"
echo "    ✓ backend"

# frontend dist
rsync -az --delete --no-owner --no-group "$ROOT/frontend/dist/" "$SSH_HOST:$REMOTE_DIR/frontend/dist/"
echo "    ✓ frontend"

echo "==> [3/6] 上传 deploy 配置..."
ssh "$SSH_HOST" "mkdir -p $REMOTE_DIR/deploy"
scp "$ROOT/deploy/attaseek-backend.service" "$SSH_HOST:$REMOTE_DIR/deploy/"
scp "$ROOT/deploy/attaseek.com.nginx" "$SSH_HOST:$REMOTE_DIR/deploy/"
echo "    ✓"

echo "==> [4/6] 安装 deploy 配置（首次）..."
ssh "$SSH_HOST" bash <<'ENDSSH'
if [ ! -f /etc/systemd/system/attaseek-backend.service ]; then
    cp /opt/attaseek/deploy/attaseek-backend.service /etc/systemd/system/
    systemctl daemon-reload
    systemctl enable attaseek-backend
    echo "    ✓ systemd service installed"
fi
if [ ! -f /etc/nginx/sites-enabled/attaseek.com ]; then
    cp /opt/attaseek/deploy/attaseek.com.nginx /etc/nginx/sites-available/attaseek.com
    ln -sf /etc/nginx/sites-available/attaseek.com /etc/nginx/sites-enabled/
    echo "    ✓ nginx config installed"
fi
ENDSSH

echo "==> [5/6] 创建 backend.env（如不存在）..."
ssh "$SSH_HOST" "test -f $REMOTE_DIR/backend.env || echo 'ATTASEEK_FRONTEND_DIR=$REMOTE_DIR/frontend/dist' > $REMOTE_DIR/backend.env"
echo "    ✓"

echo "==> [6/6] 重启服务..."
ssh "$SSH_HOST" "systemctl restart attaseek-backend && sleep 1 && systemctl reload nginx"
echo "    ✓"

sleep 1
echo ""
echo "==> 验证"
HEALTH=$(ssh "$SSH_HOST" "curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3001/health 2>/dev/null || echo 'down'")
echo "    backend /health: HTTP $HEALTH"
echo ""
echo "✅ 部署完成 — https://attaseek.com"
