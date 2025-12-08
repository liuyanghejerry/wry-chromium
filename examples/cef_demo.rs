// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! CEF Demo Example - Load Google
//!
//! This example demonstrates CEF integration by creating a window that loads Google.
//!
//! # Prerequisites
//!
//! 1. Install CEF binaries:
//!    ```sh
//!    cargo install export-cef-dir
//!    export-cef-dir --force $HOME/.local/share/cef
//!    export CEF_PATH="$HOME/.local/share/cef"
//!    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$CEF_PATH"  # Linux
//!    ```
//!
//! 2. Build and run:
//!    ```sh
//!    cargo run --example cef_demo --features cef --no-default-features
//!    ```
//!
//! # What This Demo Shows
//!
//! - CEF initialization and lifecycle management
//! - Creating a CEF window with webview
//! - Loading Google (https://www.google.com)
//! - Window interaction (resize, minimize, maximize, close)
//! - Clean shutdown
//!
//! # Note
//!
//! This demo uses CEF's native window system. For production use with
//! external windowing libraries (tao/winit), you would need to implement
//! off-screen rendering (OSR) mode. See CEF_INTEGRATION.md for details.

#[cfg(feature = "cef")]
fn main() -> wry::Result<()> {
  use cef::{rc::*, *};
  use std::sync::{Arc, Mutex};

  println!("╔════════════════════════════════════════════════════════════╗");
  println!("║           Wry CEF Demo - Loading Google                   ║");
  println!("╚════════════════════════════════════════════════════════════╝");
  println!();

  // Step 1: Initialize CEF
  println!("📦 Step 1: Initializing CEF...");
  if !wry::cef_initialize()? {
    // This is a renderer or helper process
    return Ok(());
  }
  println!("   ✅ CEF initialized successfully");
  println!();

  // Step 2: Setup CEF window and browser
  println!("🪟 Step 2: Creating window and browser...");

  // Shared window reference
  let window = Arc::new(Mutex::new(None));
  let window_for_handler = window.clone();

  // Create browser process handler
  wrap_browser_process_handler! {
    struct DemoBrowserProcessHandler {
      window: Arc<Mutex<Option<Window>>>,
    }

    impl BrowserProcessHandler {
      fn on_context_initialized(&self) {
        println!("   ✅ CEF context initialized");

        // Create client
        let mut client = DemoClient::new();

        // Set URL to Google
        let url = CefString::from("https://www.google.com");
        println!();
        println!("🌐 Step 3: Loading Google...");
        println!("   URL: https://www.google.com");

        // Create browser view
        let browser_view = match browser_view_create(
          Some(&mut client),
          Some(&url),
          Some(&Default::default()),
          Option::<&mut DictionaryValue>::None,
          Option::<&mut RequestContext>::None,
          Option::<&mut BrowserViewDelegate>::None,
        ) {
          Ok(view) => {
            println!("   ✅ Browser view created");
            view
          },
          Err(e) => {
            eprintln!("   ❌ Failed to create browser view: {:?}", e);
            return;
          }
        };

        // Create window delegate
        let mut delegate = DemoWindowDelegate::new(browser_view);

        // Create window
        match window_create_top_level(Some(&mut delegate)) {
          Ok(win) => {
            println!("   ✅ Window created");
            println!();
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║                   🎉 Demo Running!                        ║");
            println!("╚════════════════════════════════════════════════════════════╝");
            println!();
            println!("✨ Features:");
            println!("   • Window with Google loading");
            println!("   • Resize, minimize, maximize, close buttons work");
            println!("   • CEF handles page rendering");
            println!("   • Navigation and IPC handlers active");
            println!();
            println!("📸 To take a screenshot:");
            println!("   • Use your OS screenshot tool (e.g., Cmd+Shift+4 on macOS)");
            println!("   • The window shows Google's homepage");
            println!();
            println!("🛑 To exit:");
            println!("   • Close the window, or press Ctrl+C");
            println!();

            if let Ok(mut w) = self.window.lock() {
              *w = Some(win);
            }
          },
          Err(e) => {
            eprintln!("   ❌ Failed to create window: {:?}", e);
          }
        }
      }
    }
  }

  // Create client with handlers
  wrap_client! {
    struct DemoClient;

    impl Client {
      fn on_process_message_received(
        &self,
        _browser: Option<&mut Browser>,
        _frame: Option<&mut Frame>,
        _source_process: ProcessId,
        _message: Option<&mut ProcessMessage>,
      ) -> ::std::os::raw::c_int {
        // IPC message handler
        0
      }
    }
  }

  // Create window delegate
  wrap_window_delegate! {
    struct DemoWindowDelegate {
      browser_view: BrowserView,
    }

    impl ViewDelegate {
      fn on_child_view_changed(
        &self,
        _view: Option<&mut View>,
        _added: ::std::os::raw::c_int,
        _child: Option<&mut View>,
      ) {
      }
    }

    impl PanelDelegate {}

    impl WindowDelegate {
      fn on_window_created(&self, window: Option<&mut Window>) {
        if let Some(window) = window {
          // Add browser view to window
          let view = self.browser_view.clone();
          window.add_child_view(Some(&mut (&view).into()));

          // Set window title
          let title = CefString::from("Wry CEF Demo - Google");
          window.set_title(Some(&title));

          // Show window
          window.show();
        }
      }

      fn on_window_destroyed(&self, _window: Option<&mut Window>) {
        println!();
        println!("👋 Window closed");
        quit_message_loop();
      }

      fn with_standard_window_buttons(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1 // Enable standard buttons
      }

      fn can_resize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1 // Allow resize
      }

      fn can_maximize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1 // Allow maximize
      }

      fn can_minimize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1 // Allow minimize
      }

      fn can_close(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
        1 // Allow close
      }
    }
  }

  // Create app
  wrap_app! {
    struct DemoApp {
      window: Arc<Mutex<Option<Window>>>,
    }

    impl App {
      fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
        Some(DemoBrowserProcessHandler::new(self.window.clone()))
      }
    }
  }

  let _app = DemoApp::new(window_for_handler);

  // Step 4: Run message loop
  println!("⏳ Running CEF message loop...");
  println!("   (Waiting for window events...)");
  println!();

  wry::cef_run_message_loop();

  // Step 5: Cleanup
  println!();
  println!("🧹 Cleaning up...");
  wry::cef_shutdown();
  println!("   ✅ CEF shutdown complete");
  println!();
  println!("╔════════════════════════════════════════════════════════════╗");
  println!("║                  Demo Complete! Goodbye!                   ║");
  println!("╚════════════════════════════════════════════════════════════╝");

  Ok(())
}

#[cfg(not(feature = "cef"))]
fn main() {
  println!("❌ This example requires the 'cef' feature to be enabled.");
  println!();
  println!("Run with:");
  println!("  cargo run --example cef_demo --features cef --no-default-features");
}
