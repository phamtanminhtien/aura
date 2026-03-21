#!/bin/bash

# Aura Installation Script
# https://github.com/auraspace/aura

set -e

REPO_URL="https://github.com/auraspace/aura"
VERSION="${1:-latest}"
INSTALL_DIR="$HOME/.aura"
BIN_DIR="$INSTALL_DIR/bin"
AURA_EXE="$BIN_DIR/aura"

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
RESET='\033[0m'

# --- Logging Functions ---
info() {
    echo -e "${BLUE}info${RESET} $1"
}

success() {
    echo -e "${GREEN}success${RESET} $1"
}

warn() {
    echo -e "${YELLOW}warning${RESET} $1"
}

error() {
    echo -e "${RED}error${RESET} $1"
}

# 1. Detect platform
OS_NAME=$(uname -s)
ARCH_NAME=$(uname -m)

case "$OS_NAME" in
    Darwin)
        OS="macos"
        ;;
    *)
        error "Unsupported OS: $OS_NAME"
        exit 1
        ;;
esac

case "$ARCH_NAME" in
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        error "Unsupported architecture: $ARCH_NAME"
        exit 1
        ;;
esac

TARGET="aura-${OS}-${ARCH}"

# 2. Check for existing installation
IS_UPGRADE=false
if [ -f "$AURA_EXE" ]; then
    IS_UPGRADE=true
    CURRENT_VERSION=$("$AURA_EXE" --version 2>/dev/null | awk '{print $NF}')
    info "Existing Aura installation found: ${BOLD}${CURRENT_VERSION:-unknown}${RESET}"
fi

# 3. Get the actual version if "latest"
if [ "$VERSION" = "latest" ]; then
    info "Checking for latest version..."
    VERSION=$(curl -s https://api.github.com/repos/auraspace/aura/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$VERSION" ]; then
        error "Could not determine latest version. Please specify a version tag (e.g., v0.1.0)."
        exit 1
    fi
fi

if [ "$IS_UPGRADE" = true ]; then
    info "Updating Aura to ${BOLD}${VERSION}${RESET}..."
else
    info "Installing Aura ${BOLD}${VERSION}${RESET} for ${BOLD}${TARGET}${RESET}..."
fi

# 4. Construct download URL
DOWNLOAD_URL="${REPO_URL}/releases/download/${VERSION}/${TARGET}.tar.gz"

info "Downloading from: ${BLUE}${DOWNLOAD_URL}${RESET}"

# 5. DOWNLOAD and EXTRACT
mkdir -p "$INSTALL_DIR"
TEMP_FILE=$(mktemp).tar.gz

if ! curl -L "$DOWNLOAD_URL" -o "$TEMP_FILE"; then
    error "Failed to download Aura from ${DOWNLOAD_URL}"
    exit 1
fi

info "Extracting to ${BOLD}${INSTALL_DIR}${RESET}..."
tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR"
rm "$TEMP_FILE"

# 6. Setup PATH
mkdir -p "$BIN_DIR"
# The archive contains 'aura' binary at the root
if [ -f "$INSTALL_DIR/aura" ]; then
    mv "$INSTALL_DIR/aura" "$AURA_EXE"
fi

echo ""
success "Aura has been $( [ "$IS_UPGRADE" = true ] && echo "updated" || echo "installed" ) to ${BOLD}${INSTALL_DIR}${RESET}"
echo ""

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    warn "Aura bin directory is not in your PATH."
    echo ""
    info "To finish installation, add Aura to your PATH:"
    echo -e "  ${BOLD}export PATH=\"$BIN_DIR:\$PATH\"${RESET}"
    echo ""
    info "You can add this to your .zshrc or .bashrc:"
    echo -e "  ${BOLD}echo 'export PATH=\"$BIN_DIR:\$PATH\"' >> ~/.zshrc${RESET}"
    echo ""
    info "Then restart your terminal or run 'source ~/.zshrc'"
else
    success "Aura is already in your PATH."
fi
