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
// Structure used to represent a frame in the browser window. When used in the
// browser process the functions of this structure may be called on any thread
// unless otherwise indicated in the comments. When used in the render process
// the functions of this structure may only be called on the main thread.
//
#[repr(C)]
pub struct _cef_frame_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // True if this object is currently attached to a valid frame.
  //
  pub is_valid: Option<extern "C" fn(this: *mut cef_frame_t) -> libc::c_int>,

  //
  // Execute undo in this frame.
  //
  pub undo: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute redo in this frame.
  //
  pub redo: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute cut in this frame.
  //
  pub cut: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute copy in this frame.
  //
  pub copy: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute paste in this frame.
  //
  pub paste: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute delete in this frame.
  //
  pub del: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Execute select all in this frame.
  //
  pub select_all: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Save this frame's HTML source to a temporary file and open it in the
  // default text viewing application. This function can only be called from the
  // browser process.
  //
  pub view_source: Option<extern "C" fn(this: *mut cef_frame_t) -> ()>,

  //
  // Retrieve this frame's HTML source as a string sent to the specified
  // visitor.
  //
  pub get_source: Option<extern "C" fn(this: *mut cef_frame_t,
      visitor: *mut interfaces::cef_string_visitor_t) -> ()>,

  //
  // Retrieve this frame's display text as a string sent to the specified
  // visitor.
  //
  pub get_text: Option<extern "C" fn(this: *mut cef_frame_t,
      visitor: *mut interfaces::cef_string_visitor_t) -> ()>,

  //
  // Load the request represented by the |request| object.
  //
  pub load_request: Option<extern "C" fn(this: *mut cef_frame_t,
      request: *mut interfaces::cef_request_t) -> ()>,

  //
  // Load the specified |url|.
  //
  pub load_url: Option<extern "C" fn(this: *mut cef_frame_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Load the contents of |string_val| with the specified dummy |url|. |url|
  // should have a standard scheme (for example, http scheme) or behaviors like
  // link clicks and web security restrictions may not behave as expected.
  //
  pub load_string: Option<extern "C" fn(this: *mut cef_frame_t,
      string_val: *const types::cef_string_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Execute a string of JavaScript code in this frame. The |script_url|
  // parameter is the URL where the script in question can be found, if any. The
  // renderer may request this URL to show the developer the source of the
  // error.  The |start_line| parameter is the base line number to use for error
  // reporting.
  //
  pub execute_java_script: Option<extern "C" fn(this: *mut cef_frame_t,
      code: *const types::cef_string_t, script_url: *const types::cef_string_t,
      start_line: libc::c_int) -> ()>,

  //
  // Returns true (1) if this is the main (top-level) frame.
  //
  pub is_main: Option<extern "C" fn(this: *mut cef_frame_t) -> libc::c_int>,

  //
  // Returns true (1) if this is the focused frame.
  //
  pub is_focused: Option<extern "C" fn(this: *mut cef_frame_t) -> libc::c_int>,

  //
  // Returns the name for this frame. If the frame has an assigned name (for
  // example, set via the iframe "name" attribute) then that value will be
  // returned. Otherwise a unique name will be constructed based on the frame
  // parent hierarchy. The main (top-level) frame will always have an NULL name
  // value.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_name: Option<extern "C" fn(
      this: *mut cef_frame_t) -> types::cef_string_userfree_t>,

  //
  // Returns the globally unique identifier for this frame.
  //
  pub get_identifier: Option<extern "C" fn(this: *mut cef_frame_t) -> i64>,

  //
  // Returns the parent of this frame or NULL if this is the main (top-level)
  // frame.
  //
  pub get_parent: Option<extern "C" fn(
      this: *mut cef_frame_t) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the URL currently loaded in this frame.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_url: Option<extern "C" fn(
      this: *mut cef_frame_t) -> types::cef_string_userfree_t>,

  //
  // Returns the browser that this frame belongs to.
  //
  pub get_browser: Option<extern "C" fn(
      this: *mut cef_frame_t) -> *mut interfaces::cef_browser_t>,

  //
  // Get the V8 context associated with the frame. This function can only be
  // called from the render process.
  //
  pub get_v8context: Option<extern "C" fn(
      this: *mut cef_frame_t) -> *mut interfaces::cef_v8context_t>,

  //
  // Visit the DOM document. This function can only be called from the render
  // process.
  //
  pub visit_dom: Option<extern "C" fn(this: *mut cef_frame_t,
      visitor: *mut interfaces::cef_domvisitor_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_frame_t = _cef_frame_t;


//
// Structure used to represent a frame in the browser window. When used in the
// browser process the functions of this structure may be called on any thread
// unless otherwise indicated in the comments. When used in the render process
// the functions of this structure may only be called on the main thread.
//
pub struct CefFrame {
  c_object: *mut cef_frame_t,
}

impl Clone for CefFrame {
  fn clone(&self) -> CefFrame{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefFrame {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefFrame {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefFrame {
  pub unsafe fn from_c_object(c_object: *mut cef_frame_t) -> CefFrame {
    CefFrame {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_frame_t) -> CefFrame {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefFrame {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_frame_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_frame_t {
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
  // True if this object is currently attached to a valid frame.
  //
  pub fn is_valid(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_valid.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute undo in this frame.
  //
  pub fn undo(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).undo.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute redo in this frame.
  //
  pub fn redo(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).redo.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute cut in this frame.
  //
  pub fn cut(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cut.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute copy in this frame.
  //
  pub fn copy(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).copy.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute paste in this frame.
  //
  pub fn paste(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).paste.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute delete in this frame.
  //
  pub fn del(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).del.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute select all in this frame.
  //
  pub fn select_all(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).select_all.unwrap())(
          self.c_object))
    }
  }

  //
  // Save this frame's HTML source to a temporary file and open it in the
  // default text viewing application. This function can only be called from the
  // browser process.
  //
  pub fn view_source(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).view_source.unwrap())(
          self.c_object))
    }
  }

  //
  // Retrieve this frame's HTML source as a string sent to the specified
  // visitor.
  //
  pub fn get_source(&self, visitor: interfaces::CefStringVisitor) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_source.unwrap())(
          self.c_object,
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Retrieve this frame's display text as a string sent to the specified
  // visitor.
  //
  pub fn get_text(&self, visitor: interfaces::CefStringVisitor) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_text.unwrap())(
          self.c_object,
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Load the request represented by the |request| object.
  //
  pub fn load_request(&self, request: interfaces::CefRequest) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).load_request.unwrap())(
          self.c_object,
          CefWrap::to_c(request)))
    }
  }

  //
  // Load the specified |url|.
  //
  pub fn load_url(&self, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).load_url.unwrap())(
          self.c_object,
          CefWrap::to_c(url)))
    }
  }

  //
  // Load the contents of |string_val| with the specified dummy |url|. |url|
  // should have a standard scheme (for example, http scheme) or behaviors like
  // link clicks and web security restrictions may not behave as expected.
  //
  pub fn load_string(&self, string_val: &[u16], url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).load_string.unwrap())(
          self.c_object,
          CefWrap::to_c(string_val),
          CefWrap::to_c(url)))
    }
  }

  //
  // Execute a string of JavaScript code in this frame. The |script_url|
  // parameter is the URL where the script in question can be found, if any. The
  // renderer may request this URL to show the developer the source of the
  // error.  The |start_line| parameter is the base line number to use for error
  // reporting.
  //
  pub fn execute_java_script(&self, code: &[u16], script_url: &[u16],
      start_line: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).execute_java_script.unwrap())(
          self.c_object,
          CefWrap::to_c(code),
          CefWrap::to_c(script_url),
          CefWrap::to_c(start_line)))
    }
  }

  //
  // Returns true (1) if this is the main (top-level) frame.
  //
  pub fn is_main(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_main.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this is the focused frame.
  //
  pub fn is_focused(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_focused.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the name for this frame. If the frame has an assigned name (for
  // example, set via the iframe "name" attribute) then that value will be
  // returned. Otherwise a unique name will be constructed based on the frame
  // parent hierarchy. The main (top-level) frame will always have an NULL name
  // value.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the globally unique identifier for this frame.
  //
  pub fn get_identifier(&self) -> i64 {
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
  // Returns the parent of this frame or NULL if this is the main (top-level)
  // frame.
  //
  pub fn get_parent(&self) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_parent.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the URL currently loaded in this frame.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_url(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_url.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the browser that this frame belongs to.
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
  // Get the V8 context associated with the frame. This function can only be
  // called from the render process.
  //
  pub fn get_v8context(&self) -> interfaces::CefV8Context {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_v8context.unwrap())(
          self.c_object))
    }
  }

  //
  // Visit the DOM document. This function can only be called from the render
  // process.
  //
  pub fn visit_dom(&self, visitor: interfaces::CefDOMVisitor) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit_dom.unwrap())(
          self.c_object,
          CefWrap::to_c(visitor)))
    }
  }
}

impl CefWrap<*mut cef_frame_t> for CefFrame {
  fn to_c(rust_object: CefFrame) -> *mut cef_frame_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_frame_t) -> CefFrame {
    CefFrame::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_frame_t> for Option<CefFrame> {
  fn to_c(rust_object: Option<CefFrame>) -> *mut cef_frame_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_frame_t) -> Option<CefFrame> {
    if c_object.is_null() {
      None
    } else {
      Some(CefFrame::from_c_object_addref(c_object))
    }
  }
}

