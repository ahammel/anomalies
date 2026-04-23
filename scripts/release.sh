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

# Bump version in both crate manifests
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" anomalies/Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" anomalies-derive/Cargo.toml

# Pin the intra-workspace dep so crates.io can resolve it after publish
sed -i '' "s|anomalies-derive = { path = \"../anomalies-derive\" }|anomalies-derive = { path = \"../anomalies-derive\", version = \"$VERSION\" }|" anomalies/Cargo.toml

# Settle Cargo.lock before staging so the commit is self-contained
cargo check --all

confirm "git add anomalies/Cargo.toml anomalies-derive/Cargo.toml Cargo.lock" &&
    git add anomalies/Cargo.toml anomalies-derive/Cargo.toml Cargo.lock

confirm "git commit -m '🔖 v$VERSION'" &&
    git commit -m "🔖 v$VERSION"

confirm "git tag v$VERSION" &&
    git tag "v$VERSION"

confirm "git push origin main v$VERSION" &&
    git push origin main "v$VERSION"

# anomalies-derive must land on the registry before anomalies can reference it
confirm "cargo publish -p anomalies-derive" &&
    cargo publish -p anomalies-derive

echo "Waiting for anomalies-derive v$VERSION to become available on crates.io..."
sleep 10

confirm "cargo publish -p anomalies" &&
    cargo publish -p anomalies
