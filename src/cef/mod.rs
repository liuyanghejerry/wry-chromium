// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! CEF (Chromium Embedded Framework) backend for wry
//!
//! This module provides a CEF-based webview implementation that offers
//! a consistent Chromium experience across all platforms.
//!
//! # Important Note
//!
//! CEF integration with wry is currently experimental and has limitations:
//!
//! - CEF uses its own window management system, which differs from wry's design
//!   where external windowing libraries (tao/winit) manage windows
//! - Full integration would require either:
//!   1. Using CEF's windowed mode (less integrated with external window libraries)
//!   2. Using CEF's off-screen rendering mode (complex rendering pipeline)
//! - The current implementation provides the foundation for CEF integration
//!   but may require additional work for production use
//!
//! # Usage
//!
//! To use CEF backend:
//! 1. Call `cef_initialize()` before creating any windows/webviews
//! 2. Use `WebViewBuilderExtCef::build_cef()` to create webviews
//! 3. Run the message loop with `cef_run_message_loop()`
//! 4. Call `cef_shutdown()` before application exit

use std::{
  borrow::Cow,
  rc::Rc,
  sync::{Arc, Mutex, OnceLock},
};

use cef::{rc::*, *};
use http::{Request, Response};
use raw_window_handle::HasWindowHandle;

use crate::{
  Error, PageLoadEvent, Rect, RequestAsyncResponder, Result, WebContext, WebViewAttributes, RGBA,
};

mod util;

static CEF_INITIALIZED: OnceLock<bool> = OnceLock::new();
static CEF_APP: OnceLock<Arc<Mutex<Option<WryApp>>>> = OnceLock::new();

/// CEF-based WebView implementation
pub struct InnerWebView {
  browser: Option<Browser>,
  browser_view: Option<BrowserView>,
  window: Option<Window>,
}

impl InnerWebView {
  pub fn new<W: HasWindowHandle>(
    window: &W,
    attributes: WebViewAttributes,
    _platform_attributes: super::PlatformSpecificWebViewAttributes,
    _web_context: Option<&mut WebContext>,
  ) -> Result<Self> {
    // Ensure CEF is initialized
    if CEF_INITIALIZED.get().is_none() {
      return Err(Error::InitializationError(
        "CEF must be initialized before creating WebView. Call cef::initialize() first."
          .to_string(),
      ));
    }

    // Create CEF client
    let mut client = WryClient::new(attributes.clone());

    // Prepare URL
    let url = if let Some(url_str) = &attributes.url {
      CefString::from(url_str.as_str())
    } else if attributes.html.is_some() {
      CefString::from("about:blank")
    } else {
      CefString::from("about:blank")
    };

    // Create browser view
    let browser_view = browser_view_create(
      Some(&mut client),
      Some(&url),
      Some(&Default::default()),
      Option::<&mut DictionaryValue>::None,
      Option::<&mut RequestContext>::None,
      Option::<&mut BrowserViewDelegate>::None,
    )
    .map_err(|e| Error::InitializationError(format!("Failed to create browser view: {:?}", e)))?;

    let browser = browser_view.browser();

    // Load HTML if provided
    if let Some(html) = &attributes.html {
      if let Some(mut browser) = browser.as_ref() {
        let frame = browser.main_frame();
        if let Some(mut frame) = frame {
          let url = CefString::from("about:blank");
          let html_str = CefString::from(html.as_str());
          frame.load_string(Some(&html_str), Some(&url));
        }
      }
    }

    Ok(Self {
      browser,
      browser_view: Some(browser_view),
      window: None,
    })
  }

  pub fn eval(&self, js: &str, _callback: Option<impl FnOnce(String) + Send + 'static>) -> Result<()> {
    if let Some(browser) = &self.browser {
      let frame = browser.main_frame();
      if let Some(mut frame) = frame {
        let code = CefString::from(js);
        let url = CefString::from("");
        frame.execute_java_script(Some(&code), Some(&url), 0);
      }
    }
    Ok(())
  }

  pub fn print(&self) -> Result<()> {
    // CEF print functionality would be implemented here
    Ok(())
  }

  pub fn url(&self) -> Result<String> {
    if let Some(browser) = &self.browser {
      let frame = browser.main_frame();
      if let Some(frame) = frame {
        return Ok(frame.url().to_string());
      }
    }
    Ok(String::new())
  }

  pub fn bounds(&self) -> Result<Rect> {
    // CEF view bounds would be queried here
    Ok(Rect::default())
  }

  pub fn set_bounds(&self, _bounds: Rect) -> Result<()> {
    // CEF view bounds would be set here
    Ok(())
  }

