// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Utility functions for CEF backend

use cef::{rc::*, *};

/// Helper function to convert web context to CEF request context
pub fn web_context_to_cef_context(
  web_context: Option<&mut crate::WebContext>,
) -> Option<RequestContext> {
  if let Some(ctx) = web_context {
    // Get the data directory from WebContext
    if let Some(data_dir) = ctx.data_directory() {
      // Create a CEF request context with the data directory
      let cache_path = CefString::from(data_dir.to_string_lossy().as_ref());
      
      // Create request context settings
      let settings = RequestContextSettings {
        cache_path: Some(&cache_path),
        persist_session_cookies: 1,
        accept_language_list: None,
        cookieable_schemes_list: None,
        cookieable_schemes_exclude_defaults: 0,
      };
      
      // Create and return the request context
      if let Ok(req_ctx) = request_context_create_context(&settings, None) {
        return Some(req_ctx);
      }
    }
  }
  
  // Return None if no context or failed to create, CEF will use default
  None
}
