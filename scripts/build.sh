#!/bin/bash
# attaseek.com — 构建（zig 交叉编译，与 asfly.io 同方案）
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Homebrew Rust 跟 rustup 冲突 — 必须优先用 rustup 的 cargo/rustc
RUSTUP_BIN="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin"
export PATH="$RUSTUP_BIN:$PATH"

TARGET="x86_64-unknown-linux-musl"

_preflight() {
    if ! command -v zig &>/dev/null; then
        echo "ERROR: zig not found. Install: brew install zig"
        exit 1
    fi
    if ! cargo zigbuild --help &>/dev/null 2>&1; then
        echo "ERROR: cargo-zigbuild not found. Install: cargo install cargo-zigbuild"
        exit 1
    fi
    # 不需要 rustup target add — cargo-zigbuild 用 zig 自带 sysroot
}

_preflight

echo "==> 1/2 Building Rust backend (cargo zigbuild, $TARGET)..."
cd "$ROOT"
cargo zigbuild -p attaseek-web --release --target "$TARGET"
echo "    → target/$TARGET/release/attaseek-web ($(ls -lh "target/$TARGET/release/attaseek-web" | awk '{print $5}'))"

echo ""
echo "==> 2/2 Building Vue 3 frontend..."
cd "$ROOT/frontend"
npm run build
echo "    → frontend/dist/"

echo ""
echo "✅ Build complete"
