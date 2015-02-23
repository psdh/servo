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
// Implement this structure to handle events related to browser load status. The
// functions of this structure will be called on the browser process UI thread
// or render process main thread (TID_RENDERER).
//
#[repr(C)]
pub struct _cef_load_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called when the loading state has changed. This callback will be executed
  // twice -- once when loading is initiated either programmatically or by user
  // action, and once when loading is terminated due to completion, cancellation
  // of failure.
  //
  pub on_loading_state_change: Option<extern "C" fn(
      this: *mut cef_load_handler_t, browser: *mut interfaces::cef_browser_t,
      isLoading: libc::c_int, canGoBack: libc::c_int,
      canGoForward: libc::c_int) -> ()>,

  //
  // Called when the browser begins loading a frame. The |frame| value will
  // never be NULL -- call the is_main() function to check if this frame is the
  // main frame. Multiple frames may be loading at the same time. Sub-frames may
  // start or continue loading after the main frame load has ended. This
  // function may not be called for a particular frame if the load request for
  // that frame fails. For notification of overall browser load status use
  // OnLoadingStateChange instead.
  //
  pub on_load_start: Option<extern "C" fn(this: *mut cef_load_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t) -> ()>,

  //
  // Called when the browser is done loading a frame. The |frame| value will
  // never be NULL -- call the is_main() function to check if this frame is the
  // main frame. Multiple frames may be loading at the same time. Sub-frames may
  // start or continue loading after the main frame load has ended. This
  // function will always be called for all frames irrespective of whether the
  // request completes successfully.
  //
  pub on_load_end: Option<extern "C" fn(this: *mut cef_load_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t, httpStatusCode: libc::c_int) -> ()>,

  //
  // Called when the resource load for a navigation fails or is canceled.
  // |errorCode| is the error code number, |errorText| is the error text and
  // |failedUrl| is the URL that failed to load. See net\base\net_error_list.h
  // for complete descriptions of the error codes.
  //
  pub on_load_error: Option<extern "C" fn(this: *mut cef_load_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t, errorCode: types::cef_errorcode_t,
      errorText: *const types::cef_string_t,
      failedUrl: *const types::cef_string_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_load_handler_t = _cef_load_handler_t;


//
// Implement this structure to handle events related to browser load status. The
// functions of this structure will be called on the browser process UI thread
// or render process main thread (TID_RENDERER).
//
pub struct CefLoadHandler {
  c_object: *mut cef_load_handler_t,
}

impl Clone for CefLoadHandler {
  fn clone(&self) -> CefLoadHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefLoadHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefLoadHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefLoadHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_load_handler_t) -> CefLoadHandler {
    CefLoadHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_load_handler_t) -> CefLoadHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefLoadHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_load_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_load_handler_t {
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
  // Called when the loading state has changed. This callback will be executed
  // twice -- once when loading is initiated either programmatically or by user
  // action, and once when loading is terminated due to completion, cancellation
  // of failure.
  //
  pub fn on_loading_state_change(&self, browser: interfaces::CefBrowser,
      isLoading: libc::c_int, canGoBack: libc::c_int,
      canGoForward: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_loading_state_change.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(isLoading),
          CefWrap::to_c(canGoBack),
          CefWrap::to_c(canGoForward)))
    }
  }

  //
  // Called when the browser begins loading a frame. The |frame| value will
  // never be NULL -- call the is_main() function to check if this frame is the
  // main frame. Multiple frames may be loading at the same time. Sub-frames may
  // start or continue loading after the main frame load has ended. This
  // function may not be called for a particular frame if the load request for
  // that frame fails. For notification of overall browser load status use
  // OnLoadingStateChange instead.
  //
  pub fn on_load_start(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_load_start.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame)))
    }
  }

  //
  // Called when the browser is done loading a frame. The |frame| value will
  // never be NULL -- call the is_main() function to check if this frame is the
  // main frame. Multiple frames may be loading at the same time. Sub-frames may
  // start or continue loading after the main frame load has ended. This
  // function will always be called for all frames irrespective of whether the
  // request completes successfully.
  //
  pub fn on_load_end(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, httpStatusCode: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_load_end.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(httpStatusCode)))
    }
  }

  //
  // Called when the resource load for a navigation fails or is canceled.
  // |errorCode| is the error code number, |errorText| is the error text and
  // |failedUrl| is the URL that failed to load. See net\base\net_error_list.h
  // for complete descriptions of the error codes.
  //
  pub fn on_load_error(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, errorCode: types::cef_errorcode_t,
      errorText: &[u16], failedUrl: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_load_error.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(errorCode),
          CefWrap::to_c(errorText),
          CefWrap::to_c(failedUrl)))
    }
  }
}

impl CefWrap<*mut cef_load_handler_t> for CefLoadHandler {
  fn to_c(rust_object: CefLoadHandler) -> *mut cef_load_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_load_handler_t) -> CefLoadHandler {
    CefLoadHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_load_handler_t> for Option<CefLoadHandler> {
  fn to_c(rust_object: Option<CefLoadHandler>) -> *mut cef_load_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_load_handler_t) -> Option<CefLoadHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefLoadHandler::from_c_object_addref(c_object))
    }
  }
}

