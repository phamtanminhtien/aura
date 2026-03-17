# Aura Release Documentation

This document describes the release process for the Aura project.

## Overview

The release process involves building the Aura compiler and its standard library, packaging them into a platform-specific tarball, and creating a GitHub Release with these artifacts.

## Release Artifacts

Each release artifact contains:
- `aura`: The compiled Aura binary.
- `stdlib/`: The Aura standard library.

The artifact is packaged as a `.tar.gz` file with the naming convention: `aura-<os>-<arch>.tar.gz` (e.g., `aura-macos-arm64.tar.gz`).

## Automated Release (CI)

To trigger an automated release, simply run:

```bash
cargo release <version> --execute
```

This command (provided by `cargo-release`) handles versioning, tagging, and pushing to GitHub. Once the tag is pushed, the `.github/workflows/release.yml` workflow will automatically:
- Build the binary for supported platforms.
- Package the binary and `stdlib`.
- Create a GitHub Release and upload the artifacts.

## Local Release

You can perform a release locally using the `scripts/release.sh` script.

### Prerequisites
- [Rust and Cargo](https://rustup.rs/) installed.
- [GitHub CLI (gh)](https://cli.github.com/) (optional, for automated release creation).

### Usage
```bash
# Run a dry run to build and package without creating a GitHub Release
./scripts/release.sh --dry-run

# Create a full release (requires 'gh' CLI and permissions)
./scripts/release.sh --tag v0.1.0
```

### Options
- `--tag <version>`: Specify the version tag (e.g., `v0.1.0`). If omitted, it attempts to detect the current tag or uses the short commit hash.
- `--dry-run`: Build and package artifacts into `dist/` without creating a GitHub Release.
- `--help`: Show usage information.

## Versioning and Changelog

- **Versioning**: Aura follows [Semantic Versioning](https://semver.org/).
- **Changelog**: The `CHANGELOG.md` is updated manually or via tools before creating a release tag.

## Directory Structure
- `scripts/release.sh`: The local release script.
- `.github/workflows/release.yml`: The CI release workflow.
- `dist/`: Directory where release artifacts are generated.
- `pkg/`: Temporary staging directory for packaging.
