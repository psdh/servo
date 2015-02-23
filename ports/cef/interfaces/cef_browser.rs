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
// Structure used to represent a browser window. When used in the browser
// process the functions of this structure may be called on any thread unless
// otherwise indicated in the comments. When used in the render process the
// functions of this structure may only be called on the main thread.
//
#[repr(C)]
pub struct _cef_browser_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the browser host object. This function can only be called in the
  // browser process.
  //
  pub get_host: Option<extern "C" fn(
      this: *mut cef_browser_t) -> *mut interfaces::cef_browser_host_t>,

  //
  // Returns true (1) if the browser can navigate backwards.
  //
  pub can_go_back: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Navigate backwards.
  //
  pub go_back: Option<extern "C" fn(this: *mut cef_browser_t) -> ()>,

  //
  // Returns true (1) if the browser can navigate forwards.
  //
  pub can_go_forward: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Navigate forwards.
  //
  pub go_forward: Option<extern "C" fn(this: *mut cef_browser_t) -> ()>,

  //
  // Returns true (1) if the browser is currently loading.
  //
  pub is_loading: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Reload the current page.
  //
  pub reload: Option<extern "C" fn(this: *mut cef_browser_t) -> ()>,

  //
  // Reload the current page ignoring any cached data.
  //
  pub reload_ignore_cache: Option<extern "C" fn(this: *mut cef_browser_t) -> (
      )>,

  //
  // Stop loading the page.
  //
  pub stop_load: Option<extern "C" fn(this: *mut cef_browser_t) -> ()>,

  //
  // Returns the globally unique identifier for this browser.
  //
  pub get_identifier: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub is_same: Option<extern "C" fn(this: *mut cef_browser_t,
      that: *mut interfaces::cef_browser_t) -> libc::c_int>,

  //
  // Returns true (1) if the window is a popup window.
  //
  pub is_popup: Option<extern "C" fn(this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Returns true (1) if a document has been loaded in the browser.
  //
  pub has_document: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::c_int>,

  //
  // Returns the main (top-level) frame for the browser window.
  //
  pub get_main_frame: Option<extern "C" fn(
      this: *mut cef_browser_t) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the focused frame for the browser window.
  //
  pub get_focused_frame: Option<extern "C" fn(
      this: *mut cef_browser_t) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the frame with the specified identifier, or NULL if not found.
  //
  pub get_frame_byident: Option<extern "C" fn(this: *mut cef_browser_t,
      identifier: i64) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the frame with the specified name, or NULL if not found.
  //
  pub get_frame: Option<extern "C" fn(this: *mut cef_browser_t,
      name: *const types::cef_string_t) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the number of frames that currently exist.
  //
  pub get_frame_count: Option<extern "C" fn(
      this: *mut cef_browser_t) -> libc::size_t>,

  //
  // Returns the identifiers of all existing frames.
  //
  pub get_frame_identifiers: Option<extern "C" fn(this: *mut cef_browser_t,
      identifiers_count: *mut libc::size_t, identifiers: *mut i64) -> ()>,

  //
  // Returns the names of all existing frames.
  //
  pub get_frame_names: Option<extern "C" fn(this: *mut cef_browser_t,
      names: types::cef_string_list_t) -> ()>,

  //
  // Send a message to the specified |target_process|. Returns true (1) if the
  // message was sent successfully.
  //
  pub send_process_message: Option<extern "C" fn(this: *mut cef_browser_t,
      target_process: interfaces::cef_process_id_t,
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

pub type cef_browser_t = _cef_browser_t;


//
// Structure used to represent a browser window. When used in the browser
// process the functions of this structure may be called on any thread unless
// otherwise indicated in the comments. When used in the render process the
// functions of this structure may only be called on the main thread.
//
pub struct CefBrowser {
  c_object: *mut cef_browser_t,
}

impl Clone for CefBrowser {
  fn clone(&self) -> CefBrowser{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefBrowser {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefBrowser {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefBrowser {
  pub unsafe fn from_c_object(c_object: *mut cef_browser_t) -> CefBrowser {
    CefBrowser {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_browser_t) -> CefBrowser {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefBrowser {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_browser_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_browser_t {
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
  // Returns the browser host object. This function can only be called in the
  // browser process.
  //
  pub fn get_host(&self) -> interfaces::CefBrowserHost {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_host.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the browser can navigate backwards.
  //
  pub fn can_go_back(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).can_go_back.unwrap())(
          self.c_object))
    }
  }

  //
  // Navigate backwards.
  //
  pub fn go_back(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).go_back.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the browser can navigate forwards.
  //
  pub fn can_go_forward(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).can_go_forward.unwrap())(
          self.c_object))
    }
  }

  //
  // Navigate forwards.
  //
  pub fn go_forward(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).go_forward.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the browser is currently loading.
  //
  pub fn is_loading(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_loading.unwrap())(
          self.c_object))
    }
  }

  //
  // Reload the current page.
  //
  pub fn reload(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).reload.unwrap())(
          self.c_object))
    }
  }

  //
  // Reload the current page ignoring any cached data.
  //
  pub fn reload_ignore_cache(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).reload_ignore_cache.unwrap())(
          self.c_object))
    }
  }

  //
  // Stop loading the page.
  //
  pub fn stop_load(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).stop_load.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the globally unique identifier for this browser.
  //
  pub fn get_identifier(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_identifier.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub fn is_same(&self, that: interfaces::CefBrowser) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_same.unwrap())(
          self.c_object,
          CefWrap::to_c(that)))
    }
  }

  //
  // Returns true (1) if the window is a popup window.
  //
  pub fn is_popup(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_popup.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if a document has been loaded in the browser.
  //
  pub fn has_document(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_document.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the main (top-level) frame for the browser window.
  //
  pub fn get_main_frame(&self) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_main_frame.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the focused frame for the browser window.
  //
  pub fn get_focused_frame(&self) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_focused_frame.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the frame with the specified identifier, or NULL if not found.
  //
  pub fn get_frame_byident(&self, identifier: i64) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame_byident.unwrap())(
          self.c_object,
          CefWrap::to_c(identifier)))
    }
  }

  //
  // Returns the frame with the specified name, or NULL if not found.
  //
  pub fn get_frame(&self, name: &[u16]) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame.unwrap())(
          self.c_object,
          CefWrap::to_c(name)))
    }
  }

  //
  // Returns the number of frames that currently exist.
  //
  pub fn get_frame_count(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the identifiers of all existing frames.
  //
  pub fn get_frame_identifiers(&self, identifiers_count: *mut libc::size_t,
      identifiers: *mut i64) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame_identifiers.unwrap())(
          self.c_object,
          CefWrap::to_c(identifiers_count),
          CefWrap::to_c(identifiers)))
    }
  }

  //
  // Returns the names of all existing frames.
  //
  pub fn get_frame_names(&self, names: Vec<String>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame_names.unwrap())(
          self.c_object,
          CefWrap::to_c(names)))
    }
  }

  //
  // Send a message to the specified |target_process|. Returns true (1) if the
  // message was sent successfully.
  //
  pub fn send_process_message(&self, target_process: interfaces::CefProcessId,
      message: interfaces::CefProcessMessage) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_process_message.unwrap())(
          self.c_object,
          CefWrap::to_c(target_process),
          CefWrap::to_c(message)))
    }
  }
}

