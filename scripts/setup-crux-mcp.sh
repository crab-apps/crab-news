#!/bin/bash
set -e

echo "🚀 Crux MCP Setup Script"
echo "========================"
echo ""

# Check if crux-mcp exists
CRUX_MCP_PATH="$HOME/IT/Rust/projects/crux-mcp"

if [ ! -d "$CRUX_MCP_PATH" ]; then
    echo "📦 Cloning crux-mcp repository..."
    mkdir -p "$HOME/IT/Rust/projects"
    cd "$HOME/IT/Rust/projects"
    git clone https://github.com/redbadger/crux-mcp.git
    echo "✓ Repository cloned"
    echo ""
fi

# Build crux-mcp
echo "🔨 Building crux-mcp server..."
cd "$CRUX_MCP_PATH"
cargo build --release
echo "✓ crux-mcp built"
echo ""

# Check wasm32-wasip2 target
echo "🎯 Checking Rust targets..."
if ! rustup target list | grep -q "wasm32-wasip2 (installed)"; then
    echo "📥 Installing wasm32-wasip2 target..."
    rustup target add wasm32-wasip2
    echo "✓ Target installed"
else
    echo "✓ wasm32-wasip2 already installed"
fi
echo ""

# Build WASM component for crab-news
echo "🦀 Building crab-news WASM component..."
cd "$HOME/IT/Rust/projects/crab-news"
./scripts/build-wasm.sh release
echo ""

# Update MCP config with correct path
echo "⚙️  Updating MCP configuration..."
MCP_CONFIG="$HOME/IT/Rust/projects/crab-news/.kiro/mcp.json"
sed -i.bak "s|/Users/andreacfromtheapp/IT/Rust/projects/crux-mcp|$CRUX_MCP_PATH|g" "$MCP_CONFIG"
rm "$MCP_CONFIG.bak"
echo "✓ Configuration updated"
echo ""

# Update template
echo "📝 Creating reusable template..."
TEMPLATE="$HOME/crux-mcp-template.json"
sed -i.bak "s|REPLACE_WITH_CRUX_MCP_PATH|$CRUX_MCP_PATH|g" "$TEMPLATE"
rm "$TEMPLATE.bak"
echo "✓ Template created at: $TEMPLATE"
echo ""

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Start Kiro CLI in your project: cd ~/IT/Rust/projects/crab-news && kiro-cli chat"
echo "2. The crux-mcp server will auto-load"
echo "3. For other projects, copy the template: cp ~/crux-mcp-template.json <project>/.kiro/mcp.json"
echo ""
echo "See docs/crux-mcp-setup.md for more details"
