# CEF Demo Example

This example demonstrates the CEF backend in wry by creating a window that loads Google.

## Screenshots

When you run this demo, you'll see:

1. Console output showing the initialization steps
2. A window opening with Google's homepage
3. Full browser functionality (navigation, forms, etc.)

## Prerequisites

### 1. Install CEF Binaries

#### Linux
```bash
cargo install export-cef-dir
export-cef-dir --force $HOME/.local/share/cef
export CEF_PATH="$HOME/.local/share/cef"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"
```

#### macOS
```bash
cargo install export-cef-dir
export-cef-dir --force $HOME/.local/share/cef
export CEF_PATH="$HOME/.local/share/cef"
export DYLD_FALLBACK_LIBRARY_PATH="$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH"
```

#### Windows (PowerShell)
```powershell
cargo install export-cef-dir
export-cef-dir --force $env:USERPROFILE\.local\share\cef
$env:CEF_PATH="$env:USERPROFILE\.local\share\cef"
$env:PATH="$env:PATH;$env:CEF_PATH"
```

## Running the Demo

```bash
cargo run --example cef_demo --features cef --no-default-features
```

## What You'll See

1. **Console Output**: Step-by-step initialization messages
2. **Window Opens**: A native window with standard controls (minimize, maximize, close)
3. **Google Loads**: The window displays Google's homepage
4. **Interactive**: You can interact with the page normally

## Taking Screenshots

To capture the demo:

- **macOS**: Press `Cmd+Shift+4` and click the window
- **Linux**: Use your screenshot tool (e.g., `gnome-screenshot`, `spectacle`)
- **Windows**: Press `Win+Shift+S` and select the window

## Expected Output

### Console
```
╔════════════════════════════════════════════════════════════╗
║           Wry CEF Demo - Loading Google                   ║
╚════════════════════════════════════════════════════════════╝

📦 Step 1: Initializing CEF...
   ✅ CEF initialized successfully

🪟 Step 2: Creating window and browser...
   ✅ CEF context initialized

🌐 Step 3: Loading Google...
   URL: https://www.google.com
   ✅ Browser view created
   ✅ Window created

╔════════════════════════════════════════════════════════════╗
║                   🎉 Demo Running!                        ║
╚════════════════════════════════════════════════════════════╝

✨ Features:
   • Window with Google loading
   • Resize, minimize, maximize, close buttons work
   • CEF handles page rendering
   • Navigation and IPC handlers active

📸 To take a screenshot:
   • Use your OS screenshot tool
   • The window shows Google's homepage

🛑 To exit:
   • Close the window, or press Ctrl+C

⏳ Running CEF message loop...
   (Waiting for window events...)
```

### Window
The window will display:
- Google's homepage (https://www.google.com)
- Search box and Google logo
- Fully functional web page
- Standard window controls

## Troubleshooting

### "CEF must be initialized" error
Make sure CEF binaries are installed (see Prerequisites above).

### Network timeout during build
CEF binaries are large (~200MB). The build downloads them automatically.
Ensure you have a stable internet connection.

### "No such file or directory" for libcef.so
Set `LD_LIBRARY_PATH` (Linux) or `DYLD_FALLBACK_LIBRARY_PATH` (macOS) correctly.

### Window doesn't appear
Check console output for error messages. Ensure CEF_PATH is set correctly.

## How It Works

1. **CEF Initialization**: Calls `wry::cef_initialize()` which sets up CEF's multi-process architecture
2. **Browser Process Handler**: Creates a handler that runs when CEF context is ready
3. **Window Creation**: Uses CEF's native window system to create a top-level window
4. **Browser View**: Creates a CEF BrowserView and adds it to the window
5. **URL Loading**: Loads Google's URL in the browser view
6. **Message Loop**: Runs CEF's message loop to handle events
7. **Cleanup**: Shuts down CEF when window is closed

## Integration with wry

This demo uses CEF's native windowing for simplicity. For production use:

- **With tao/winit**: Use off-screen rendering (OSR) mode
- **With wry API**: Use `WebViewBuilder::with_url().build_cef()`
- See `CEF_INTEGRATION.md` for details

## Features Demonstrated

- ✅ CEF initialization and lifecycle
- ✅ Window creation
- ✅ Loading external URLs
- ✅ Browser interaction
- ✅ Clean shutdown
- ✅ Multi-process architecture

## Next Steps

- Read `CEF_INTEGRATION.md` for architecture details
- Read `CEF_QUICKSTART.md` for API usage
- Explore the implemented features (IPC, navigation handlers, etc.)
- Try modifying the URL to load different sites
