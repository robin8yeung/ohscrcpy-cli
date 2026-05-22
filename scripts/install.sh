#!/usr/bin/env bash
set -e

BINARY_NAME="ohscrcpy"
INSTALL_DIR="/usr/local/bin"
GITHUB_REPO="robin8yeung/ohos-scrcpy-cli"

detect_arch() {
  if [[ "$(uname -m)" == "arm64" ]]; then
    echo "aarch64-apple-darwin"
  else
    echo "x86_64-apple-darwin"
  fi
}

check_command() {
  if ! command -v "$1" &>/dev/null; then
    echo "错误：未找到 $1，请先安装"
    exit 1
  fi
}

check_command curl
check_command sudo

echo "检测系统架构..."
ARCH=$(detect_arch)
echo "架构: $ARCH"

echo "获取最新版本..."

# 尝试从 API 获取版本（可能受速率限制）
LATEST_TAG=""
RETRY_COUNT=3
for i in $(seq 1 $RETRY_COUNT); do
  LATEST_TAG=$(curl -s -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/$GITHUB_REPO/releases/latest" 2>/dev/null \
    | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
  if [[ -n "$LATEST_TAG" ]]; then
    break
  fi
  sleep 1
done

# 如果 API 获取失败，尝试解析 HTML 页面
if [[ -z "$LATEST_TAG" ]]; then
  echo "API 请求受限，尝试备用方式..."
  LATEST_TAG=$(curl -s "https://github.com/$GITHUB_REPO/releases" 2>/dev/null \
    | grep -o "releases/tag/[^\"']*" | head -1 | sed 's|releases/tag/||')
fi

if [[ -z "$LATEST_TAG" ]]; then
  echo "错误：无法获取最新版本"
  echo ""
  echo "可能的原因："
  echo "  1. GitHub API 速率限制（请稍后再试）"
  echo "  2. 仓库尚未创建 Release（请联系管理员）"
  echo ""
  echo "手动安装方法："
  echo "  git clone https://github.com/$GITHUB_REPO.git"
  echo "  cd ohscrcpy-cli"
  echo "  bash scripts/build_cli.sh"
  echo "  sudo cp release/ohscrcpy-$ARCH /usr/local/bin/ohscrcpy"
  exit 1
fi

echo "最新版本: $LATEST_TAG"
echo "正在下载 $BINARY_NAME..."

DOWNLOAD_URL="https://github.com/$GITHUB_REPO/releases/download/$LATEST_TAG/ohscrcpy-$ARCH"
TMP_FILE=$(mktemp)

curl -sSL "$DOWNLOAD_URL" -o "$TMP_FILE"

if [[ ! -f "$TMP_FILE" || ! -s "$TMP_FILE" ]]; then
  echo "错误：下载失败或文件为空"
  echo "请检查以下 URL 是否可访问："
  echo "  $DOWNLOAD_URL"
  rm -f "$TMP_FILE"
  exit 1
fi

echo "验证二进制文件..."
chmod +x "$TMP_FILE"

echo "安装到 $INSTALL_DIR..."
sudo mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"

if [[ ! -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
  echo "错误：安装失败"
  exit 1
fi

echo ""
echo "✅ $BINARY_NAME v$LATEST_TAG 安装成功！"
echo ""
echo "使用方法:"
echo "  ohscrcpy --help          # 查看帮助"
echo "  ohscrcpy                 # 连接唯一设备"
echo "  ohscrcpy -s <serial>     # 连接指定设备"
echo ""
echo "需要先安装 hdc 工具（DevEco Studio 附带）"
echo "  export PATH=\"\$PATH:/Applications/DevEco-Studio.app/Contents/tools/hdc/bin\""
