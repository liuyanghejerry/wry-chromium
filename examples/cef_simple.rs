// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! CEF Simple Example
//!
//! This example demonstrates how to use wry with the CEF backend.
//!
//! # Prerequisites
//!
//! 1. Install CEF binaries (see below)
//! 2. Build with the `cef` feature: `cargo run --example cef_simple --features cef --no-default-features`
//!
//! ## Installing CEF Binaries
//!
//! ### Linux or macOS:
//! ```sh
//! cargo run -p export-cef-dir -- --force $HOME/.local/share/cef
//! export CEF_PATH="$HOME/.local/share/cef"
//! export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"  # Linux
//! export DYLD_FALLBACK_LIBRARY_PATH="$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH"  # macOS
//! ```
//!
//! ### Windows (PowerShell):
//! ```pwsh
//! cargo run -p export-cef-dir -- --force $env:USERPROFILE/.local/share/cef
//! $env:CEF_PATH="$env:USERPROFILE/.local/share/cef"
//! $env:PATH="$env:PATH;$env:CEF_PATH"
//! ```

#[cfg(feature = "cef")]
fn main() -> wry::Result<()> {
  // Initialize CEF
  // This must be called before creating any windows/webviews
  // For multi-process architecture, renderer processes will exit here
  if !wry::cef_initialize()? {
    return Ok(()); // Renderer process, exit normally
  }

  println!("CEF initialized successfully");

  // Note: For a complete CEF integration, you would typically:
  // 1. Create a window using your preferred windowing library (tao, winit, etc.)
  // 2. Use WebViewBuilderExtCef::build_cef() to create the webview
  // 3. Run the CEF message loop using wry::cef_run_message_loop()
  // 4. Clean up with wry::cef_shutdown()

  println!("CEF example placeholder - full integration requires window creation");
  println!("See the wry documentation for complete CEF integration examples");

  // Shutdown CEF
  wry::cef_shutdown();

  Ok(())
}

#[cfg(not(feature = "cef"))]
fn main() {
  println!("This example requires the 'cef' feature to be enabled.");
  println!("Run with: cargo run --example cef_simple --features cef --no-default-features");
}
