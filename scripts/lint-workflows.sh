#!/bin/bash

# Lint all Dify workflow YAML files
# Usage: ./scripts/lint-workflows.sh [path]
#   path: Optional path to specific workflow file or directory

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
LINTER_PATH="$ROOT_DIR/dify-linter/target/release/dify-linter"

# Build linter if needed
if [ ! -f "$LINTER_PATH" ]; then
    echo "Building dify-linter..."
    cd "$ROOT_DIR/dify-linter" && cargo build --release
fi

TARGET="${1:-$ROOT_DIR/workflows}"

if [ -f "$TARGET" ]; then
    # Single file
    echo "Linting: $TARGET"
    "$LINTER_PATH" "$TARGET"
elif [ -d "$TARGET" ]; then
    # Directory - find all workflow.yml files
    ERRORS=0
    for FILE in $(find "$TARGET" -name "*.yml" -type f); do
        echo "Linting: $FILE"
        if ! "$LINTER_PATH" "$FILE"; then
            ERRORS=$((ERRORS + 1))
        fi
        echo ""
    done

    if [ $ERRORS -gt 0 ]; then
        echo "Found errors in $ERRORS file(s)"
        exit 1
    fi
    echo "All files passed!"
else
    echo "Error: $TARGET not found"
    exit 1
fi
