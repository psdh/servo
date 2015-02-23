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
// Structure used to implement browser process callbacks. The functions of this
// structure will be called on the browser process main thread unless otherwise
// indicated.
//
#[repr(C)]
pub struct _cef_browser_process_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called on the browser process UI thread immediately after the CEF context
  // has been initialized.
  //
  pub on_context_initialized: Option<extern "C" fn(
      this: *mut cef_browser_process_handler_t) -> ()>,

  //
  // Called before a child process is launched. Will be called on the browser
  // process UI thread when launching a render process and on the browser
  // process IO thread when launching a GPU or plugin process. Provides an
  // opportunity to modify the child process command line. Do not keep a
  // reference to |command_line| outside of this function.
  //
  pub on_before_child_process_launch: Option<extern "C" fn(
      this: *mut cef_browser_process_handler_t,
      command_line: *mut interfaces::cef_command_line_t) -> ()>,

  //
  // Called on the browser process IO thread after the main thread has been
  // created for a new render process. Provides an opportunity to specify extra
  // information that will be passed to
  // cef_render_process_handler_t::on_render_thread_created() in the render
  // process. Do not keep a reference to |extra_info| outside of this function.
  //
  pub on_render_process_thread_created: Option<extern "C" fn(
      this: *mut cef_browser_process_handler_t,
      extra_info: *mut interfaces::cef_list_value_t) -> ()>,

  //
  // Return the handler for printing on Linux. If a print handler is not
  // provided then printing will not be supported on the Linux platform.
  //
  pub get_print_handler: Option<extern "C" fn(
      this: *mut cef_browser_process_handler_t) -> *mut interfaces::cef_print_handler_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_browser_process_handler_t = _cef_browser_process_handler_t;


//
// Structure used to implement browser process callbacks. The functions of this
// structure will be called on the browser process main thread unless otherwise
// indicated.
//
pub struct CefBrowserProcessHandler {
  c_object: *mut cef_browser_process_handler_t,
}

impl Clone for CefBrowserProcessHandler {
  fn clone(&self) -> CefBrowserProcessHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefBrowserProcessHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefBrowserProcessHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefBrowserProcessHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_browser_process_handler_t) -> CefBrowserProcessHandler {
    CefBrowserProcessHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_browser_process_handler_t) -> CefBrowserProcessHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefBrowserProcessHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_browser_process_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_browser_process_handler_t {
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
  // Called on the browser process UI thread immediately after the CEF context
  // has been initialized.
  //
  pub fn on_context_initialized(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_context_initialized.unwrap())(
          self.c_object))
    }
  }

  //
  // Called before a child process is launched. Will be called on the browser
  // process UI thread when launching a render process and on the browser
  // process IO thread when launching a GPU or plugin process. Provides an
  // opportunity to modify the child process command line. Do not keep a
  // reference to |command_line| outside of this function.
  //
  pub fn on_before_child_process_launch(&self,
      command_line: interfaces::CefCommandLine) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_child_process_launch.unwrap())(
          self.c_object,
          CefWrap::to_c(command_line)))
    }
  }

  //
  // Called on the browser process IO thread after the main thread has been
  // created for a new render process. Provides an opportunity to specify extra
  // information that will be passed to
  // cef_render_process_handler_t::on_render_thread_created() in the render
  // process. Do not keep a reference to |extra_info| outside of this function.
  //
  pub fn on_render_process_thread_created(&self,
      extra_info: interfaces::CefListValue) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_render_process_thread_created.unwrap())(
          self.c_object,
          CefWrap::to_c(extra_info)))
    }
  }

  //
  // Return the handler for printing on Linux. If a print handler is not
  // provided then printing will not be supported on the Linux platform.
  //
  pub fn get_print_handler(&self) -> interfaces::CefPrintHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_print_handler.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_browser_process_handler_t> for CefBrowserProcessHandler {
  fn to_c(rust_object: CefBrowserProcessHandler) -> *mut cef_browser_process_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_browser_process_handler_t) -> CefBrowserProcessHandler {
    CefBrowserProcessHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_browser_process_handler_t> for Option<CefBrowserProcessHandler> {
  fn to_c(rust_object: Option<CefBrowserProcessHandler>) -> *mut cef_browser_process_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_browser_process_handler_t) -> Option<CefBrowserProcessHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefBrowserProcessHandler::from_c_object_addref(c_object))
    }
  }
}