impl CefWrap<*mut cef_browser_t> for CefBrowser {
  fn to_c(rust_object: CefBrowser) -> *mut cef_browser_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_browser_t) -> CefBrowser {
    CefBrowser::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_browser_t> for Option<CefBrowser> {
  fn to_c(rust_object: Option<CefBrowser>) -> *mut cef_browser_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_browser_t) -> Option<CefBrowser> {
    if c_object.is_null() {
      None
    } else {
      Some(CefBrowser::from_c_object_addref(c_object))
    }
  }
}


//
// Callback structure for cef_browser_host_t::RunFileDialog. The functions of
// this structure will be called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_run_file_dialog_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called asynchronously after the file dialog is dismissed. If the selection
  // was successful |file_paths| will be a single value or a list of values
  // depending on the dialog mode. If the selection was cancelled |file_paths|
  // will be NULL.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_run_file_dialog_callback_t,
      browser_host: *mut interfaces::cef_browser_host_t,
      file_paths: types::cef_string_list_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_run_file_dialog_callback_t = _cef_run_file_dialog_callback_t;


//
// Callback structure for cef_browser_host_t::RunFileDialog. The functions of
// this structure will be called on the browser process UI thread.
//
pub struct CefRunFileDialogCallback {
  c_object: *mut cef_run_file_dialog_callback_t,
}

impl Clone for CefRunFileDialogCallback {
  fn clone(&self) -> CefRunFileDialogCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefRunFileDialogCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefRunFileDialogCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefRunFileDialogCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_run_file_dialog_callback_t) -> CefRunFileDialogCallback {
    CefRunFileDialogCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_run_file_dialog_callback_t) -> CefRunFileDialogCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefRunFileDialogCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_run_file_dialog_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_run_file_dialog_callback_t {
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
  // Called asynchronously after the file dialog is dismissed. If the selection
  // was successful |file_paths| will be a single value or a list of values
  // depending on the dialog mode. If the selection was cancelled |file_paths|
  // will be NULL.
  //
  pub fn cont(&self, browser_host: interfaces::CefBrowserHost,
      file_paths: Vec<String>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(browser_host),
          CefWrap::to_c(file_paths)))
    }
  }
}

