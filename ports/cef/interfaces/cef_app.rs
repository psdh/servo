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
// Implement this structure to provide handler implementations. Methods will be
// called by the process and/or thread indicated.
//
#[repr(C)]
pub struct _cef_app_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Provides an opportunity to view and/or modify command-line arguments before
  // processing by CEF and Chromium. The |process_type| value will be NULL for
  // the browser process. Do not keep a reference to the cef_command_line_t
  // object passed to this function. The CefSettings.command_line_args_disabled
  // value can be used to start with an NULL command-line object. Any values
  // specified in CefSettings that equate to command-line arguments will be set
  // before this function is called. Be cautious when using this function to
  // modify command-line arguments for non-browser processes as this may result
  // in undefined behavior including crashes.
  //
  pub on_before_command_line_processing: Option<extern "C" fn(
      this: *mut cef_app_t, process_type: *const types::cef_string_t,
      command_line: *mut interfaces::cef_command_line_t) -> ()>,

  //
  // Provides an opportunity to register custom schemes. Do not keep a reference
  // to the |registrar| object. This function is called on the main thread for
  // each process and the registered schemes should be the same across all
  // processes.
  //
  pub on_register_custom_schemes: Option<extern "C" fn(this: *mut cef_app_t,
      registrar: *mut interfaces::cef_scheme_registrar_t) -> ()>,

  //
  // Return the handler for resource bundle events. If
  // CefSettings.pack_loading_disabled is true (1) a handler must be returned.
  // If no handler is returned resources will be loaded from pack files. This
  // function is called by the browser and render processes on multiple threads.
  //
  pub get_resource_bundle_handler: Option<extern "C" fn(
      this: *mut cef_app_t) -> *mut interfaces::cef_resource_bundle_handler_t>,

  //
  // Return the handler for functionality specific to the browser process. This
  // function is called on multiple threads in the browser process.
  //
  pub get_browser_process_handler: Option<extern "C" fn(
      this: *mut cef_app_t) -> *mut interfaces::cef_browser_process_handler_t>,

  //
  // Return the handler for functionality specific to the render process. This
  // function is called on the render process main thread.
  //
  pub get_render_process_handler: Option<extern "C" fn(
      this: *mut cef_app_t) -> *mut interfaces::cef_render_process_handler_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_app_t = _cef_app_t;


//
// Implement this structure to provide handler implementations. Methods will be
// called by the process and/or thread indicated.
//
pub struct CefApp {
  c_object: *mut cef_app_t,
}

impl Clone for CefApp {
  fn clone(&self) -> CefApp{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefApp {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefApp {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefApp {
  pub unsafe fn from_c_object(c_object: *mut cef_app_t) -> CefApp {
    CefApp {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_app_t) -> CefApp {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefApp {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_app_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_app_t {
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
  // Provides an opportunity to view and/or modify command-line arguments before
  // processing by CEF and Chromium. The |process_type| value will be NULL for
  // the browser process. Do not keep a reference to the cef_command_line_t
  // object passed to this function. The CefSettings.command_line_args_disabled
  // value can be used to start with an NULL command-line object. Any values
  // specified in CefSettings that equate to command-line arguments will be set
  // before this function is called. Be cautious when using this function to
  // modify command-line arguments for non-browser processes as this may result
  // in undefined behavior including crashes.
  //
  pub fn on_before_command_line_processing(&self, process_type: &[u16],
      command_line: interfaces::CefCommandLine) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_command_line_processing.unwrap())(
          self.c_object,
          CefWrap::to_c(process_type),
          CefWrap::to_c(command_line)))
    }
  }

  //
  // Provides an opportunity to register custom schemes. Do not keep a reference
  // to the |registrar| object. This function is called on the main thread for
  // each process and the registered schemes should be the same across all
  // processes.
  //
  pub fn on_register_custom_schemes(&self,
      registrar: interfaces::CefSchemeRegistrar) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_register_custom_schemes.unwrap())(
          self.c_object,
          CefWrap::to_c(registrar)))
    }
  }

  //
  // Return the handler for resource bundle events. If
  // CefSettings.pack_loading_disabled is true (1) a handler must be returned.
  // If no handler is returned resources will be loaded from pack files. This
  // function is called by the browser and render processes on multiple threads.
  //
  pub fn get_resource_bundle_handler(
      &self) -> interfaces::CefResourceBundleHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_resource_bundle_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for functionality specific to the browser process. This
  // function is called on multiple threads in the browser process.
  //
  pub fn get_browser_process_handler(
      &self) -> interfaces::CefBrowserProcessHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_browser_process_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the handler for functionality specific to the render process. This
  // function is called on the render process main thread.
  //
  pub fn get_render_process_handler(
      &self) -> interfaces::CefRenderProcessHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_render_process_handler.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_app_t> for CefApp {
  fn to_c(rust_object: CefApp) -> *mut cef_app_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_app_t) -> CefApp {
    CefApp::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_app_t> for Option<CefApp> {
  fn to_c(rust_object: Option<CefApp>) -> *mut cef_app_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_app_t) -> Option<CefApp> {
    if c_object.is_null() {
      None
    } else {
      Some(CefApp::from_c_object_addref(c_object))
    }
  }
}

