// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Utility functions for CEF backend

/// Helper function to convert web context to CEF request context
pub fn web_context_to_cef_context(
  _web_context: Option<&mut crate::WebContext>,
) -> Option<cef::RequestContext> {
  // TODO: Implement conversion from WebContext to CEF RequestContext
  None
}