impl CefWrap<*mut cef_run_file_dialog_callback_t> for CefRunFileDialogCallback {
  fn to_c(rust_object: CefRunFileDialogCallback) -> *mut cef_run_file_dialog_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_run_file_dialog_callback_t) -> CefRunFileDialogCallback {
    CefRunFileDialogCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_run_file_dialog_callback_t> for Option<CefRunFileDialogCallback> {
  fn to_c(rust_object: Option<CefRunFileDialogCallback>) -> *mut cef_run_file_dialog_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_run_file_dialog_callback_t) -> Option<CefRunFileDialogCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefRunFileDialogCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to represent the browser process aspects of a browser window.
// The functions of this structure can only be called in the browser process.
// They may be called on any thread in that process unless otherwise indicated
// in the comments.
//
#[repr(C)]
pub struct _cef_browser_host_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the hosted browser object.
  //
  pub get_browser: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> *mut interfaces::cef_browser_t>,

  //
  // Request that the browser close. The JavaScript 'onbeforeunload' event will
  // be fired. If |force_close| is false (0) the event handler, if any, will be
  // allowed to prompt the user and the user can optionally cancel the close. If
  // |force_close| is true (1) the prompt will not be displayed and the close
  // will proceed. Results in a call to cef_life_span_handler_t::do_close() if
  // the event handler allows the close or if |force_close| is true (1). See
  // cef_life_span_handler_t::do_close() documentation for additional usage
  // information.
  //
  pub close_browser: Option<extern "C" fn(this: *mut cef_browser_host_t,
      force_close: libc::c_int) -> ()>,

  //
  // Set whether the browser is focused.
  //
  pub set_focus: Option<extern "C" fn(this: *mut cef_browser_host_t,
      focus: libc::c_int) -> ()>,

  //
  // Set whether the window containing the browser is visible
  // (minimized/unminimized, app hidden/unhidden, etc). Only used on Mac OS X.
  //
  pub set_window_visibility: Option<extern "C" fn(this: *mut cef_browser_host_t,
      visible: libc::c_int) -> ()>,

  //
  // Retrieve the window handle for this browser.
  //
  pub get_window_handle: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> types::cef_window_handle_t>,

  //
  // Retrieve the window handle of the browser that opened this browser. Will
  // return NULL for non-popup windows. This function can be used in combination
  // with custom handling of modal windows.
  //
  pub get_opener_window_handle: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> types::cef_window_handle_t>,

  //
  // Returns the client for this browser.
  //
  pub get_client: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> *mut interfaces::cef_client_t>,

  //
  // Returns the request context for this browser.
  //
  pub get_request_context: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> *mut interfaces::cef_request_context_t>,

  //
  // Get the current zoom level. The default zoom level is 0.0. This function
  // can only be called on the UI thread.
  //
  pub get_zoom_level: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> libc::c_double>,

  //
  // Change the zoom level to the specified value. Specify 0.0 to reset the zoom
  // level. If called on the UI thread the change will be applied immediately.
  // Otherwise, the change will be applied asynchronously on the UI thread.
  //
  pub set_zoom_level: Option<extern "C" fn(this: *mut cef_browser_host_t,
      zoomLevel: libc::c_double) -> ()>,

  //
  // Call to run a file chooser dialog. Only a single file chooser dialog may be
  // pending at any given time. |mode| represents the type of dialog to display.
  // |title| to the title to be used for the dialog and may be NULL to show the
  // default title ("Open" or "Save" depending on the mode). |default_file_name|
  // is the default file name to select in the dialog. |accept_types| is a list
  // of valid lower-cased MIME types or file extensions specified in an input
  // element and is used to restrict selectable files to such types. |callback|
  // will be executed after the dialog is dismissed or immediately if another
  // dialog is already pending. The dialog will be initiated asynchronously on
  // the UI thread.
  //
  pub run_file_dialog: Option<extern "C" fn(this: *mut cef_browser_host_t,
      mode: types::cef_file_dialog_mode_t, title: *const types::cef_string_t,
      default_file_name: *const types::cef_string_t,
      accept_types: types::cef_string_list_t,
      callback: *mut interfaces::cef_run_file_dialog_callback_t) -> ()>,

  //
  // Download the file at |url| using cef_download_handler_t.
  //
  pub start_download: Option<extern "C" fn(this: *mut cef_browser_host_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Print the current browser contents.
  //
  pub print: Option<extern "C" fn(this: *mut cef_browser_host_t) -> ()>,

  //
  // Search for |searchText|. |identifier| can be used to have multiple searches
  // running simultaneously. |forward| indicates whether to search forward or
  // backward within the page. |matchCase| indicates whether the search should
  // be case-sensitive. |findNext| indicates whether this is the first request
  // or a follow-up.
  //
  pub find: Option<extern "C" fn(this: *mut cef_browser_host_t,
      identifier: libc::c_int, searchText: *const types::cef_string_t,
      forward: libc::c_int, matchCase: libc::c_int, findNext: libc::c_int) -> (
      )>,

  //
  // Cancel all searches that are currently going on.
  //
  pub stop_finding: Option<extern "C" fn(this: *mut cef_browser_host_t,
      clearSelection: libc::c_int) -> ()>,

  //
  // Open developer tools in its own window. If |inspect_element_at| is non-
  // NULL the element at the specified (x,y) location will be inspected.
  //
  pub show_dev_tools: Option<extern "C" fn(this: *mut cef_browser_host_t,
      windowInfo: *const interfaces::cef_window_info_t,
      client: *mut interfaces::cef_client_t,
      settings: *const interfaces::cef_browser_settings_t,
      inspect_element_at: *const types::cef_point_t) -> ()>,

  //
  // Explicitly close the developer tools window if one exists for this browser
  // instance.
  //
  pub close_dev_tools: Option<extern "C" fn(this: *mut cef_browser_host_t) -> (
      )>,

  //
  // Set whether mouse cursor change is disabled.
  //
  pub set_mouse_cursor_change_disabled: Option<extern "C" fn(
      this: *mut cef_browser_host_t, disabled: libc::c_int) -> ()>,

  //
  // Returns true (1) if mouse cursor change is disabled.
  //
  pub is_mouse_cursor_change_disabled: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> libc::c_int>,

  //
  // If a misspelled word is currently selected in an editable node calling this
  // function will replace it with the specified |word|.
  //
  pub replace_misspelling: Option<extern "C" fn(this: *mut cef_browser_host_t,
      word: *const types::cef_string_t) -> ()>,

  //
  // Add the specified |word| to the spelling dictionary.
  //
  pub add_word_to_dictionary: Option<extern "C" fn(
      this: *mut cef_browser_host_t, word: *const types::cef_string_t) -> ()>,

  //
  // Returns true (1) if window rendering is disabled.
  //
  pub is_window_rendering_disabled: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> libc::c_int>,

  //
  // Notify the browser that the widget has been resized. The browser will first
  // call cef_render_handler_t::GetViewRect to get the new size and then call
  // cef_render_handler_t::OnPaint asynchronously with the updated regions. This
  // function is only used when window rendering is disabled.
  //
  pub was_resized: Option<extern "C" fn(this: *mut cef_browser_host_t) -> ()>,

  //
  // Notify the browser that it has been hidden or shown. Layouting and
  // cef_render_handler_t::OnPaint notification will stop when the browser is
  // hidden. This function is only used when window rendering is disabled.
  //
  pub was_hidden: Option<extern "C" fn(this: *mut cef_browser_host_t,
      hidden: libc::c_int) -> ()>,

  //
  // Send a notification to the browser that the screen info has changed. The
  // browser will then call cef_render_handler_t::GetScreenInfo to update the
  // screen information with the new values. This simulates moving the webview
  // window from one display to another, or changing the properties of the
  // current display. This function is only used when window rendering is
  // disabled.
  //
  pub notify_screen_info_changed: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // Invalidate the view. The browser will call cef_render_handler_t::OnPaint
  // asynchronously. This function is only used when window rendering is
  // disabled.
  //
  pub invalidate: Option<extern "C" fn(this: *mut cef_browser_host_t,
      ty: types::cef_paint_element_type_t) -> ()>,

  //
  // Send a key event to the browser.
  //
  pub send_key_event: Option<extern "C" fn(this: *mut cef_browser_host_t,
      event: *const interfaces::cef_key_event_t) -> ()>,

  //
  // Send a mouse click event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view.
  //
  pub send_mouse_click_event: Option<extern "C" fn(
      this: *mut cef_browser_host_t,
      event: *const interfaces::cef_mouse_event_t,
      ty: types::cef_mouse_button_type_t, mouseUp: libc::c_int,
      clickCount: libc::c_int) -> ()>,

  //
  // Send a mouse move event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view.
  //
  pub send_mouse_move_event: Option<extern "C" fn(this: *mut cef_browser_host_t,
      event: *const interfaces::cef_mouse_event_t,
      mouseLeave: libc::c_int) -> ()>,

  //
  // Send a mouse wheel event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view. The |deltaX| and |deltaY|
  // values represent the movement delta in the X and Y directions respectively.
  // In order to scroll inside select popups with window rendering disabled
  // cef_render_handler_t::GetScreenPoint should be implemented properly.
  //
  pub send_mouse_wheel_event: Option<extern "C" fn(
      this: *mut cef_browser_host_t,
      event: *const interfaces::cef_mouse_event_t, deltaX: libc::c_int,
      deltaY: libc::c_int) -> ()>,

  //
  // Send a focus event to the browser.
  //
  pub send_focus_event: Option<extern "C" fn(this: *mut cef_browser_host_t,
      setFocus: libc::c_int) -> ()>,

  //
  // Send a capture lost event to the browser.
  //
  pub send_capture_lost_event: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // Notify the browser that the window hosting it is about to be moved or
  // resized. This function is only used on Windows and Linux.
  //
  pub notify_move_or_resize_started: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // Get the NSTextInputContext implementation for enabling IME on Mac when
  // window rendering is disabled.
  //
  pub get_nstext_input_context: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> types::cef_text_input_context_t>,

  //
  // Handles a keyDown event prior to passing it through the NSTextInputClient
  // machinery.
  //
  pub handle_key_event_before_text_input_client: Option<extern "C" fn(
      this: *mut cef_browser_host_t, keyEvent: types::cef_event_handle_t) -> (
      )>,

  //
  // Performs any additional actions after NSTextInputClient handles the event.
  //
  pub handle_key_event_after_text_input_client: Option<extern "C" fn(
      this: *mut cef_browser_host_t, keyEvent: types::cef_event_handle_t) -> (
      )>,

  //
  // Call this function when the user drags the mouse into the web view (before
  // calling DragTargetDragOver/DragTargetLeave/DragTargetDrop). |drag_data|
  // should not contain file contents as this type of data is not allowed to be
  // dragged into the web view. File contents can be removed using
  // cef_drag_data_t::ResetFileContents (for example, if |drag_data| comes from
  // cef_render_handler_t::StartDragging). This function is only used when
  // window rendering is disabled.
  //
  pub drag_target_drag_enter: Option<extern "C" fn(
      this: *mut cef_browser_host_t,
      drag_data: *mut interfaces::cef_drag_data_t,
      event: *const interfaces::cef_mouse_event_t,
      allowed_ops: types::cef_drag_operations_mask_t) -> ()>,

  //
  // Call this function each time the mouse is moved across the web view during
  // a drag operation (after calling DragTargetDragEnter and before calling
  // DragTargetDragLeave/DragTargetDrop). This function is only used when window
  // rendering is disabled.
  //
  pub drag_target_drag_over: Option<extern "C" fn(this: *mut cef_browser_host_t,
      event: *const interfaces::cef_mouse_event_t,
      allowed_ops: types::cef_drag_operations_mask_t) -> ()>,

  //
  // Call this function when the user drags the mouse out of the web view (after
  // calling DragTargetDragEnter). This function is only used when window
  // rendering is disabled.
  //
  pub drag_target_drag_leave: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // Call this function when the user completes the drag operation by dropping
  // the object onto the web view (after calling DragTargetDragEnter). The
  // object being dropped is |drag_data|, given as an argument to the previous
  // DragTargetDragEnter call. This function is only used when window rendering
  // is disabled.
  //
  pub drag_target_drop: Option<extern "C" fn(this: *mut cef_browser_host_t,
      event: *const interfaces::cef_mouse_event_t) -> ()>,

  //
  // Call this function when the drag operation started by a
  // cef_render_handler_t::StartDragging call has ended either in a drop or by
  // being cancelled. |x| and |y| are mouse coordinates relative to the upper-
  // left corner of the view. If the web view is both the drag source and the
  // drag target then all DragTarget* functions should be called before
  // DragSource* methods. This function is only used when window rendering is
  // disabled.
  //
  pub drag_source_ended_at: Option<extern "C" fn(this: *mut cef_browser_host_t,
      x: libc::c_int, y: libc::c_int,
      op: types::cef_drag_operations_mask_t) -> ()>,

  //
  // Call this function when the drag operation started by a
  // cef_render_handler_t::StartDragging call has completed. This function may
  // be called immediately without first calling DragSourceEndedAt to cancel a
  // drag operation. If the web view is both the drag source and the drag target
  // then all DragTarget* functions should be called before DragSource* methods.
  // This function is only used when window rendering is disabled.
  //
  pub drag_source_system_drag_ended: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // Instructs the browser to initialize accelerated compositing. The
  // appropriate Direct3D or OpenGL state must have been set up before calling
  // this function.
  //
  pub initialize_compositing: Option<extern "C" fn(
      this: *mut cef_browser_host_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_browser_host_t = _cef_browser_host_t;


//
// Structure used to represent the browser process aspects of a browser window.
// The functions of this structure can only be called in the browser process.
// They may be called on any thread in that process unless otherwise indicated
// in the comments.
//
pub struct CefBrowserHost {
  c_object: *mut cef_browser_host_t,
}

impl Clone for CefBrowserHost {
  fn clone(&self) -> CefBrowserHost{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefBrowserHost {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefBrowserHost {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefBrowserHost {
  pub unsafe fn from_c_object(c_object: *mut cef_browser_host_t) -> CefBrowserHost {
    CefBrowserHost {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_browser_host_t) -> CefBrowserHost {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefBrowserHost {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_browser_host_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_browser_host_t {
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
  // Returns the hosted browser object.
  //
  pub fn get_browser(&self) -> interfaces::CefBrowser {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_browser.unwrap())(
          self.c_object))
    }
  }

  //
  // Request that the browser close. The JavaScript 'onbeforeunload' event will
  // be fired. If |force_close| is false (0) the event handler, if any, will be
  // allowed to prompt the user and the user can optionally cancel the close. If
  // |force_close| is true (1) the prompt will not be displayed and the close
  // will proceed. Results in a call to cef_life_span_handler_t::do_close() if
  // the event handler allows the close or if |force_close| is true (1). See
  // cef_life_span_handler_t::do_close() documentation for additional usage
  // information.
  //
  pub fn close_browser(&self, force_close: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).close_browser.unwrap())(
          self.c_object,
          CefWrap::to_c(force_close)))
    }
  }

  //
  // Set whether the browser is focused.
  //
  pub fn set_focus(&self, focus: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_focus.unwrap())(
          self.c_object,
          CefWrap::to_c(focus)))
    }
  }

  //
  // Set whether the window containing the browser is visible
  // (minimized/unminimized, app hidden/unhidden, etc). Only used on Mac OS X.
  //
  pub fn set_window_visibility(&self, visible: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_window_visibility.unwrap())(
          self.c_object,
          CefWrap::to_c(visible)))
    }
  }

  //
  // Retrieve the window handle for this browser.
  //
  pub fn get_window_handle(&self) -> types::cef_window_handle_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_window_handle.unwrap())(
          self.c_object))
    }
  }

  //
  // Retrieve the window handle of the browser that opened this browser. Will
  // return NULL for non-popup windows. This function can be used in combination
  // with custom handling of modal windows.
  //
  pub fn get_opener_window_handle(&self) -> types::cef_window_handle_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_opener_window_handle.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the client for this browser.
  //
  pub fn get_client(&self) -> interfaces::CefClient {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_client.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the request context for this browser.
  //
  pub fn get_request_context(&self) -> interfaces::CefRequestContext {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_request_context.unwrap())(
          self.c_object))
    }
  }

  //
  // Get the current zoom level. The default zoom level is 0.0. This function
  // can only be called on the UI thread.
  //
  pub fn get_zoom_level(&self) -> libc::c_double {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_zoom_level.unwrap())(
          self.c_object))
    }
  }

  //
  // Change the zoom level to the specified value. Specify 0.0 to reset the zoom
  // level. If called on the UI thread the change will be applied immediately.
  // Otherwise, the change will be applied asynchronously on the UI thread.
  //
  pub fn set_zoom_level(&self, zoomLevel: libc::c_double) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_zoom_level.unwrap())(
          self.c_object,
          CefWrap::to_c(zoomLevel)))
    }
  }

  //
  // Call to run a file chooser dialog. Only a single file chooser dialog may be
  // pending at any given time. |mode| represents the type of dialog to display.
  // |title| to the title to be used for the dialog and may be NULL to show the
  // default title ("Open" or "Save" depending on the mode). |default_file_name|
  // is the default file name to select in the dialog. |accept_types| is a list
  // of valid lower-cased MIME types or file extensions specified in an input
  // element and is used to restrict selectable files to such types. |callback|
  // will be executed after the dialog is dismissed or immediately if another
  // dialog is already pending. The dialog will be initiated asynchronously on
  // the UI thread.
  //
  pub fn run_file_dialog(&self, mode: types::cef_file_dialog_mode_t,
      title: &[u16], default_file_name: &[u16], accept_types: Vec<String>,
      callback: interfaces::CefRunFileDialogCallback) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).run_file_dialog.unwrap())(
          self.c_object,
          CefWrap::to_c(mode),
          CefWrap::to_c(title),
          CefWrap::to_c(default_file_name),
          CefWrap::to_c(accept_types),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Download the file at |url| using cef_download_handler_t.
  //
  pub fn start_download(&self, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).start_download.unwrap())(
          self.c_object,
          CefWrap::to_c(url)))
    }
  }

  //
  // Print the current browser contents.
  //
  pub fn print(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).print.unwrap())(
          self.c_object))
    }
  }

  //
  // Search for |searchText|. |identifier| can be used to have multiple searches
  // running simultaneously. |forward| indicates whether to search forward or
  // backward within the page. |matchCase| indicates whether the search should
  // be case-sensitive. |findNext| indicates whether this is the first request
  // or a follow-up.
  //
  pub fn find(&self, identifier: libc::c_int, searchText: &[u16],
      forward: libc::c_int, matchCase: libc::c_int, findNext: libc::c_int) -> (
      ) {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).find.unwrap())(
          self.c_object,
          CefWrap::to_c(identifier),
          CefWrap::to_c(searchText),
          CefWrap::to_c(forward),
          CefWrap::to_c(matchCase),
          CefWrap::to_c(findNext)))
    }
  }

  //
  // Cancel all searches that are currently going on.
  //
  pub fn stop_finding(&self, clearSelection: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).stop_finding.unwrap())(
          self.c_object,
          CefWrap::to_c(clearSelection)))
    }
  }

  //
  // Open developer tools in its own window. If |inspect_element_at| is non-
  // NULL the element at the specified (x,y) location will be inspected.
  //
  pub fn show_dev_tools(&self, windowInfo: &interfaces::CefWindowInfo,
      client: interfaces::CefClient, settings: &interfaces::CefBrowserSettings,
      inspect_element_at: &types::cef_point_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).show_dev_tools.unwrap())(
          self.c_object,
          CefWrap::to_c(windowInfo),
          CefWrap::to_c(client),
          CefWrap::to_c(settings),
          CefWrap::to_c(inspect_element_at)))
    }
  }

  //
  // Explicitly close the developer tools window if one exists for this browser
  // instance.
  //
  pub fn close_dev_tools(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).close_dev_tools.unwrap())(
          self.c_object))
    }
  }

  //
  // Set whether mouse cursor change is disabled.
  //
  pub fn set_mouse_cursor_change_disabled(&self, disabled: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_mouse_cursor_change_disabled.unwrap())(
          self.c_object,
          CefWrap::to_c(disabled)))
    }
  }

  //
  // Returns true (1) if mouse cursor change is disabled.
  //
  pub fn is_mouse_cursor_change_disabled(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_mouse_cursor_change_disabled.unwrap())(
          self.c_object))
    }
  }

  //
  // If a misspelled word is currently selected in an editable node calling this
  // function will replace it with the specified |word|.
  //
  pub fn replace_misspelling(&self, word: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).replace_misspelling.unwrap())(
          self.c_object,
          CefWrap::to_c(word)))
    }
  }

  //
  // Add the specified |word| to the spelling dictionary.
  //
  pub fn add_word_to_dictionary(&self, word: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_word_to_dictionary.unwrap())(
          self.c_object,
          CefWrap::to_c(word)))
    }
  }

  //
  // Returns true (1) if window rendering is disabled.
  //
  pub fn is_window_rendering_disabled(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_window_rendering_disabled.unwrap())(
          self.c_object))
    }
  }

  //
  // Notify the browser that the widget has been resized. The browser will first
  // call cef_render_handler_t::GetViewRect to get the new size and then call
  // cef_render_handler_t::OnPaint asynchronously with the updated regions. This
  // function is only used when window rendering is disabled.
  //
  pub fn was_resized(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).was_resized.unwrap())(
          self.c_object))
    }
  }

  //
  // Notify the browser that it has been hidden or shown. Layouting and
  // cef_render_handler_t::OnPaint notification will stop when the browser is
  // hidden. This function is only used when window rendering is disabled.
  //
  pub fn was_hidden(&self, hidden: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).was_hidden.unwrap())(
          self.c_object,
          CefWrap::to_c(hidden)))
    }
  }

  //
  // Send a notification to the browser that the screen info has changed. The
  // browser will then call cef_render_handler_t::GetScreenInfo to update the
  // screen information with the new values. This simulates moving the webview
  // window from one display to another, or changing the properties of the
  // current display. This function is only used when window rendering is
  // disabled.
  //
  pub fn notify_screen_info_changed(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).notify_screen_info_changed.unwrap())(
          self.c_object))
    }
  }

  //
  // Invalidate the view. The browser will call cef_render_handler_t::OnPaint
  // asynchronously. This function is only used when window rendering is
  // disabled.
  //
  pub fn invalidate(&self, ty: types::cef_paint_element_type_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).invalidate.unwrap())(
          self.c_object,
          CefWrap::to_c(ty)))
    }
  }

  //
  // Send a key event to the browser.
  //
  pub fn send_key_event(&self, event: &interfaces::CefKeyEvent) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_key_event.unwrap())(
          self.c_object,
          CefWrap::to_c(event)))
    }
  }

  //
  // Send a mouse click event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view.
  //
  pub fn send_mouse_click_event(&self, event: &interfaces::CefMouseEvent,
      ty: types::cef_mouse_button_type_t, mouseUp: libc::c_int,
      clickCount: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_mouse_click_event.unwrap())(
          self.c_object,
          CefWrap::to_c(event),
          CefWrap::to_c(ty),
          CefWrap::to_c(mouseUp),
          CefWrap::to_c(clickCount)))
    }
  }

  //
  // Send a mouse move event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view.
  //
  pub fn send_mouse_move_event(&self, event: &interfaces::CefMouseEvent,
      mouseLeave: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_mouse_move_event.unwrap())(
          self.c_object,
          CefWrap::to_c(event),
          CefWrap::to_c(mouseLeave)))
    }
  }

  //
  // Send a mouse wheel event to the browser. The |x| and |y| coordinates are
  // relative to the upper-left corner of the view. The |deltaX| and |deltaY|
  // values represent the movement delta in the X and Y directions respectively.
  // In order to scroll inside select popups with window rendering disabled
  // cef_render_handler_t::GetScreenPoint should be implemented properly.
  //
  pub fn send_mouse_wheel_event(&self, event: &interfaces::CefMouseEvent,
      deltaX: libc::c_int, deltaY: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_mouse_wheel_event.unwrap())(
          self.c_object,
          CefWrap::to_c(event),
          CefWrap::to_c(deltaX),
          CefWrap::to_c(deltaY)))
    }
  }

  //
  // Send a focus event to the browser.
  //
  pub fn send_focus_event(&self, setFocus: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_focus_event.unwrap())(
          self.c_object,
          CefWrap::to_c(setFocus)))
    }
  }

  //
  // Send a capture lost event to the browser.
  //
  pub fn send_capture_lost_event(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).send_capture_lost_event.unwrap())(
          self.c_object))
    }
  }

  //
  // Notify the browser that the window hosting it is about to be moved or
  // resized. This function is only used on Windows and Linux.
  //
  pub fn notify_move_or_resize_started(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).notify_move_or_resize_started.unwrap())(
          self.c_object))
    }
  }

  //
  // Get the NSTextInputContext implementation for enabling IME on Mac when
  // window rendering is disabled.
  //
  pub fn get_nstext_input_context(&self) -> types::cef_text_input_context_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_nstext_input_context.unwrap())(
          self.c_object))
    }
  }

  //
  // Handles a keyDown event prior to passing it through the NSTextInputClient
  // machinery.
  //
  pub fn handle_key_event_before_text_input_client(&self,
      keyEvent: types::cef_event_handle_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).handle_key_event_before_text_input_client.unwrap())(
          self.c_object,
          CefWrap::to_c(keyEvent)))
    }
  }

  //
  // Performs any additional actions after NSTextInputClient handles the event.
  //
  pub fn handle_key_event_after_text_input_client(&self,
      keyEvent: types::cef_event_handle_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).handle_key_event_after_text_input_client.unwrap())(
          self.c_object,
          CefWrap::to_c(keyEvent)))
    }
  }

  //
  // Call this function when the user drags the mouse into the web view (before
  // calling DragTargetDragOver/DragTargetLeave/DragTargetDrop). |drag_data|
  // should not contain file contents as this type of data is not allowed to be
  // dragged into the web view. File contents can be removed using
  // cef_drag_data_t::ResetFileContents (for example, if |drag_data| comes from
  // cef_render_handler_t::StartDragging). This function is only used when
  // window rendering is disabled.
  //
  pub fn drag_target_drag_enter(&self, drag_data: interfaces::CefDragData,
      event: &interfaces::CefMouseEvent,
      allowed_ops: types::cef_drag_operations_mask_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_target_drag_enter.unwrap())(
          self.c_object,
          CefWrap::to_c(drag_data),
          CefWrap::to_c(event),
          CefWrap::to_c(allowed_ops)))
    }
  }

  //
  // Call this function each time the mouse is moved across the web view during
  // a drag operation (after calling DragTargetDragEnter and before calling
  // DragTargetDragLeave/DragTargetDrop). This function is only used when window
  // rendering is disabled.
  //
  pub fn drag_target_drag_over(&self, event: &interfaces::CefMouseEvent,
      allowed_ops: types::cef_drag_operations_mask_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_target_drag_over.unwrap())(
          self.c_object,
          CefWrap::to_c(event),
          CefWrap::to_c(allowed_ops)))
    }
  }

  //
  // Call this function when the user drags the mouse out of the web view (after
  // calling DragTargetDragEnter). This function is only used when window
  // rendering is disabled.
  //
  pub fn drag_target_drag_leave(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_target_drag_leave.unwrap())(
          self.c_object))
    }
  }

  //
  // Call this function when the user completes the drag operation by dropping
  // the object onto the web view (after calling DragTargetDragEnter). The
  // object being dropped is |drag_data|, given as an argument to the previous
  // DragTargetDragEnter call. This function is only used when window rendering
  // is disabled.
  //
  pub fn drag_target_drop(&self, event: &interfaces::CefMouseEvent) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_target_drop.unwrap())(
          self.c_object,
          CefWrap::to_c(event)))
    }
  }

  //
  // Call this function when the drag operation started by a
  // cef_render_handler_t::StartDragging call has ended either in a drop or by
  // being cancelled. |x| and |y| are mouse coordinates relative to the upper-
  // left corner of the view. If the web view is both the drag source and the
  // drag target then all DragTarget* functions should be called before
  // DragSource* methods. This function is only used when window rendering is
  // disabled.
  //
  pub fn drag_source_ended_at(&self, x: libc::c_int, y: libc::c_int,
      op: types::cef_drag_operations_mask_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_source_ended_at.unwrap())(
          self.c_object,
          CefWrap::to_c(x),
          CefWrap::to_c(y),
          CefWrap::to_c(op)))
    }
  }

  //
  // Call this function when the drag operation started by a
  // cef_render_handler_t::StartDragging call has completed. This function may
  // be called immediately without first calling DragSourceEndedAt to cancel a
  // drag operation. If the web view is both the drag source and the drag target
  // then all DragTarget* functions should be called before DragSource* methods.
  // This function is only used when window rendering is disabled.
  //
  pub fn drag_source_system_drag_ended(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).drag_source_system_drag_ended.unwrap())(
          self.c_object))
    }
  }

  //
  // Instructs the browser to initialize accelerated compositing. The
  // appropriate Direct3D or OpenGL state must have been set up before calling
  // this function.
  //
  pub fn initialize_compositing(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).initialize_compositing.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new browser window using the window parameters specified by
  // |windowInfo|. All values will be copied internally and the actual window
  // will be created on the UI thread. If |request_context| is NULL the global
  // request context will be used. This function can be called on any browser
  // process thread and will not block.
  //
  pub fn create_browser(windowInfo: &interfaces::CefWindowInfo,
      client: interfaces::CefClient, url: &[u16],
      settings: &interfaces::CefBrowserSettings,
      request_context: interfaces::CefRequestContext) -> libc::c_int {
    unsafe {
      CefWrap::to_rust(
        ::browser::cef_browser_host_create_browser(
          CefWrap::to_c(windowInfo),
          CefWrap::to_c(client),
          CefWrap::to_c(url),
          CefWrap::to_c(settings),
          CefWrap::to_c(request_context)))
    }
  }

  //
  // Create a new browser window using the window parameters specified by
  // |windowInfo|. If |request_context| is NULL the global request context will
  // be used. This function can only be called on the browser process UI thread.
  //
  pub fn create_browser_sync(windowInfo: &interfaces::CefWindowInfo,
      client: interfaces::CefClient, url: &[u16],
      settings: &interfaces::CefBrowserSettings,
      request_context: interfaces::CefRequestContext) -> interfaces::CefBrowser {
    unsafe {
      CefWrap::to_rust(
        ::browser::cef_browser_host_create_browser_sync(
          CefWrap::to_c(windowInfo),
          CefWrap::to_c(client),
          CefWrap::to_c(url),
          CefWrap::to_c(settings),
          CefWrap::to_c(request_context)))
    }
  }
}

impl CefWrap<*mut cef_browser_host_t> for CefBrowserHost {
  fn to_c(rust_object: CefBrowserHost) -> *mut cef_browser_host_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_browser_host_t) -> CefBrowserHost {
    CefBrowserHost::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_browser_host_t> for Option<CefBrowserHost> {
  fn to_c(rust_object: Option<CefBrowserHost>) -> *mut cef_browser_host_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_browser_host_t) -> Option<CefBrowserHost> {
    if c_object.is_null() {
      None
    } else {
      Some(CefBrowserHost::from_c_object_addref(c_object))
    }
  }
}

