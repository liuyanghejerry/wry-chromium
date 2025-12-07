# Quick Start: Using CEF Backend in Wry

This guide helps you get started with the CEF backend in wry.

## Prerequisites

1. **Rust** (latest stable)
2. **CEF binaries** installed on your system

## Step 1: Install CEF Binaries

### Option A: Using export-cef-dir (Recommended)

```bash
# Install the tool
cargo install export-cef-dir

# Export CEF binaries to your system
# Linux/macOS:
export-cef-dir --force $HOME/.local/share/cef

# Windows (PowerShell):
export-cef-dir --force $env:USERPROFILE\.local\share\cef
```

### Option B: Manual Installation

Download CEF binaries from the [official Chromium Embedded Framework downloads page](https://cef-builds.spotifycdn.com/index.html) and extract them to a location of your choice.

## Step 2: Set Environment Variables

### Linux
```bash
export CEF_PATH="$HOME/.local/share/cef"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"
```

### macOS
```bash
export CEF_PATH="$HOME/.local/share/cef"
export DYLD_FALLBACK_LIBRARY_PATH="$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH:$CEF_PATH/Chromium Embedded Framework.framework/Libraries"
```

### Windows (PowerShell)
```powershell
$env:CEF_PATH="$env:USERPROFILE\.local\share\cef"
$env:PATH="$env:PATH;$env:CEF_PATH"
```

## Step 3: Add wry to Your Project

In your `Cargo.toml`:

```toml
[dependencies]
wry = { version = "0.53", features = ["cef"], default-features = false }
```

**Important:** Use `default-features = false` when using CEF, as it's an alternative to the OS-native webviews.

## Step 4: Initialize CEF

In your `main.rs`:

```rust
fn main() -> wry::Result<()> {
    // Initialize CEF first (handles multi-process architecture)
    // Returns false if this is a renderer process
    if !wry::cef_initialize()? {
        // This is a renderer process, exit
        return Ok(());
    }

    // Your application code here...

    // Cleanup before exit
    wry::cef_shutdown();
    Ok(())
}
```

## Step 5: Run Your Application

```bash
cargo run
```

## Current Limitations

The CEF backend is currently **experimental**. The following features are **not yet implemented**:

- ❌ Custom protocol handlers
- ❌ IPC (inter-process communication)
- ❌ Navigation handlers
- ❌ Download handlers
- ❌ Print functionality
- ❌ Bounds management
- ❌ Visibility control
- ❌ Focus management
- ❌ WebContext integration

These features will fail with clear error messages if you try to use them.

## What Works

The following features are implemented:

- ✅ CEF initialization and shutdown
- ✅ Basic webview creation
- ✅ JavaScript evaluation
- ✅ URL loading
- ✅ HTML string loading
- ✅ Message loop management

## Example

See `examples/cef_simple.rs` for a basic example:

```bash
cargo run --example cef_simple --features cef --no-default-features
```

## Troubleshooting

### "CEF must be initialized before creating WebView"
Make sure you call `wry::cef_initialize()` before creating any webviews.

### "Failed to initialize CEF"
- Verify CEF binaries are installed
- Check CEF_PATH environment variable is set correctly
- Ensure CEF libraries are in your LD_LIBRARY_PATH/DYLD_FALLBACK_LIBRARY_PATH/PATH

### Network errors during build
If building fails with network errors, it's trying to download CEF binaries. Install them manually first (see Step 1).

### Feature X is not supported
If you see errors like "Custom protocols are not yet supported with CEF backend", that feature hasn't been implemented yet. See the limitations section above.

## Getting Help

- **Documentation**: See `CEF_INTEGRATION.md` for detailed architecture information
- **Examples**: Check `examples/cef_simple.rs`
- **Issues**: Report issues on the GitHub repository

## Next Steps

Once you have CEF working:

1. Explore the example code to understand the initialization flow
2. Read `CEF_INTEGRATION.md` for architecture details
3. Consider contributing to implement missing features!

## Contributing

The CEF backend needs help! Key areas for contribution:

1. **Window Integration**: Integrate CEF's rendering with tao/winit windows
2. **Feature Implementation**: Add support for custom protocols, IPC, etc.
3. **Examples**: Create more comprehensive examples
4. **Testing**: Add tests for CEF functionality
5. **Documentation**: Improve usage documentation

See `CEF_INTEGRATION.md` for the full list of TODOs.
