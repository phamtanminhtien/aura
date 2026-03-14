#!/bin/bash

REPO_ROOT=$(git rev-parse --show-toplevel)
HOOKS_DIR="$REPO_ROOT/.git/hooks"
PRE_COMMIT_SCRIPT="$REPO_ROOT/scripts/pre-commit.sh"

echo "Setting up git hooks..."

# Ensure the scripts are executable
chmod +x "$PRE_COMMIT_SCRIPT"

# Copy/Link the pre-commit script to the git hooks directory
cp "$PRE_COMMIT_SCRIPT" "$HOOKS_DIR/pre-commit"
chmod +x "$HOOKS_DIR/pre-commit"

echo "Done! Git pre-commit hook installed."
