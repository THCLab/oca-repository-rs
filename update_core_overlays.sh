#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$SCRIPT_DIR"

SOURCE_FILE="$REPO_ROOT/../oca-rs/overlay-file/core_overlays/semantic.overlayfile"
TARGET_FILE="$REPO_ROOT/core_overlays/semantic.overlayfile"

if [ ! -f "$SOURCE_FILE" ]; then
    echo "ERROR: Source file not found: $SOURCE_FILE"
    echo "       Ensure oca-rs repository is cloned alongside oca-repository-rs."
    exit 1
fi

if ! diff -q "$SOURCE_FILE" "$TARGET_FILE" > /dev/null 2>&1; then
    echo "core_overlays/semantic.overlayfile is out of date."
    echo ""
    diff "$SOURCE_FILE" "$TARGET_FILE" || true
    echo ""
    echo "Updating..."
    cp "$SOURCE_FILE" "$TARGET_FILE"
    echo "Done. core_overlays/semantic.overlayfile updated from oca-rs."
else
    echo "core_overlays/semantic.overlayfile is up to date."
fi
