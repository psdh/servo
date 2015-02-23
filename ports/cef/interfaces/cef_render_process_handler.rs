/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Structure used to implement render process callbacks. The functions of this
// structure will be called on the render process main thread (TID_RENDERER)
// unless otherwise indicated.
//
#[repr(C)]
pub struct _cef_render_process_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called after the render process main thread has been created. |extra_info|
  // is a read-only value originating from
  // cef_browser_process_handler_t::on_render_process_thread_created(). Do not
  // keep a reference to |extra_info| outside of this function.
  //
  pub on_render_thread_created: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      extra_info: *mut interfaces::cef_list_value_t) -> ()>,

  //
  // Called after WebKit has been initialized.
  //
  pub on_web_kit_initialized: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t) -> ()>,

  //
  // Called after a browser has been created. When browsing cross-origin a new
  // browser will be created before the old browser with the same identifier is
  // destroyed.
  //
  pub on_browser_created: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // Called before a browser is destroyed.
  //
  pub on_browser_destroyed: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // Return the handler for browser load status events.
  //
  pub get_load_handler: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t) -> *mut interfaces::cef_load_handler_t>,

  //
  // Called before browser navigation. Return true (1) to cancel the navigation
  // or false (0) to allow the navigation to proceed. The |request| object
  // cannot be modified in this callback.
  //
  pub on_before_navigation: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      request: *mut interfaces::cef_request_t,
      navigation_type: types::cef_navigation_type_t,
      is_redirect: libc::c_int) -> libc::c_int>,

  //
  // Called immediately after the V8 context for a frame has been created. To
  // retrieve the JavaScript 'window' object use the
  // cef_v8context_t::get_global() function. V8 handles can only be accessed
  // from the thread on which they are created. A task runner for posting tasks
  // on the associated thread can be retrieved via the
  // cef_v8context_t::get_task_runner() function.
  //
  pub on_context_created: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      context: *mut interfaces::cef_v8context_t) -> ()>,

  //
  // Called immediately before the V8 context for a frame is released. No
  // references to the context should be kept after this function is called.
  //
  pub on_context_released: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      context: *mut interfaces::cef_v8context_t) -> ()>,

  //
  // Called for global uncaught exceptions in a frame. Execution of this
  // callback is disabled by default. To enable set
  // CefSettings.uncaught_exception_stack_size > 0.
  //
  pub on_uncaught_exception: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      context: *mut interfaces::cef_v8context_t,
      exception: *mut interfaces::cef_v8exception_t,
      stackTrace: *mut interfaces::cef_v8stack_trace_t) -> ()>,

  //
  // Called when a new node in the the browser gets focus. The |node| value may
  // be NULL if no specific node has gained focus. The node object passed to
  // this function represents a snapshot of the DOM at the time this function is
  // executed. DOM objects are only valid for the scope of this function. Do not
  // keep references to or attempt to access any DOM objects outside the scope
  // of this function.
  //
  pub on_focused_node_changed: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      node: *mut interfaces::cef_domnode_t) -> ()>,

  //
  // Called when a new message is received from a different process. Return true
  // (1) if the message was handled or false (0) otherwise. Do not keep a
  // reference to or attempt to access the message outside of this callback.
  //
  pub on_process_message_received: Option<extern "C" fn(
      this: *mut cef_render_process_handler_t,
      browser: *mut interfaces::cef_browser_t,
      source_process: interfaces::cef_process_id_t,
      message: *mut interfaces::cef_process_message_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_render_process_handler_t = _cef_render_process_handler_t;


//
// Structure used to implement render process callbacks. The functions of this
// structure will be called on the render process main thread (TID_RENDERER)
// unless otherwise indicated.
//
pub struct CefRenderProcessHandler {
  c_object: *mut cef_render_process_handler_t,
}

impl Clone for CefRenderProcessHandler {
  fn clone(&self) -> CefRenderProcessHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefRenderProcessHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefRenderProcessHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefRenderProcessHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_render_process_handler_t) -> CefRenderProcessHandler {
    CefRenderProcessHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_render_process_handler_t) -> CefRenderProcessHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefRenderProcessHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_render_process_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_render_process_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Called after the render process main thread has been created. |extra_info|
  // is a read-only value originating from
  // cef_browser_process_handler_t::on_render_process_thread_created(). Do not
  // keep a reference to |extra_info| outside of this function.
  //
  pub fn on_render_thread_created(&self,
      extra_info: interfaces::CefListValue) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_render_thread_created.unwrap())(
          self.c_object,
          CefWrap::to_c(extra_info)))
    }
  }

  //
  // Called after WebKit has been initialized.
  //
  pub fn on_web_kit_initialized(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_web_kit_initialized.unwrap())(
          self.c_object))
    }
  }

  //
  // Called after a browser has been created. When browsing cross-origin a new
  // browser will be created before the old browser with the same identifier is
  // destroyed.
  //
  pub fn on_browser_created(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_browser_created.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called before a browser is destroyed.
  //
  pub fn on_browser_destroyed(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_browser_destroyed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Return the handler for browser load status events.
  //
  pub fn get_load_handler(&self) -> interfaces::CefLoadHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_load_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Called before browser navigation. Return true (1) to cancel the navigation
  // or false (0) to allow the navigation to proceed. The |request| object
  // cannot be modified in this callback.
  //
  pub fn on_before_navigation(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, request: interfaces::CefRequest,
      navigation_type: types::cef_navigation_type_t,
      is_redirect: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_navigation.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(request),
          CefWrap::to_c(navigation_type),
          CefWrap::to_c(is_redirect)))
    }
  }

  //
  // Called immediately after the V8 context for a frame has been created. To
  // retrieve the JavaScript 'window' object use the
  // cef_v8context_t::get_global() function. V8 handles can only be accessed
  // from the thread on which they are created. A task runner for posting tasks
  // on the associated thread can be retrieved via the
  // cef_v8context_t::get_task_runner() function.
  //
  pub fn on_context_created(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, context: interfaces::CefV8Context) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_context_created.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(context)))
    }
  }

  //
  // Called immediately before the V8 context for a frame is released. No
  // references to the context should be kept after this function is called.
  //
  pub fn on_context_released(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, context: interfaces::CefV8Context) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_context_released.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(context)))
    }
  }

  //
  // Called for global uncaught exceptions in a frame. Execution of this
  // callback is disabled by default. To enable set
  // CefSettings.uncaught_exception_stack_size > 0.
  //
  pub fn on_uncaught_exception(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, context: interfaces::CefV8Context,
      exception: interfaces::CefV8Exception,
      stackTrace: interfaces::CefV8StackTrace) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_uncaught_exception.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(context),
          CefWrap::to_c(exception),
          CefWrap::to_c(stackTrace)))
    }
  }

  //
  // Called when a new node in the the browser gets focus. The |node| value may
  // be NULL if no specific node has gained focus. The node object passed to
  // this function represents a snapshot of the DOM at the time this function is
  // executed. DOM objects are only valid for the scope of this function. Do not
  // keep references to or attempt to access any DOM objects outside the scope
  // of this function.
  //
  pub fn on_focused_node_changed(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, node: interfaces::CefDOMNode) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_focused_node_changed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(node)))
    }
  }

  //
  // Called when a new message is received from a different process. Return true
  // (1) if the message was handled or false (0) otherwise. Do not keep a
  // reference to or attempt to access the message outside of this callback.
  //
  pub fn on_process_message_received(&self, browser: interfaces::CefBrowser,
      source_process: interfaces::CefProcessId,
      message: interfaces::CefProcessMessage) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_process_message_received.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(source_process),
          CefWrap::to_c(message)))
    }
  }
}

impl CefWrap<*mut cef_render_process_handler_t> for CefRenderProcessHandler {
  fn to_c(rust_object: CefRenderProcessHandler) -> *mut cef_render_process_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_render_process_handler_t) -> CefRenderProcessHandler {
    CefRenderProcessHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_render_process_handler_t> for Option<CefRenderProcessHandler> {
  fn to_c(rust_object: Option<CefRenderProcessHandler>) -> *mut cef_render_process_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_render_process_handler_t) -> Option<CefRenderProcessHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefRenderProcessHandler::from_c_object_addref(c_object))
    }
  }
}

