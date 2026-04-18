#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:?usage: release.sh <version>}"

confirm() {
    local prompt="$1"
    read -r -p "$prompt [Y/n] " reply
    case "${reply:-Y}" in
        [Yy]*) return 0 ;;
        *)     echo "Aborted."; exit 1 ;;
    esac
}

# Bump version in Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

confirm "git add Cargo.toml Cargo.lock" &&
    git add Cargo.toml Cargo.lock

confirm "git commit -m '🔖 v$VERSION'" &&
    git commit -m "🔖 v$VERSION"

confirm "git tag v$VERSION" &&
    git tag "v$VERSION"

confirm "git push origin main v$VERSION" &&
    git push origin main "v$VERSION"

confirm "cargo publish" &&
    cargo publish
