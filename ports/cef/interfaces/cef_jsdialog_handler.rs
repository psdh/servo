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
// Callback structure used for asynchronous continuation of JavaScript dialog
// requests.
//
#[repr(C)]
pub struct _cef_jsdialog_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Continue the JS dialog request. Set |success| to true (1) if the OK button
  // was pressed. The |user_input| value should be specified for prompt dialogs.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_jsdialog_callback_t,
      success: libc::c_int, user_input: *const types::cef_string_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_jsdialog_callback_t = _cef_jsdialog_callback_t;


//
// Callback structure used for asynchronous continuation of JavaScript dialog
// requests.
//
pub struct CefJSDialogCallback {
  c_object: *mut cef_jsdialog_callback_t,
}

impl Clone for CefJSDialogCallback {
  fn clone(&self) -> CefJSDialogCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefJSDialogCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefJSDialogCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefJSDialogCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_jsdialog_callback_t) -> CefJSDialogCallback {
    CefJSDialogCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_jsdialog_callback_t) -> CefJSDialogCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefJSDialogCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_jsdialog_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_jsdialog_callback_t {
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
  // Continue the JS dialog request. Set |success| to true (1) if the OK button
  // was pressed. The |user_input| value should be specified for prompt dialogs.
  //
  pub fn cont(&self, success: libc::c_int, user_input: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(success),
          CefWrap::to_c(user_input)))
    }
  }
}

