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
// Callback structure for asynchronous continuation of file dialog requests.
//
#[repr(C)]
pub struct _cef_file_dialog_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Continue the file selection with the specified |file_paths|. This may be a
  // single value or a list of values depending on the dialog mode. An NULL
  // value is treated the same as calling cancel().
  //
  pub cont: Option<extern "C" fn(this: *mut cef_file_dialog_callback_t,
      file_paths: types::cef_string_list_t) -> ()>,

  //
  // Cancel the file selection.
  //
  pub cancel: Option<extern "C" fn(this: *mut cef_file_dialog_callback_t) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_file_dialog_callback_t = _cef_file_dialog_callback_t;


//
// Callback structure for asynchronous continuation of file dialog requests.
//
pub struct CefFileDialogCallback {
  c_object: *mut cef_file_dialog_callback_t,
}

impl Clone for CefFileDialogCallback {
  fn clone(&self) -> CefFileDialogCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefFileDialogCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefFileDialogCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefFileDialogCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_file_dialog_callback_t) -> CefFileDialogCallback {
    CefFileDialogCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_file_dialog_callback_t) -> CefFileDialogCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefFileDialogCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_file_dialog_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_file_dialog_callback_t {
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
  // Continue the file selection with the specified |file_paths|. This may be a
  // single value or a list of values depending on the dialog mode. An NULL
  // value is treated the same as calling cancel().
  //
  pub fn cont(&self, file_paths: Vec<String>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(file_paths)))
    }
  }

  //
  // Cancel the file selection.
  //
  pub fn cancel(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cancel.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_file_dialog_callback_t> for CefFileDialogCallback {
  fn to_c(rust_object: CefFileDialogCallback) -> *mut cef_file_dialog_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_file_dialog_callback_t) -> CefFileDialogCallback {
    CefFileDialogCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_file_dialog_callback_t> for Option<CefFileDialogCallback> {
  fn to_c(rust_object: Option<CefFileDialogCallback>) -> *mut cef_file_dialog_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_file_dialog_callback_t) -> Option<CefFileDialogCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefFileDialogCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Implement this structure to handle dialog events. The functions of this
// structure will be called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_dialog_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called to run a file chooser dialog. |mode| represents the type of dialog
  // to display. |title| to the title to be used for the dialog and may be NULL
  // to show the default title ("Open" or "Save" depending on the mode).
  // |default_file_name| is the default file name to select in the dialog.
  // |accept_types| is a list of valid lower-cased MIME types or file extensions
  // specified in an input element and is used to restrict selectable files to
  // such types. To display a custom dialog return true (1) and execute
  // |callback| either inline or at a later time. To display the default dialog
  // return false (0).
  //
  pub on_file_dialog: Option<extern "C" fn(this: *mut cef_dialog_handler_t,
      browser: *mut interfaces::cef_browser_t,
      mode: types::cef_file_dialog_mode_t, title: *const types::cef_string_t,
      default_file_name: *const types::cef_string_t,
      accept_types: types::cef_string_list_t,
      callback: *mut interfaces::cef_file_dialog_callback_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_dialog_handler_t = _cef_dialog_handler_t;


//
// Implement this structure to handle dialog events. The functions of this
// structure will be called on the browser process UI thread.
//
pub struct CefDialogHandler {
  c_object: *mut cef_dialog_handler_t,
}

impl Clone for CefDialogHandler {
  fn clone(&self) -> CefDialogHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDialogHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDialogHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDialogHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_dialog_handler_t) -> CefDialogHandler {
    CefDialogHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_dialog_handler_t) -> CefDialogHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDialogHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_dialog_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_dialog_handler_t {
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
  // Called to run a file chooser dialog. |mode| represents the type of dialog
  // to display. |title| to the title to be used for the dialog and may be NULL
  // to show the default title ("Open" or "Save" depending on the mode).
  // |default_file_name| is the default file name to select in the dialog.
  // |accept_types| is a list of valid lower-cased MIME types or file extensions
  // specified in an input element and is used to restrict selectable files to
  // such types. To display a custom dialog return true (1) and execute
  // |callback| either inline or at a later time. To display the default dialog
  // return false (0).
  //
  pub fn on_file_dialog(&self, browser: interfaces::CefBrowser,
      mode: types::cef_file_dialog_mode_t, title: &[u16],
      default_file_name: &[u16], accept_types: Vec<String>,
      callback: interfaces::CefFileDialogCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_file_dialog.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(mode),
          CefWrap::to_c(title),
          CefWrap::to_c(default_file_name),
          CefWrap::to_c(accept_types),
          CefWrap::to_c(callback)))
    }
  }
}

impl CefWrap<*mut cef_dialog_handler_t> for CefDialogHandler {
  fn to_c(rust_object: CefDialogHandler) -> *mut cef_dialog_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_dialog_handler_t) -> CefDialogHandler {
    CefDialogHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_dialog_handler_t> for Option<CefDialogHandler> {
  fn to_c(rust_object: Option<CefDialogHandler>) -> *mut cef_dialog_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_dialog_handler_t) -> Option<CefDialogHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDialogHandler::from_c_object_addref(c_object))
    }
  }
}

