#!/usr/bin/env bash
set -e

cd "$(dirname "$0")/.."

echo "构建 ohscrcpy CLI..."

if [[ ! -d "cli" ]]; then
  echo "错误：cli 目录不存在"
  exit 1
fi

cd cli

# 检测当前机器架构
CURRENT_ARCH=$(uname -m)
echo "当前架构: $CURRENT_ARCH"

echo "==> 安装 Rust 目标（如未安装）..."
if [[ "$CURRENT_ARCH" == "arm64" ]]; then
  rustup target add aarch64-apple-darwin 2>/dev/null || true
else
  rustup target add x86_64-apple-darwin 2>/dev/null || true
fi

# 只构建当前架构（避免跨架构编译时依赖库不兼容）
if [[ "$CURRENT_ARCH" == "arm64" ]]; then
  echo "==> 构建 aarch64 (Apple Silicon)..."
  cargo build --release --target aarch64-apple-darwin
  
  mkdir -p ../release
  cp target/aarch64-apple-darwin/release/ohscrcpy ../release/ohscrcpy-aarch64-apple-darwin
  
  echo ""
  echo "✅ 构建完成！产物位于 release/ 目录"
  echo "   - ohscrcpy-aarch64-apple-darwin  (Apple Silicon 专用)"
  echo ""
  echo "注意：如需构建 Universal Binary，需要先安装 x86_64 版本的 SDL2"
  echo "  arch -x86_64 brew install sdl2"
  echo ""
else
  echo "==> 构建 x86_64 (Intel)..."
  cargo build --release --target x86_64-apple-darwin
  
  mkdir -p ../release
  cp target/x86_64-apple-darwin/release/ohscrcpy ../release/ohscrcpy-x86_64-apple-darwin
  
  echo ""
  echo "✅ 构建完成！产物位于 release/ 目录"
  echo "   - ohscrcpy-x86_64-apple-darwin   (Intel 专用)"
fi

echo ""
echo "上传到 GitHub Releases 后，用户可通过以下命令安装："
echo "  curl -sSL https://raw.githubusercontent.com/robin8yeung/ohscrcpy-cli/main/scripts/install.sh | sh"
