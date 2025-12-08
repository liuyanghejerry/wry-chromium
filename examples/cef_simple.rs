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
//! # Install export-cef-dir tool from cef-rs repository
//! cargo install export-cef-dir
//! export-cef-dir --force $HOME/.local/share/cef
//!
//! # Set environment variables
//! export CEF_PATH="$HOME/.local/share/cef"
//! export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"  # Linux
//! export DYLD_FALLBACK_LIBRARY_PATH="$DYLD_FALLBACK_LIBRARY_PATH:$CEF_PATH"  # macOS
//! ```
//!
//! ### Windows (PowerShell):
//! ```pwsh
//! cargo install export-cef-dir
//! export-cef-dir --force $env:USERPROFILE/.local/share/cef
//!
//! $env:CEF_PATH="$env:USERPROFILE/.local/share/cef"
//! $env:PATH="$env:PATH;$env:CEF_PATH"
//! ```
//!
//! # Important Note
//!
//! This example demonstrates the CEF integration structure in wry.
//! CEF backend is currently experimental due to architectural differences
//! between CEF's window management and wry's design with external windowing libraries.
//!
//! A complete integration would require either:
//! 1. Using CEF's windowed mode (less integrated with external window libraries)
//! 2. Using CEF's off-screen rendering mode (complex rendering pipeline)
//!
//! This example shows the initialization flow and basic structure.

#[cfg(feature = "cef")]
fn main() -> wry::Result<()> {
  println!("=== Wry CEF Backend Example ===");
  println!();

  // Step 1: Initialize CEF
  // This must be called before creating any windows/webviews
  // For multi-process architecture, renderer processes will exit here
  println!("Initializing CEF...");
  if !wry::cef_initialize()? {
    println!("This is a renderer process - exiting.");
    return Ok(()); // Renderer process, exit normally
  }

  println!("✓ CEF initialized successfully");
  println!();

  // Step 2: Normal CEF-based webview usage would happen here
  // This requires:
  // - Creating a window (using tao, winit, or CEF's own window system)
  // - Using WebViewBuilderExtCef::build_cef() to create the webview
  // - Running the CEF message loop

  println!("CEF Integration Structure:");
  println!("  1. Call wry::cef_initialize() - ✓ Done");
  println!("  2. Create window with your windowing library");
  println!("  3. Use WebViewBuilderExtCef::build_cef() to create webview");
  println!("  4. Run wry::cef_run_message_loop()");
  println!("  5. Cleanup with wry::cef_shutdown()");
  println!();

  println!("Note: Full webview creation requires additional integration");
  println!("      between CEF's window system and external windowing libraries.");
  println!();

  // Step 3: Shutdown CEF
  println!("Shutting down CEF...");
  wry::cef_shutdown();
  println!("✓ CEF shutdown complete");

  println!();
  println!("=== Example Complete ===");
  println!();
  println!("This example demonstrates the basic CEF initialization flow.");
  println!("For production use, additional work is needed to integrate");
  println!("CEF's windowing with external libraries like tao or winit.");

  Ok(())
}

#[cfg(not(feature = "cef"))]
fn main() {
  println!("This example requires the 'cef' feature to be enabled.");
  println!("Run with: cargo run --example cef_simple --features cef --no-default-features");
}
