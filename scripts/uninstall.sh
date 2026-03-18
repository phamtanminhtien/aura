#!/bin/bash

# Aura Uninstallation Script
# https://github.com/auraspace/aura

INSTALL_DIR="$HOME/.aura"

# Confirm uninstallation
read -p "Are you sure you want to uninstall Aura? (y/N): " confirm
if [[ ! $confirm =~ ^[Yy]$ ]]; then
    echo "Uninstallation cancelled."
    exit 0
fi

if [ -d "$INSTALL_DIR" ]; then
    echo "Removing Aura installation directory: $INSTALL_DIR"
    rm -rf "$INSTALL_DIR"
    echo "Aura has been uninstalled."
else
    echo "Aura installation directory not found at $INSTALL_DIR."
fi

echo ""
echo "Please remember to remove the Aura PATH entry from your shell configuration (e.g., .zshrc or .bashrc)."
echo "Look for a line like: export PATH=\"\$HOME/.aura/bin:\$PATH\""
