#!/bin/bash
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WASM_TARGET="wasm32-wasip2"
BUILD_TYPE="${1:-release}"

echo "🦀 Building Crux WASM component for crab-news..."
echo "   Target: $WASM_TARGET"
echo "   Build type: $BUILD_TYPE"
echo ""

cd "$PROJECT_ROOT/shared"

if [ "$BUILD_TYPE" = "debug" ]; then
    cargo build --target "$WASM_TARGET"
    WASM_PATH="$PROJECT_ROOT/target/$WASM_TARGET/debug/shared.wasm"
else
    cargo build --target "$WASM_TARGET" --release
    WASM_PATH="$PROJECT_ROOT/target/$WASM_TARGET/release/shared.wasm"
fi

if [ -f "$WASM_PATH" ]; then
    echo "✓ WASM component built successfully!"
    echo "  Location: $WASM_PATH"
    echo ""
    echo "To use with crux-mcp, set:"
    echo "  export CRUX_COMPONENT=$WASM_PATH"
else
    echo "✗ Build failed - WASM component not found"
    exit 1
fi