  pub fn set_visible(&self, _visible: bool) -> Result<()> {
    // CEF view visibility would be set here
    Ok(())
  }

  pub fn focus(&self) -> Result<()> {
    // CEF view focus would be set here
    Ok(())
  }
}

impl Drop for InnerWebView {
  fn drop(&mut self) {
    // Clean up CEF resources
    if let Some(mut browser) = self.browser.take() {
      let host = browser.host();
      if let Some(mut host) = host {
        host.close_browser(1);
      }
    }
  }
}

/// Initialize CEF for the process
///
/// This must be called before creating any WebViews with CEF backend.
/// It handles the multi-process architecture of CEF.
///
/// Returns true if this is the browser process and initialization succeeded,
/// false if this is a renderer process (and the process should exit after returning).
pub fn initialize() -> Result<bool> {
  let args = cef::args::Args::new();
  let cmd = args.as_cmd_line().unwrap();

  let switch = CefString::from("type");
  let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

  let app = Arc::new(Mutex::new(Some(WryApp::new())));
  CEF_APP.get_or_init(|| app.clone());

  let mut app_guard = app.lock().unwrap();
  let app_ref = app_guard.as_mut().unwrap();

  // Execute process
  let ret = execute_process(Some(args.as_main_args()), Some(app_ref), std::ptr::null_mut());

  if !is_browser_process {
    // This is a renderer or other helper process
    std::process::exit(ret);
  }

  // Initialize CEF for browser process
  let settings = Settings {
    no_sandbox: !cfg!(feature = "sandbox") as _,
    ..Default::default()
  };

  let init_result = cef::initialize(
    Some(args.as_main_args()),
    Some(&settings),
    Some(app_ref),
    std::ptr::null_mut(),
  );

  if init_result != 1 {
    return Err(Error::InitializationError(
      "Failed to initialize CEF".to_string(),
    ));
  }

  CEF_INITIALIZED.get_or_init(|| true);
  Ok(true)
}

/// Shutdown CEF
///
/// This should be called before the application exits.
pub fn shutdown() {
  cef::shutdown();
}

/// Run the CEF message loop
///
/// This should be called from the main thread.
pub fn run_message_loop() {
  cef::run_message_loop();
}

/// Quit the CEF message loop
///
/// This will cause `run_message_loop()` to return.
pub fn quit_message_loop() {
  cef::quit_message_loop();
}

// CEF App implementation
wrap_app! {
  struct WryApp {
    // Add any app-level state here if needed
  }

  impl App {
    fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
      Some(WryBrowserProcessHandler::new())
    }
  }
}

// Browser Process Handler
wrap_browser_process_handler! {
  struct WryBrowserProcessHandler {
    // Add browser process state here if needed
  }

  impl BrowserProcessHandler {
    fn on_context_initialized(&self) {
      // CEF context is initialized
      println!("CEF context initialized");
    }
  }
}

// CEF Client implementation
wrap_client! {
  struct WryClient {
    attributes: WebViewAttributes<'static>,
  }

  impl Client {
    // Add client callbacks here
  }
}

impl WryClient {
  fn new(attributes: WebViewAttributes) -> Self {
    // Convert to 'static lifetime by cloning necessary data
    let static_attrs = WebViewAttributes {
      id: attributes.id.map(|id| Box::leak(id.to_string().into_boxed_str()) as &str),
      context: None, // Can't be made static easily
      user_agent: attributes.user_agent.clone(),
      visible: attributes.visible,
      transparent: attributes.transparent,
      background_color: attributes.background_color,
      url: attributes.url.clone(),
      headers: attributes.headers.clone(),
      html: attributes.html.clone(),
      initialization_scripts: attributes.initialization_scripts.clone(),
      custom_protocols: vec![], // Would need special handling
      ipc_handler: None,        // Would need special handling
      drag_drop_handler: None,  // Would need special handling
      navigation_handler: None, // Would need special handling
      download_started_handler: None,
      download_completed_handler: None,
      new_window_req_handler: None,
      clipboard: attributes.clipboard,
      devtools: attributes.devtools,
      zoom_hotkeys_enabled: attributes.zoom_hotkeys_enabled,
      accept_first_mouse: attributes.accept_first_mouse,
      back_forward_navigation_gestures: attributes.back_forward_navigation_gestures,
      document_title_changed_handler: None,
      incognito: attributes.incognito,
      autoplay: attributes.autoplay,
      on_page_load_handler: None,
      proxy_config: attributes.proxy_config.clone(),
      focused: attributes.focused,
      bounds: attributes.bounds,
      background_throttling: attributes.background_throttling,
      javascript_disabled: attributes.javascript_disabled,
    };

    Self::allocate(static_attrs)
  }
}
