#!/usr/bin/env bash
set -e

BINARY_NAME="ohscrcpy"
GITHUB_REPO="robin8yeung/ohscrcpy-cli"
USER_INSTALL_DIR="$HOME/.ohscrcpy/bin"
SYSTEM_INSTALL_DIR="/usr/local/bin"

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

add_to_path() {
  local dir="$1"
  local shell_config=""
  
  if [[ -f "$HOME/.zshrc" ]]; then
    shell_config="$HOME/.zshrc"
  elif [[ -f "$HOME/.bashrc" ]]; then
    shell_config="$HOME/.bashrc"
  elif [[ -f "$HOME/.bash_profile" ]]; then
    shell_config="$HOME/.bash_profile"
  fi
  
  if [[ -n "$shell_config" ]]; then
    if ! grep -q "$dir" "$shell_config"; then
      echo "" >> "$shell_config"
      echo "# Add ohscrcpy to PATH" >> "$shell_config"
      echo "export PATH=\"\$PATH:$dir\"" >> "$shell_config"
      echo ""
      echo "已添加到 PATH，请重新加载配置："
      echo "  source $shell_config"
    fi
  else
    echo ""
    echo "请手动添加到 PATH："
    echo "  export PATH=\"\$PATH:$dir\""
  fi
}

# 解析参数
INSTALL_TYPE=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --user)
      INSTALL_TYPE="user"
      shift
      ;;
    --system)
      INSTALL_TYPE="system"
      shift
      ;;
    *)
      echo "未知参数: $1"
      echo "用法: $0 [--user|--system]"
      exit 1
      ;;
  esac
done

check_command curl

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

# 选择安装方式
if [[ "$INSTALL_TYPE" == "user" ]]; then
  # 用户目录安装
  INSTALL_DIR="$USER_INSTALL_DIR"
  mkdir -p "$INSTALL_DIR"
  mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
  echo ""
  echo "✅ $BINARY_NAME v$LATEST_TAG 安装到用户目录！"
  add_to_path "$INSTALL_DIR"
elif [[ "$INSTALL_TYPE" == "system" ]]; then
  # 系统目录安装
  INSTALL_DIR="$SYSTEM_INSTALL_DIR"
  echo "需要管理员权限，请输入密码："
  sudo mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
  echo ""
  echo "✅ $BINARY_NAME v$LATEST_TAG 安装到系统目录！"
else
  # 交互式选择
  echo ""
  echo "选择安装方式："
  echo "  1) 用户目录（$USER_INSTALL_DIR，无需 sudo）- 推荐"
  echo "  2) 系统目录（$SYSTEM_INSTALL_DIR，需要 sudo）"
  read -p "请选择 [1-2]: " choice

  case $choice in
    1)
      INSTALL_DIR="$USER_INSTALL_DIR"
      mkdir -p "$INSTALL_DIR"
      mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
      echo ""
      echo "✅ $BINARY_NAME v$LATEST_TAG 安装到用户目录！"
      add_to_path "$INSTALL_DIR"
      ;;
    2)
      INSTALL_DIR="$SYSTEM_INSTALL_DIR"
      echo "需要管理员权限，请输入密码："
      sudo mv "$TMP_FILE" "$INSTALL_DIR/$BINARY_NAME"
      echo ""
      echo "✅ $BINARY_NAME v$LATEST_TAG 安装到系统目录！"
      ;;
    *)
      echo "无效的选择，已取消安装"
      rm -f "$TMP_FILE"
      exit 1
      ;;
  esac
fi

if [[ ! -f "$INSTALL_DIR/$BINARY_NAME" ]]; then
  echo "错误：安装失败"
  exit 1
fi

echo ""
echo "使用方法:"
echo "  ohscrcpy --help          # 查看帮助"
echo "  ohscrcpy                 # 连接唯一设备"
echo "  ohscrcpy -s <serial>     # 连接指定设备"
echo ""
echo "需要先安装 hdc 工具（DevEco Studio 附带）"
echo "  export PATH=\"\$PATH:/Applications/DevEco-Studio.app/Contents/tools/hdc/bin\""
