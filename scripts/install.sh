#!/bin/bash

# Aura Installation Script
# https://github.com/phamtanminhtien/aura

set -e

REPO_URL="https://github.com/phamtanminhtien/aura"
VERSION="${1:-latest}"
INSTALL_DIR="$HOME/.aura"

# 1. Detect platform
OS_NAME=$(uname -s)
ARCH_NAME=$(uname -m)

case "$OS_NAME" in
    Darwin)
        OS="macos"
        ;;
    *)
        echo "Error: Unsupported OS $OS_NAME"
        exit 1
        ;;
esac

case "$ARCH_NAME" in
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Error: Unsupported architecture $ARCH_NAME"
        exit 1
        ;;
esac

TARGET="aura-${OS}-${ARCH}"

# 2. Get the actual version if "latest"
if [ "$VERSION" = "latest" ]; then
    echo "Checking for latest version..."
    VERSION=$(curl -s https://api.github.com/repos/phamtanminhtien/aura/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        echo "Error: Could not determine latest version. Please specify a version tag (e.g., v0.1.0)."
        exit 1
    fi
fi

# 3. Construct download URL
DOWNLOAD_URL="${REPO_URL}/releases/download/${VERSION}/${TARGET}.tar.gz"

echo "Downloading Aura ${VERSION} for ${TARGET}..."
echo "URL: ${DOWNLOAD_URL}"

# 4. DOWNLOAD and EXTRACT
mkdir -p "$INSTALL_DIR"
TEMP_FILE=$(mktemp).tar.gz

curl -L "$DOWNLOAD_URL" -o "$TEMP_FILE"

echo "Extracting to ${INSTALL_DIR}..."
tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR"
rm "$TEMP_FILE"

# 5. Setup PATH
# We'll put the binary in a 'bin' subdirectory for cleanliness
mkdir -p "$INSTALL_DIR/bin"
mv "$INSTALL_DIR/aura" "$INSTALL_DIR/bin/aura" 2>/dev/null || true

EXE_PATH="$INSTALL_DIR/bin"

echo ""
echo "Aura has been installed to ${INSTALL_DIR}"
echo ""
echo "To finish installation, add Aura to your PATH:"
echo "  export PATH=\"$EXE_PATH:\$PATH\""
echo ""
echo "You can add this to your .zshrc or .bashrc:"
echo "  echo 'export PATH=\"$EXE_PATH:\$PATH\"' >> ~/.zshrc"
echo ""
echo "Then restart your terminal or run 'source ~/.zshrc'"