impl CefWrap<*mut cef_jsdialog_callback_t> for CefJSDialogCallback {
  fn to_c(rust_object: CefJSDialogCallback) -> *mut cef_jsdialog_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_jsdialog_callback_t) -> CefJSDialogCallback {
    CefJSDialogCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_jsdialog_callback_t> for Option<CefJSDialogCallback> {
  fn to_c(rust_object: Option<CefJSDialogCallback>) -> *mut cef_jsdialog_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_jsdialog_callback_t) -> Option<CefJSDialogCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefJSDialogCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Implement this structure to handle events related to JavaScript dialogs. The
// functions of this structure will be called on the UI thread.
//
#[repr(C)]
pub struct _cef_jsdialog_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called to run a JavaScript dialog. The |default_prompt_text| value will be
  // specified for prompt dialogs only. Set |suppress_message| to true (1) and
  // return false (0) to suppress the message (suppressing messages is
  // preferable to immediately executing the callback as this is used to detect
  // presumably malicious behavior like spamming alert messages in
  // onbeforeunload). Set |suppress_message| to false (0) and return false (0)
  // to use the default implementation (the default implementation will show one
  // modal dialog at a time and suppress any additional dialog requests until
  // the displayed dialog is dismissed). Return true (1) if the application will
  // use a custom dialog or if the callback has been executed immediately.
  // Custom dialogs may be either modal or modeless. If a custom dialog is used
  // the application must execute |callback| once the custom dialog is
  // dismissed.
  //
  pub on_jsdialog: Option<extern "C" fn(this: *mut cef_jsdialog_handler_t,
      browser: *mut interfaces::cef_browser_t,
      origin_url: *const types::cef_string_t,
      accept_lang: *const types::cef_string_t,
      dialog_type: types::cef_jsdialog_type_t,
      message_text: *const types::cef_string_t,
      default_prompt_text: *const types::cef_string_t,
      callback: *mut interfaces::cef_jsdialog_callback_t,
      suppress_message: *mut libc::c_int) -> libc::c_int>,

  //
  // Called to run a dialog asking the user if they want to leave a page. Return
  // false (0) to use the default dialog implementation. Return true (1) if the
  // application will use a custom dialog or if the callback has been executed
  // immediately. Custom dialogs may be either modal or modeless. If a custom
  // dialog is used the application must execute |callback| once the custom
  // dialog is dismissed.
  //
  pub on_before_unload_dialog: Option<extern "C" fn(
      this: *mut cef_jsdialog_handler_t,
      browser: *mut interfaces::cef_browser_t,
      message_text: *const types::cef_string_t, is_reload: libc::c_int,
      callback: *mut interfaces::cef_jsdialog_callback_t) -> libc::c_int>,

  //
  // Called to cancel any pending dialogs and reset any saved dialog state. Will
  // be called due to events like page navigation irregardless of whether any
  // dialogs are currently pending.
  //
  pub on_reset_dialog_state: Option<extern "C" fn(
      this: *mut cef_jsdialog_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // Called when the default implementation dialog is closed.
  //
  pub on_dialog_closed: Option<extern "C" fn(this: *mut cef_jsdialog_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_jsdialog_handler_t = _cef_jsdialog_handler_t;


//
// Implement this structure to handle events related to JavaScript dialogs. The
// functions of this structure will be called on the UI thread.
//
pub struct CefJSDialogHandler {
  c_object: *mut cef_jsdialog_handler_t,
}

impl Clone for CefJSDialogHandler {
  fn clone(&self) -> CefJSDialogHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefJSDialogHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefJSDialogHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefJSDialogHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_jsdialog_handler_t) -> CefJSDialogHandler {
    CefJSDialogHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_jsdialog_handler_t) -> CefJSDialogHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefJSDialogHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_jsdialog_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_jsdialog_handler_t {
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
  // Called to run a JavaScript dialog. The |default_prompt_text| value will be
  // specified for prompt dialogs only. Set |suppress_message| to true (1) and
  // return false (0) to suppress the message (suppressing messages is
  // preferable to immediately executing the callback as this is used to detect
  // presumably malicious behavior like spamming alert messages in
  // onbeforeunload). Set |suppress_message| to false (0) and return false (0)
  // to use the default implementation (the default implementation will show one
  // modal dialog at a time and suppress any additional dialog requests until
  // the displayed dialog is dismissed). Return true (1) if the application will
  // use a custom dialog or if the callback has been executed immediately.
  // Custom dialogs may be either modal or modeless. If a custom dialog is used
  // the application must execute |callback| once the custom dialog is
  // dismissed.
  //
  pub fn on_jsdialog(&self, browser: interfaces::CefBrowser, origin_url: &[u16],
      accept_lang: &[u16], dialog_type: types::cef_jsdialog_type_t,
      message_text: &[u16], default_prompt_text: &[u16],
      callback: interfaces::CefJSDialogCallback,
      suppress_message: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_jsdialog.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(origin_url),
          CefWrap::to_c(accept_lang),
          CefWrap::to_c(dialog_type),
          CefWrap::to_c(message_text),
          CefWrap::to_c(default_prompt_text),
          CefWrap::to_c(callback),
          CefWrap::to_c(suppress_message)))
    }
  }

  //
  // Called to run a dialog asking the user if they want to leave a page. Return
  // false (0) to use the default dialog implementation. Return true (1) if the
  // application will use a custom dialog or if the callback has been executed
  // immediately. Custom dialogs may be either modal or modeless. If a custom
  // dialog is used the application must execute |callback| once the custom
  // dialog is dismissed.
  //
  pub fn on_before_unload_dialog(&self, browser: interfaces::CefBrowser,
      message_text: &[u16], is_reload: libc::c_int,
      callback: interfaces::CefJSDialogCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_unload_dialog.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(message_text),
          CefWrap::to_c(is_reload),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Called to cancel any pending dialogs and reset any saved dialog state. Will
  // be called due to events like page navigation irregardless of whether any
  // dialogs are currently pending.
  //
  pub fn on_reset_dialog_state(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_reset_dialog_state.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called when the default implementation dialog is closed.
  //
  pub fn on_dialog_closed(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_dialog_closed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }
}

impl CefWrap<*mut cef_jsdialog_handler_t> for CefJSDialogHandler {
  fn to_c(rust_object: CefJSDialogHandler) -> *mut cef_jsdialog_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_jsdialog_handler_t) -> CefJSDialogHandler {
    CefJSDialogHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_jsdialog_handler_t> for Option<CefJSDialogHandler> {
  fn to_c(rust_object: Option<CefJSDialogHandler>) -> *mut cef_jsdialog_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_jsdialog_handler_t) -> Option<CefJSDialogHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefJSDialogHandler::from_c_object_addref(c_object))
    }
  }
}

