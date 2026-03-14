#!/bin/bash

# Get staged files
STAGED_RS_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

if [ -z "$STAGED_RS_FILES" ]; then
    exit 0
fi

echo "--- Running pre-commit checks ---"

# 1. Check formatting
echo "Checking formatting: cargo fmt -- --check"
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "ERROR: Code is not formatted correctly. Run 'cargo fmt' to fix it."
    exit 1
fi

# 2. Run clippy
echo "Running clippy: cargo clippy -- -A warnings"
cargo clippy -- -A warnings
if [ $? -ne 0 ]; then
    echo "ERROR: Clippy checks failed. Fix the warnings before committing."
    exit 1
fi

echo "--- Pre-commit checks passed! ---"
exit 0
