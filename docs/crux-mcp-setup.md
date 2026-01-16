# Crux MCP Server Setup

This guide explains how to set up the [crux-mcp](https://github.com/redbadger/crux-mcp) server for your Crux projects.

## Quick Start

Run the automated setup script:

```bash
./scripts/setup-crux-mcp.sh
```

This will:
- Clone and build the crux-mcp server
- Install the wasm32-wasip2 target
- Build your WASM component
- Configure MCP for Kiro CLI

Then start Kiro CLI and the MCP server will auto-load!

## Manual Setup

### Prerequisites

1. **Rust with wasm32-wasip2 target**:
   ```bash
   rustup target add wasm32-wasip2
   ```

2. **Clone crux-mcp server**:
   ```bash
   cd ~/IT/Rust/projects  # or your preferred location
   git clone https://github.com/redbadger/crux-mcp.git
   cd crux-mcp
   cargo build --release
   ```

## Building Your Crux Component

Build your shared library as a WASM component:

```bash
# From your project root (e.g., crab-news)
cd shared
cargo build --target wasm32-wasip2 --release
```

The WASM component will be at:
```
target/wasm32-wasip2/release/shared.wasm
```

## Kiro CLI MCP Configuration

### Option 1: Global Configuration (Reusable)

Add to `~/.kiro/mcp.json`:

```json
{
  "mcpServers": {
    "crux-crab-news": {
      "command": "/path/to/crux-mcp/target/release/crux-mcp",
      "args": [],
      "env": {
        "CRUX_COMPONENT": "/Users/andreacfromtheapp/IT/Rust/projects/crab-news/target/wasm32-wasip2/release/shared.wasm"
      }
    }
  }
}
```

### Option 2: Project-Specific Configuration

Add to `.kiro/mcp.json` in your project root:

```json
{
  "mcpServers": {
    "crux-app": {
      "command": "/path/to/crux-mcp/target/release/crux-mcp",
      "args": [],
      "env": {
        "CRUX_COMPONENT": "./target/wasm32-wasip2/release/shared.wasm"
      }
    }
  }
}
```

## Usage with Kiro CLI

1. **Start Kiro CLI** in your project directory
2. **The MCP server will auto-load** if configured
3. **Use crux-mcp tools** in your conversation:
   ```
   > Use the crux-app MCP server to inspect the application model
   ```

## Reusing Across Projects

### Template Configuration

Create `~/crux-mcp-template.json`:

```json
{
  "mcpServers": {
    "crux-app": {
      "command": "/path/to/crux-mcp/target/release/crux-mcp",
      "args": [],
      "env": {
        "CRUX_COMPONENT": "./target/wasm32-wasip2/release/shared.wasm"
      }
    }
  }
}
```

### For Each New Project

```bash
# In your new Crux project
mkdir -p .kiro
cp ~/crux-mcp-template.json .kiro/mcp.json

# Build the WASM component
cd shared
cargo build --target wasm32-wasip2 --release
```

## Build Script (Optional)

Create `scripts/build-wasm.sh`:

```bash
#!/bin/bash
set -e

echo "Building Crux WASM component..."
cd shared
cargo build --target wasm32-wasip2 --release
echo "✓ WASM component built at: target/wasm32-wasip2/release/shared.wasm"
```

Make it executable:
```bash
chmod +x scripts/build-wasm.sh
```

## Troubleshooting

### "CRUX_COMPONENT not found"
- Ensure the path in `env.CRUX_COMPONENT` is correct
- Use absolute paths for global config
- Use relative paths (starting with `./`) for project-specific config

### "wasm32-wasip2 target not found"
```bash
rustup target add wasm32-wasip2
```

### MCP server not loading
- Check Kiro CLI logs: `/code logs`
- Verify crux-mcp binary exists and is executable
- Ensure WASM component is built

## Notes

- The crux-mcp server is still in development (not fully implemented)
- Requires Crux PR #401 features
- WASM component must be rebuilt after code changes
