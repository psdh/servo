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
// Structure representing a V8 context handle. V8 handles can only be accessed
// from the thread on which they are created. Valid threads for creating a V8
// handle include the render process main thread (TID_RENDERER) and WebWorker
// threads. A task runner for posting tasks on the associated thread can be
// retrieved via the cef_v8context_t::get_task_runner() function.
//
#[repr(C)]
pub struct _cef_v8context_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the task runner associated with this context. V8 handles can only
  // be accessed from the thread on which they are created. This function can be
  // called on any render process thread.
  //
  pub get_task_runner: Option<extern "C" fn(
      this: *mut cef_v8context_t) -> *mut interfaces::cef_task_runner_t>,

  //
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_v8context_t) -> libc::c_int>,

  //
  // Returns the browser for this context. This function will return an NULL
  // reference for WebWorker contexts.
  //
  pub get_browser: Option<extern "C" fn(
      this: *mut cef_v8context_t) -> *mut interfaces::cef_browser_t>,

  //
  // Returns the frame for this context. This function will return an NULL
  // reference for WebWorker contexts.
  //
  pub get_frame: Option<extern "C" fn(
      this: *mut cef_v8context_t) -> *mut interfaces::cef_frame_t>,

  //
  // Returns the global object for this context. The context must be entered
  // before calling this function.
  //
  pub get_global: Option<extern "C" fn(
      this: *mut cef_v8context_t) -> *mut interfaces::cef_v8value_t>,

  //
  // Enter this context. A context must be explicitly entered before creating a
  // V8 Object, Array, Function or Date asynchronously. exit() must be called
  // the same number of times as enter() before releasing this context. V8
  // objects belong to the context in which they are created. Returns true (1)
  // if the scope was entered successfully.
  //
  pub enter: Option<extern "C" fn(this: *mut cef_v8context_t) -> libc::c_int>,

  //
  // Exit this context. Call this function only after calling enter(). Returns
  // true (1) if the scope was exited successfully.
  //
  pub exit: Option<extern "C" fn(this: *mut cef_v8context_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub is_same: Option<extern "C" fn(this: *mut cef_v8context_t,
      that: *mut interfaces::cef_v8context_t) -> libc::c_int>,

  //
  // Evaluates the specified JavaScript code using this context's global object.
  // On success |retval| will be set to the return value, if any, and the
  // function will return true (1). On failure |exception| will be set to the
  // exception, if any, and the function will return false (0).
  //
  pub eval: Option<extern "C" fn(this: *mut cef_v8context_t,
      code: *const types::cef_string_t, retval: *mut interfaces::cef_v8value_t,
      exception: *mut interfaces::cef_v8exception_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8context_t = _cef_v8context_t;


//
// Structure representing a V8 context handle. V8 handles can only be accessed
// from the thread on which they are created. Valid threads for creating a V8
// handle include the render process main thread (TID_RENDERER) and WebWorker
// threads. A task runner for posting tasks on the associated thread can be
// retrieved via the cef_v8context_t::get_task_runner() function.
//
pub struct CefV8Context {
  c_object: *mut cef_v8context_t,
}

impl Clone for CefV8Context {
  fn clone(&self) -> CefV8Context{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8Context {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8Context {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8Context {
  pub unsafe fn from_c_object(c_object: *mut cef_v8context_t) -> CefV8Context {
    CefV8Context {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8context_t) -> CefV8Context {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8Context {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8context_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8context_t {
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
  // Returns the task runner associated with this context. V8 handles can only
  // be accessed from the thread on which they are created. This function can be
  // called on any render process thread.
  //
  pub fn get_task_runner(&self) -> interfaces::CefTaskRunner {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_task_runner.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
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
  // Returns the browser for this context. This function will return an NULL
  // reference for WebWorker contexts.
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
  // Returns the frame for this context. This function will return an NULL
  // reference for WebWorker contexts.
  //
  pub fn get_frame(&self) -> interfaces::CefFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the global object for this context. The context must be entered
  // before calling this function.
  //
  pub fn get_global(&self) -> interfaces::CefV8Value {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_global.unwrap())(
          self.c_object))
    }
  }

  //
  // Enter this context. A context must be explicitly entered before creating a
  // V8 Object, Array, Function or Date asynchronously. exit() must be called
  // the same number of times as enter() before releasing this context. V8
  // objects belong to the context in which they are created. Returns true (1)
  // if the scope was entered successfully.
  //
  pub fn enter(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).enter.unwrap())(
          self.c_object))
    }
  }

  //
  // Exit this context. Call this function only after calling enter(). Returns
  // true (1) if the scope was exited successfully.
  //
  pub fn exit(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).exit.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub fn is_same(&self, that: interfaces::CefV8Context) -> libc::c_int {
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
  // Evaluates the specified JavaScript code using this context's global object.
  // On success |retval| will be set to the return value, if any, and the
  // function will return true (1). On failure |exception| will be set to the
  // exception, if any, and the function will return false (0).
  //
  pub fn eval(&self, code: &[u16], retval: interfaces::CefV8Value,
      exception: interfaces::CefV8Exception) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).eval.unwrap())(
          self.c_object,
          CefWrap::to_c(code),
          CefWrap::to_c(retval),
          CefWrap::to_c(exception)))
    }
  }

  //
  // Returns the current (top) context object in the V8 context stack.
  //
  pub fn get_current_context() -> interfaces::CefV8Context {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8context_get_current_context(
))
    }
  }

  //
  // Returns the entered (bottom) context object in the V8 context stack.
  //
  pub fn get_entered_context() -> interfaces::CefV8Context {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8context_get_entered_context(
))
    }
  }

  //
  // Returns true (1) if V8 is currently inside a context.
  //
  pub fn in_context() -> libc::c_int {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8context_in_context(
))
    }
  }
}

impl CefWrap<*mut cef_v8context_t> for CefV8Context {
  fn to_c(rust_object: CefV8Context) -> *mut cef_v8context_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8context_t) -> CefV8Context {
    CefV8Context::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8context_t> for Option<CefV8Context> {
  fn to_c(rust_object: Option<CefV8Context>) -> *mut cef_v8context_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8context_t) -> Option<CefV8Context> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8Context::from_c_object_addref(c_object))
    }
  }
}


//
// Structure that should be implemented to handle V8 function calls. The
// functions of this structure will be called on the thread associated with the
// V8 function.
//
#[repr(C)]
pub struct _cef_v8handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Handle execution of the function identified by |name|. |object| is the
  // receiver ('this' object) of the function. |arguments| is the list of
  // arguments passed to the function. If execution succeeds set |retval| to the
  // function return value. If execution fails set |exception| to the exception
  // that will be thrown. Return true (1) if execution was handled.
  //
  pub execute: Option<extern "C" fn(this: *mut cef_v8handler_t,
      name: *const types::cef_string_t, object: *mut interfaces::cef_v8value_t,
      arguments_count: libc::size_t,
      arguments: *const *mut interfaces::cef_v8value_t,
      retval: *mut interfaces::cef_v8value_t,
      exception: *mut types::cef_string_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8handler_t = _cef_v8handler_t;


//
// Structure that should be implemented to handle V8 function calls. The
// functions of this structure will be called on the thread associated with the
// V8 function.
//
pub struct CefV8Handler {
  c_object: *mut cef_v8handler_t,
}

impl Clone for CefV8Handler {
  fn clone(&self) -> CefV8Handler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8Handler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8Handler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8Handler {
  pub unsafe fn from_c_object(c_object: *mut cef_v8handler_t) -> CefV8Handler {
    CefV8Handler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8handler_t) -> CefV8Handler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8Handler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8handler_t {
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
  // Handle execution of the function identified by |name|. |object| is the
  // receiver ('this' object) of the function. |arguments| is the list of
  // arguments passed to the function. If execution succeeds set |retval| to the
  // function return value. If execution fails set |exception| to the exception
  // that will be thrown. Return true (1) if execution was handled.
  //
  pub fn execute(&self, name: &[u16], object: interfaces::CefV8Value,
      arguments_count: libc::size_t, arguments: *const interfaces::CefV8Value,
      retval: interfaces::CefV8Value,
      exception: *mut types::cef_string_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).execute.unwrap())(
          self.c_object,
          CefWrap::to_c(name),
          CefWrap::to_c(object),
          CefWrap::to_c(arguments_count),
          CefWrap::to_c(arguments),
          CefWrap::to_c(retval),
          CefWrap::to_c(exception)))
    }
  }
}

impl CefWrap<*mut cef_v8handler_t> for CefV8Handler {
  fn to_c(rust_object: CefV8Handler) -> *mut cef_v8handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8handler_t) -> CefV8Handler {
    CefV8Handler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8handler_t> for Option<CefV8Handler> {
  fn to_c(rust_object: Option<CefV8Handler>) -> *mut cef_v8handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8handler_t) -> Option<CefV8Handler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8Handler::from_c_object_addref(c_object))
    }
  }
}


//
// Structure that should be implemented to handle V8 accessor calls. Accessor
// identifiers are registered by calling cef_v8value_t::set_value_byaccessor().
// The functions of this structure will be called on the thread associated with
// the V8 accessor.
//
#[repr(C)]
pub struct _cef_v8accessor_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Handle retrieval the accessor value identified by |name|. |object| is the
  // receiver ('this' object) of the accessor. If retrieval succeeds set
  // |retval| to the return value. If retrieval fails set |exception| to the
  // exception that will be thrown. Return true (1) if accessor retrieval was
  // handled.
  //
  pub get: Option<extern "C" fn(this: *mut cef_v8accessor_t,
      name: *const types::cef_string_t, object: *mut interfaces::cef_v8value_t,
      retval: *mut interfaces::cef_v8value_t,
      exception: *mut types::cef_string_t) -> libc::c_int>,

  //
  // Handle assignment of the accessor value identified by |name|. |object| is
  // the receiver ('this' object) of the accessor. |value| is the new value
  // being assigned to the accessor. If assignment fails set |exception| to the
  // exception that will be thrown. Return true (1) if accessor assignment was
  // handled.
  //
  pub set: Option<extern "C" fn(this: *mut cef_v8accessor_t,
      name: *const types::cef_string_t, object: *mut interfaces::cef_v8value_t,
      value: *mut interfaces::cef_v8value_t,
      exception: *mut types::cef_string_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8accessor_t = _cef_v8accessor_t;


//
// Structure that should be implemented to handle V8 accessor calls. Accessor
// identifiers are registered by calling cef_v8value_t::set_value_byaccessor().
// The functions of this structure will be called on the thread associated with
// the V8 accessor.
//
pub struct CefV8Accessor {
  c_object: *mut cef_v8accessor_t,
}

impl Clone for CefV8Accessor {
  fn clone(&self) -> CefV8Accessor{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8Accessor {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8Accessor {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8Accessor {
  pub unsafe fn from_c_object(c_object: *mut cef_v8accessor_t) -> CefV8Accessor {
    CefV8Accessor {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8accessor_t) -> CefV8Accessor {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8Accessor {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8accessor_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8accessor_t {
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
  // Handle retrieval the accessor value identified by |name|. |object| is the
  // receiver ('this' object) of the accessor. If retrieval succeeds set
  // |retval| to the return value. If retrieval fails set |exception| to the
  // exception that will be thrown. Return true (1) if accessor retrieval was
  // handled.
  //
  pub fn get(&self, name: &[u16], object: interfaces::CefV8Value,
      retval: interfaces::CefV8Value,
      exception: *mut types::cef_string_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get.unwrap())(
          self.c_object,
          CefWrap::to_c(name),
          CefWrap::to_c(object),
          CefWrap::to_c(retval),
          CefWrap::to_c(exception)))
    }
  }

  //
  // Handle assignment of the accessor value identified by |name|. |object| is
  // the receiver ('this' object) of the accessor. |value| is the new value
  // being assigned to the accessor. If assignment fails set |exception| to the
  // exception that will be thrown. Return true (1) if accessor assignment was
  // handled.
  //
  pub fn set(&self, name: &[u16], object: interfaces::CefV8Value,
      value: interfaces::CefV8Value,
      exception: *mut types::cef_string_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set.unwrap())(
          self.c_object,
          CefWrap::to_c(name),
          CefWrap::to_c(object),
          CefWrap::to_c(value),
          CefWrap::to_c(exception)))
    }
  }
}

impl CefWrap<*mut cef_v8accessor_t> for CefV8Accessor {
  fn to_c(rust_object: CefV8Accessor) -> *mut cef_v8accessor_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8accessor_t) -> CefV8Accessor {
    CefV8Accessor::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8accessor_t> for Option<CefV8Accessor> {
  fn to_c(rust_object: Option<CefV8Accessor>) -> *mut cef_v8accessor_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8accessor_t) -> Option<CefV8Accessor> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8Accessor::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a V8 exception. The functions of this structure may be
// called on any render process thread.
//
#[repr(C)]
pub struct _cef_v8exception_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the exception message.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_message: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> types::cef_string_userfree_t>,

  //
  // Returns the line of source code that the exception occurred within.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_source_line: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> types::cef_string_userfree_t>,

  //
  // Returns the resource name for the script from where the function causing
  // the error originates.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_script_resource_name: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> types::cef_string_userfree_t>,

  //
  // Returns the 1-based number of the line where the error occurred or 0 if the
  // line number is unknown.
  //
  pub get_line_number: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> libc::c_int>,

  //
  // Returns the index within the script of the first character where the error
  // occurred.
  //
  pub get_start_position: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> libc::c_int>,

  //
  // Returns the index within the script of the last character where the error
  // occurred.
  //
  pub get_end_position: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> libc::c_int>,

  //
  // Returns the index within the line of the first character where the error
  // occurred.
  //
  pub get_start_column: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> libc::c_int>,

  //
  // Returns the index within the line of the last character where the error
  // occurred.
  //
  pub get_end_column: Option<extern "C" fn(
      this: *mut cef_v8exception_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8exception_t = _cef_v8exception_t;


//
// Structure representing a V8 exception. The functions of this structure may be
// called on any render process thread.
//
pub struct CefV8Exception {
  c_object: *mut cef_v8exception_t,
}

impl Clone for CefV8Exception {
  fn clone(&self) -> CefV8Exception{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8Exception {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8Exception {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8Exception {
  pub unsafe fn from_c_object(c_object: *mut cef_v8exception_t) -> CefV8Exception {
    CefV8Exception {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8exception_t) -> CefV8Exception {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8Exception {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8exception_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8exception_t {
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
  // Returns the exception message.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_message(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_message.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the line of source code that the exception occurred within.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_source_line(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_source_line.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the resource name for the script from where the function causing
  // the error originates.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_script_resource_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_script_resource_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the 1-based number of the line where the error occurred or 0 if the
  // line number is unknown.
  //
  pub fn get_line_number(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_line_number.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the index within the script of the first character where the error
  // occurred.
  //
  pub fn get_start_position(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_start_position.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the index within the script of the last character where the error
  // occurred.
  //
  pub fn get_end_position(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_end_position.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the index within the line of the first character where the error
  // occurred.
  //
  pub fn get_start_column(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_start_column.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the index within the line of the last character where the error
  // occurred.
  //
  pub fn get_end_column(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_end_column.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_v8exception_t> for CefV8Exception {
  fn to_c(rust_object: CefV8Exception) -> *mut cef_v8exception_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8exception_t) -> CefV8Exception {
    CefV8Exception::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8exception_t> for Option<CefV8Exception> {
  fn to_c(rust_object: Option<CefV8Exception>) -> *mut cef_v8exception_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8exception_t) -> Option<CefV8Exception> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8Exception::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a V8 value handle. V8 handles can only be accessed
// from the thread on which they are created. Valid threads for creating a V8
// handle include the render process main thread (TID_RENDERER) and WebWorker
// threads. A task runner for posting tasks on the associated thread can be
// retrieved via the cef_v8context_t::get_task_runner() function.
//
#[repr(C)]
pub struct _cef_v8value_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
  //
  pub is_valid: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is undefined.
  //
  pub is_undefined: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is null.
  //
  pub is_null: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is bool.
  //
  pub is_bool: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is int.
  //
  pub is_int: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is unsigned int.
  //
  pub is_uint: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is double.
  //
  pub is_double: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is Date.
  //
  pub is_date: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is string.
  //
  pub is_string: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is object.
  //
  pub is_object: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is array.
  //
  pub is_array: Option<extern "C" fn(this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // True if the value type is function.
  //
  pub is_function: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub is_same: Option<extern "C" fn(this: *mut cef_v8value_t,
      that: *mut interfaces::cef_v8value_t) -> libc::c_int>,

  //
  // Return a bool value.  The underlying data will be converted to if
  // necessary.
  //
  pub get_bool_value: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Return an int value.  The underlying data will be converted to if
  // necessary.
  //
  pub get_int_value: Option<extern "C" fn(this: *mut cef_v8value_t) -> i32>,

  //
  // Return an unsigned int value.  The underlying data will be converted to if
  // necessary.
  //
  pub get_uint_value: Option<extern "C" fn(this: *mut cef_v8value_t) -> u32>,

  //
  // Return a double value.  The underlying data will be converted to if
  // necessary.
  //
  pub get_double_value: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_double>,

  //
  // Return a Date value.  The underlying data will be converted to if
  // necessary.
  //
  pub get_date_value: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> types::cef_time_t>,

  //
  // Return a string value.  The underlying data will be converted to if
  // necessary.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_string_value: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> types::cef_string_userfree_t>,


  // OBJECT METHODS - These functions are only available on objects. Arrays and
  // functions are also objects. String- and integer-based keys can be used
  // interchangably with the framework converting between them as necessary.

  //
  // Returns true (1) if this is a user created object.
  //
  pub is_user_created: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Returns true (1) if the last function call resulted in an exception. This
  // attribute exists only in the scope of the current CEF value object.
  //
  pub has_exception: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Returns the exception resulting from the last function call. This attribute
  // exists only in the scope of the current CEF value object.
  //
  pub get_exception: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> *mut interfaces::cef_v8exception_t>,

  //
  // Clears the last exception and returns true (1) on success.
  //
  pub clear_exception: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Returns true (1) if this object will re-throw future exceptions. This
  // attribute exists only in the scope of the current CEF value object.
  //
  pub will_rethrow_exceptions: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Set whether this object will re-throw future exceptions. By default
  // exceptions are not re-thrown. If a exception is re-thrown the current
  // context should not be accessed again until after the exception has been
  // caught and not re-thrown. Returns true (1) on success. This attribute
  // exists only in the scope of the current CEF value object.
  //
  pub set_rethrow_exceptions: Option<extern "C" fn(this: *mut cef_v8value_t,
      rethrow: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the object has a value with the specified identifier.
  //
  pub has_value_bykey: Option<extern "C" fn(this: *mut cef_v8value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Returns true (1) if the object has a value with the specified identifier.
  //
  pub has_value_byindex: Option<extern "C" fn(this: *mut cef_v8value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Deletes the value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only and don't-delete values this function
  // will return true (1) even though deletion failed.
  //
  pub delete_value_bykey: Option<extern "C" fn(this: *mut cef_v8value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Deletes the value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly, deletion
  // fails or an exception is thrown. For read-only and don't-delete values this
  // function will return true (1) even though deletion failed.
  //
  pub delete_value_byindex: Option<extern "C" fn(this: *mut cef_v8value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Returns the value with the specified identifier on success. Returns NULL if
  // this function is called incorrectly or an exception is thrown.
  //
  pub get_value_bykey: Option<extern "C" fn(this: *mut cef_v8value_t,
      key: *const types::cef_string_t) -> *mut interfaces::cef_v8value_t>,

  //
  // Returns the value with the specified identifier on success. Returns NULL if
  // this function is called incorrectly or an exception is thrown.
  //
  pub get_value_byindex: Option<extern "C" fn(this: *mut cef_v8value_t,
      index: libc::c_int) -> *mut interfaces::cef_v8value_t>,

  //
  // Associates a value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only values this function will return true
  // (1) even though assignment failed.
  //
  pub set_value_bykey: Option<extern "C" fn(this: *mut cef_v8value_t,
      key: *const types::cef_string_t, value: *mut interfaces::cef_v8value_t,
      attribute: types::cef_v8_propertyattribute_t) -> libc::c_int>,

  //
  // Associates a value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only values this function will return true
  // (1) even though assignment failed.
  //
  pub set_value_byindex: Option<extern "C" fn(this: *mut cef_v8value_t,
      index: libc::c_int,
      value: *mut interfaces::cef_v8value_t) -> libc::c_int>,

  //
  // Registers an identifier and returns true (1) on success. Access to the
  // identifier will be forwarded to the cef_v8accessor_t instance passed to
  // cef_v8value_t::cef_v8value_create_object(). Returns false (0) if this
  // function is called incorrectly or an exception is thrown. For read-only
  // values this function will return true (1) even though assignment failed.
  //
  pub set_value_byaccessor: Option<extern "C" fn(this: *mut cef_v8value_t,
      key: *const types::cef_string_t, settings: types::cef_v8_accesscontrol_t,
      attribute: types::cef_v8_propertyattribute_t) -> libc::c_int>,

  //
  // Read the keys for the object's values into the specified vector. Integer-
  // based keys will also be returned as strings.
  //
  pub get_keys: Option<extern "C" fn(this: *mut cef_v8value_t,
      keys: types::cef_string_list_t) -> libc::c_int>,

  //
  // Sets the user data for this object and returns true (1) on success. Returns
  // false (0) if this function is called incorrectly. This function can only be
  // called on user created objects.
  //
  pub set_user_data: Option<extern "C" fn(this: *mut cef_v8value_t,
      user_data: *mut interfaces::cef_base_t) -> libc::c_int>,

  //
  // Returns the user data, if any, assigned to this object.
  //
  pub get_user_data: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> *mut interfaces::cef_base_t>,

  //
  // Returns the amount of externally allocated memory registered for the
  // object.
  //
  pub get_externally_allocated_memory: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,

  //
  // Adjusts the amount of registered external memory for the object. Used to
  // give V8 an indication of the amount of externally allocated memory that is
  // kept alive by JavaScript objects. V8 uses this information to decide when
  // to perform global garbage collection. Each cef_v8value_t tracks the amount
  // of external memory associated with it and automatically decreases the
  // global total by the appropriate amount on its destruction.
  // |change_in_bytes| specifies the number of bytes to adjust by. This function
  // returns the number of bytes associated with the object after the
  // adjustment. This function can only be called on user created objects.
  //
  pub adjust_externally_allocated_memory: Option<extern "C" fn(
      this: *mut cef_v8value_t, change_in_bytes: libc::c_int) -> libc::c_int>,


  // ARRAY METHODS - These functions are only available on arrays.

  //
  // Returns the number of elements in the array.
  //
  pub get_array_length: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> libc::c_int>,


  // FUNCTION METHODS - These functions are only available on functions.

  //
  // Returns the function name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_function_name: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> types::cef_string_userfree_t>,

  //
  // Returns the function handler or NULL if not a CEF-created function.
  //
  pub get_function_handler: Option<extern "C" fn(
      this: *mut cef_v8value_t) -> *mut interfaces::cef_v8handler_t>,

  //
  // Execute the function using the current V8 context. This function should
  // only be called from within the scope of a cef_v8handler_t or
  // cef_v8accessor_t callback, or in combination with calling enter() and
  // exit() on a stored cef_v8context_t reference. |object| is the receiver
  // ('this' object) of the function. If |object| is NULL the current context's
  // global object will be used. |arguments| is the list of arguments that will
  // be passed to the function. Returns the function return value on success.
  // Returns NULL if this function is called incorrectly or an exception is
  // thrown.
  //
  pub execute_function: Option<extern "C" fn(this: *mut cef_v8value_t,
      object: *mut interfaces::cef_v8value_t, arguments_count: libc::size_t,
      arguments: *const *mut interfaces::cef_v8value_t) -> *mut interfaces::cef_v8value_t>,

  //
  // Execute the function using the specified V8 context. |object| is the
  // receiver ('this' object) of the function. If |object| is NULL the specified
  // context's global object will be used. |arguments| is the list of arguments
  // that will be passed to the function. Returns the function return value on
  // success. Returns NULL if this function is called incorrectly or an
  // exception is thrown.
  //
  pub execute_function_with_context: Option<extern "C" fn(
      this: *mut cef_v8value_t, context: *mut interfaces::cef_v8context_t,
      object: *mut interfaces::cef_v8value_t, arguments_count: libc::size_t,
      arguments: *const *mut interfaces::cef_v8value_t) -> *mut interfaces::cef_v8value_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8value_t = _cef_v8value_t;


//
// Structure representing a V8 value handle. V8 handles can only be accessed
// from the thread on which they are created. Valid threads for creating a V8
// handle include the render process main thread (TID_RENDERER) and WebWorker
// threads. A task runner for posting tasks on the associated thread can be
// retrieved via the cef_v8context_t::get_task_runner() function.
//
pub struct CefV8Value {
  c_object: *mut cef_v8value_t,
}

impl Clone for CefV8Value {
  fn clone(&self) -> CefV8Value{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8Value {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8Value {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8Value {
  pub unsafe fn from_c_object(c_object: *mut cef_v8value_t) -> CefV8Value {
    CefV8Value {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8value_t) -> CefV8Value {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8Value {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8value_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8value_t {
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
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
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
  // True if the value type is undefined.
  //
  pub fn is_undefined(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_undefined.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is null.
  //
  pub fn is_null(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_null.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is bool.
  //
  pub fn is_bool(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_bool.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is int.
  //
  pub fn is_int(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_int.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is unsigned int.
  //
  pub fn is_uint(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_uint.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is double.
  //
  pub fn is_double(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_double.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is Date.
  //
  pub fn is_date(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_date.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is string.
  //
  pub fn is_string(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_string.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is object.
  //
  pub fn is_object(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_object.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is array.
  //
  pub fn is_array(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_array.unwrap())(
          self.c_object))
    }
  }

  //
  // True if the value type is function.
  //
  pub fn is_function(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_function.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this object is pointing to the same handle as |that|
  // object.
  //
  pub fn is_same(&self, that: interfaces::CefV8Value) -> libc::c_int {
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
  // Return a bool value.  The underlying data will be converted to if
  // necessary.
  //
  pub fn get_bool_value(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_bool_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Return an int value.  The underlying data will be converted to if
  // necessary.
  //
  pub fn get_int_value(&self) -> i32 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_int_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Return an unsigned int value.  The underlying data will be converted to if
  // necessary.
  //
  pub fn get_uint_value(&self) -> u32 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_uint_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Return a double value.  The underlying data will be converted to if
  // necessary.
  //
  pub fn get_double_value(&self) -> libc::c_double {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_double_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Return a Date value.  The underlying data will be converted to if
  // necessary.
  //
  pub fn get_date_value(&self) -> types::cef_time_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_date_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Return a string value.  The underlying data will be converted to if
  // necessary.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_string_value(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_string_value.unwrap())(
          self.c_object))
    }
  }


  // OBJECT METHODS - These functions are only available on objects. Arrays and
  // functions are also objects. String- and integer-based keys can be used
  // interchangably with the framework converting between them as necessary.

  //
  // Returns true (1) if this is a user created object.
  //
  pub fn is_user_created(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_user_created.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the last function call resulted in an exception. This
  // attribute exists only in the scope of the current CEF value object.
  //
  pub fn has_exception(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_exception.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the exception resulting from the last function call. This attribute
  // exists only in the scope of the current CEF value object.
  //
  pub fn get_exception(&self) -> interfaces::CefV8Exception {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_exception.unwrap())(
          self.c_object))
    }
  }

  //
  // Clears the last exception and returns true (1) on success.
  //
  pub fn clear_exception(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).clear_exception.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this object will re-throw future exceptions. This
  // attribute exists only in the scope of the current CEF value object.
  //
  pub fn will_rethrow_exceptions(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).will_rethrow_exceptions.unwrap())(
          self.c_object))
    }
  }

  //
  // Set whether this object will re-throw future exceptions. By default
  // exceptions are not re-thrown. If a exception is re-thrown the current
  // context should not be accessed again until after the exception has been
  // caught and not re-thrown. Returns true (1) on success. This attribute
  // exists only in the scope of the current CEF value object.
  //
  pub fn set_rethrow_exceptions(&self, rethrow: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_rethrow_exceptions.unwrap())(
          self.c_object,
          CefWrap::to_c(rethrow)))
    }
  }

  //
  // Returns true (1) if the object has a value with the specified identifier.
  //
  pub fn has_value_bykey(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_value_bykey.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns true (1) if the object has a value with the specified identifier.
  //
  pub fn has_value_byindex(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_value_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Deletes the value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only and don't-delete values this function
  // will return true (1) even though deletion failed.
  //
  pub fn delete_value_bykey(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).delete_value_bykey.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Deletes the value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly, deletion
  // fails or an exception is thrown. For read-only and don't-delete values this
  // function will return true (1) even though deletion failed.
  //
  pub fn delete_value_byindex(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).delete_value_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value with the specified identifier on success. Returns NULL if
  // this function is called incorrectly or an exception is thrown.
  //
  pub fn get_value_bykey(&self, key: &[u16]) -> interfaces::CefV8Value {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_value_bykey.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value with the specified identifier on success. Returns NULL if
  // this function is called incorrectly or an exception is thrown.
  //
  pub fn get_value_byindex(&self,
      index: libc::c_int) -> interfaces::CefV8Value {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_value_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Associates a value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only values this function will return true
  // (1) even though assignment failed.
  //
  pub fn set_value_bykey(&self, key: &[u16], value: interfaces::CefV8Value,
      attribute: types::cef_v8_propertyattribute_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_value_bykey.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value),
          CefWrap::to_c(attribute)))
    }
  }

  //
  // Associates a value with the specified identifier and returns true (1) on
  // success. Returns false (0) if this function is called incorrectly or an
  // exception is thrown. For read-only values this function will return true
  // (1) even though assignment failed.
  //
  pub fn set_value_byindex(&self, index: libc::c_int,
      value: interfaces::CefV8Value) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_value_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Registers an identifier and returns true (1) on success. Access to the
  // identifier will be forwarded to the cef_v8accessor_t instance passed to
  // cef_v8value_t::cef_v8value_create_object(). Returns false (0) if this
  // function is called incorrectly or an exception is thrown. For read-only
  // values this function will return true (1) even though assignment failed.
  //
  pub fn set_value_byaccessor(&self, key: &[u16],
      settings: types::cef_v8_accesscontrol_t,
      attribute: types::cef_v8_propertyattribute_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_value_byaccessor.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(settings),
          CefWrap::to_c(attribute)))
    }
  }

  //
  // Read the keys for the object's values into the specified vector. Integer-
  // based keys will also be returned as strings.
  //
  pub fn get_keys(&self, keys: Vec<String>) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_keys.unwrap())(
          self.c_object,
          CefWrap::to_c(keys)))
    }
  }

  //
  // Sets the user data for this object and returns true (1) on success. Returns
  // false (0) if this function is called incorrectly. This function can only be
  // called on user created objects.
  //
  pub fn set_user_data(&self, user_data: interfaces::CefBase) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_user_data.unwrap())(
          self.c_object,
          CefWrap::to_c(user_data)))
    }
  }

  //
  // Returns the user data, if any, assigned to this object.
  //
  pub fn get_user_data(&self) -> interfaces::CefBase {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_user_data.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the amount of externally allocated memory registered for the
  // object.
  //
  pub fn get_externally_allocated_memory(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_externally_allocated_memory.unwrap())(
          self.c_object))
    }
  }

  //
  // Adjusts the amount of registered external memory for the object. Used to
  // give V8 an indication of the amount of externally allocated memory that is
  // kept alive by JavaScript objects. V8 uses this information to decide when
  // to perform global garbage collection. Each cef_v8value_t tracks the amount
  // of external memory associated with it and automatically decreases the
  // global total by the appropriate amount on its destruction.
  // |change_in_bytes| specifies the number of bytes to adjust by. This function
  // returns the number of bytes associated with the object after the
  // adjustment. This function can only be called on user created objects.
  //
  pub fn adjust_externally_allocated_memory(&self,
      change_in_bytes: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).adjust_externally_allocated_memory.unwrap())(
          self.c_object,
          CefWrap::to_c(change_in_bytes)))
    }
  }


  // ARRAY METHODS - These functions are only available on arrays.

  //
  // Returns the number of elements in the array.
  //
  pub fn get_array_length(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_array_length.unwrap())(
          self.c_object))
    }
  }


  // FUNCTION METHODS - These functions are only available on functions.

  //
  // Returns the function name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_function_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_function_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the function handler or NULL if not a CEF-created function.
  //
  pub fn get_function_handler(&self) -> interfaces::CefV8Handler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_function_handler.unwrap())(
          self.c_object))
    }
  }

  //
  // Execute the function using the current V8 context. This function should
  // only be called from within the scope of a cef_v8handler_t or
  // cef_v8accessor_t callback, or in combination with calling enter() and
  // exit() on a stored cef_v8context_t reference. |object| is the receiver
  // ('this' object) of the function. If |object| is NULL the current context's
  // global object will be used. |arguments| is the list of arguments that will
  // be passed to the function. Returns the function return value on success.
  // Returns NULL if this function is called incorrectly or an exception is
  // thrown.
  //
  pub fn execute_function(&self, object: interfaces::CefV8Value,
      arguments_count: libc::size_t,
      arguments: *const interfaces::CefV8Value) -> interfaces::CefV8Value {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).execute_function.unwrap())(
          self.c_object,
          CefWrap::to_c(object),
          CefWrap::to_c(arguments_count),
          CefWrap::to_c(arguments)))
    }
  }

  //
  // Execute the function using the specified V8 context. |object| is the
  // receiver ('this' object) of the function. If |object| is NULL the specified
  // context's global object will be used. |arguments| is the list of arguments
  // that will be passed to the function. Returns the function return value on
  // success. Returns NULL if this function is called incorrectly or an
  // exception is thrown.
  //
  pub fn execute_function_with_context(&self, context: interfaces::CefV8Context,
      object: interfaces::CefV8Value, arguments_count: libc::size_t,
      arguments: *const interfaces::CefV8Value) -> interfaces::CefV8Value {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).execute_function_with_context.unwrap())(
          self.c_object,
          CefWrap::to_c(context),
          CefWrap::to_c(object),
          CefWrap::to_c(arguments_count),
          CefWrap::to_c(arguments)))
    }
  }

  //
  // Create a new cef_v8value_t object of type undefined.
  //
  pub fn create_undefined() -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_undefined(
))
    }
  }

  //
  // Create a new cef_v8value_t object of type null.
  //
  pub fn create_null() -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_null(
))
    }
  }

  //
  // Create a new cef_v8value_t object of type bool.
  //
  pub fn create_bool(value: libc::c_int) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_bool(
          CefWrap::to_c(value)))
    }
  }

  //
  // Create a new cef_v8value_t object of type int.
  //
  pub fn create_int(value: i32) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_int(
          CefWrap::to_c(value)))
    }
  }

  //
  // Create a new cef_v8value_t object of type unsigned int.
  //
  pub fn create_uint(value: u32) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_uint(
          CefWrap::to_c(value)))
    }
  }

  //
  // Create a new cef_v8value_t object of type double.
  //
  pub fn create_double(value: libc::c_double) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_double(
          CefWrap::to_c(value)))
    }
  }

  //
  // Create a new cef_v8value_t object of type Date. This function should only
  // be called from within the scope of a cef_v8context_tHandler,
  // cef_v8handler_t or cef_v8accessor_t callback, or in combination with
  // calling enter() and exit() on a stored cef_v8context_t reference.
  //
  pub fn create_date(date: &types::cef_time_t) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_date(
          CefWrap::to_c(date)))
    }
  }

  //
  // Create a new cef_v8value_t object of type string.
  //
  pub fn create_string(value: &[u16]) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_string(
          CefWrap::to_c(value)))
    }
  }

  //
  // Create a new cef_v8value_t object of type object with optional accessor.
  // This function should only be called from within the scope of a
  // cef_v8context_tHandler, cef_v8handler_t or cef_v8accessor_t callback, or in
  // combination with calling enter() and exit() on a stored cef_v8context_t
  // reference.
  //
  pub fn create_object(
      accessor: interfaces::CefV8Accessor) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_object(
          CefWrap::to_c(accessor)))
    }
  }

  //
  // Create a new cef_v8value_t object of type array with the specified
  // |length|. If |length| is negative the returned array will have length 0.
  // This function should only be called from within the scope of a
  // cef_v8context_tHandler, cef_v8handler_t or cef_v8accessor_t callback, or in
  // combination with calling enter() and exit() on a stored cef_v8context_t
  // reference.
  //
  pub fn create_array(length: libc::c_int) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_array(
          CefWrap::to_c(length)))
    }
  }

  //
  // Create a new cef_v8value_t object of type function. This function should
  // only be called from within the scope of a cef_v8context_tHandler,
  // cef_v8handler_t or cef_v8accessor_t callback, or in combination with
  // calling enter() and exit() on a stored cef_v8context_t reference.
  //
  pub fn create_function(name: &[u16],
      handler: interfaces::CefV8Handler) -> interfaces::CefV8Value {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8value_create_function(
          CefWrap::to_c(name),
          CefWrap::to_c(handler)))
    }
  }
}

impl CefWrap<*mut cef_v8value_t> for CefV8Value {
  fn to_c(rust_object: CefV8Value) -> *mut cef_v8value_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8value_t) -> CefV8Value {
    CefV8Value::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8value_t> for Option<CefV8Value> {
  fn to_c(rust_object: Option<CefV8Value>) -> *mut cef_v8value_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8value_t) -> Option<CefV8Value> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8Value::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a V8 stack trace handle. V8 handles can only be
// accessed from the thread on which they are created. Valid threads for
// creating a V8 handle include the render process main thread (TID_RENDERER)
// and WebWorker threads. A task runner for posting tasks on the associated
// thread can be retrieved via the cef_v8context_t::get_task_runner() function.
//
#[repr(C)]
pub struct _cef_v8stack_trace_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_v8stack_trace_t) -> libc::c_int>,

  //
  // Returns the number of stack frames.
  //
  pub get_frame_count: Option<extern "C" fn(
      this: *mut cef_v8stack_trace_t) -> libc::c_int>,

  //
  // Returns the stack frame at the specified 0-based index.
  //
  pub get_frame: Option<extern "C" fn(this: *mut cef_v8stack_trace_t,
      index: libc::c_int) -> *mut interfaces::cef_v8stack_frame_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8stack_trace_t = _cef_v8stack_trace_t;


//
// Structure representing a V8 stack trace handle. V8 handles can only be
// accessed from the thread on which they are created. Valid threads for
// creating a V8 handle include the render process main thread (TID_RENDERER)
// and WebWorker threads. A task runner for posting tasks on the associated
// thread can be retrieved via the cef_v8context_t::get_task_runner() function.
//
pub struct CefV8StackTrace {
  c_object: *mut cef_v8stack_trace_t,
}

impl Clone for CefV8StackTrace {
  fn clone(&self) -> CefV8StackTrace{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8StackTrace {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8StackTrace {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8StackTrace {
  pub unsafe fn from_c_object(c_object: *mut cef_v8stack_trace_t) -> CefV8StackTrace {
    CefV8StackTrace {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8stack_trace_t) -> CefV8StackTrace {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8StackTrace {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8stack_trace_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8stack_trace_t {
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
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
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
  // Returns the number of stack frames.
  //
  pub fn get_frame_count(&self) -> libc::c_int {
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
  // Returns the stack frame at the specified 0-based index.
  //
  pub fn get_frame(&self, index: libc::c_int) -> interfaces::CefV8StackFrame {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_frame.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the stack trace for the currently active context. |frame_limit| is
  // the maximum number of frames that will be captured.
  //
  pub fn get_current(frame_limit: libc::c_int) -> interfaces::CefV8StackTrace {
    unsafe {
      CefWrap::to_rust(
        ::v8::cef_v8stack_trace_get_current(
          CefWrap::to_c(frame_limit)))
    }
  }
}

impl CefWrap<*mut cef_v8stack_trace_t> for CefV8StackTrace {
  fn to_c(rust_object: CefV8StackTrace) -> *mut cef_v8stack_trace_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8stack_trace_t) -> CefV8StackTrace {
    CefV8StackTrace::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8stack_trace_t> for Option<CefV8StackTrace> {
  fn to_c(rust_object: Option<CefV8StackTrace>) -> *mut cef_v8stack_trace_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8stack_trace_t) -> Option<CefV8StackTrace> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8StackTrace::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a V8 stack frame handle. V8 handles can only be
// accessed from the thread on which they are created. Valid threads for
// creating a V8 handle include the render process main thread (TID_RENDERER)
// and WebWorker threads. A task runner for posting tasks on the associated
// thread can be retrieved via the cef_v8context_t::get_task_runner() function.
//
#[repr(C)]
pub struct _cef_v8stack_frame_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> libc::c_int>,

  //
  // Returns the name of the resource script that contains the function.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_script_name: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> types::cef_string_userfree_t>,

  //
  // Returns the name of the resource script that contains the function or the
  // sourceURL value if the script name is undefined and its source ends with a
  // "//@ sourceURL=..." string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_script_name_or_source_url: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> types::cef_string_userfree_t>,

  //
  // Returns the name of the function.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_function_name: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> types::cef_string_userfree_t>,

  //
  // Returns the 1-based line number for the function call or 0 if unknown.
  //
  pub get_line_number: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> libc::c_int>,

  //
  // Returns the 1-based column offset on the line for the function call or 0 if
  // unknown.
  //
  pub get_column: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> libc::c_int>,

  //
  // Returns true (1) if the function was compiled using eval().
  //
  pub is_eval: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> libc::c_int>,

  //
  // Returns true (1) if the function was called as a constructor via "new".
  //
  pub is_constructor: Option<extern "C" fn(
      this: *mut cef_v8stack_frame_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_v8stack_frame_t = _cef_v8stack_frame_t;


//
// Structure representing a V8 stack frame handle. V8 handles can only be
// accessed from the thread on which they are created. Valid threads for
// creating a V8 handle include the render process main thread (TID_RENDERER)
// and WebWorker threads. A task runner for posting tasks on the associated
// thread can be retrieved via the cef_v8context_t::get_task_runner() function.
//
pub struct CefV8StackFrame {
  c_object: *mut cef_v8stack_frame_t,
}

impl Clone for CefV8StackFrame {
  fn clone(&self) -> CefV8StackFrame{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefV8StackFrame {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefV8StackFrame {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefV8StackFrame {
  pub unsafe fn from_c_object(c_object: *mut cef_v8stack_frame_t) -> CefV8StackFrame {
    CefV8StackFrame {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_v8stack_frame_t) -> CefV8StackFrame {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefV8StackFrame {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_v8stack_frame_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_v8stack_frame_t {
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
  // Returns true (1) if the underlying handle is valid and it can be accessed
  // on the current thread. Do not call any other functions if this function
  // returns false (0).
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
  // Returns the name of the resource script that contains the function.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_script_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_script_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the name of the resource script that contains the function or the
  // sourceURL value if the script name is undefined and its source ends with a
  // "//@ sourceURL=..." string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_script_name_or_source_url(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_script_name_or_source_url.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the name of the function.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_function_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_function_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the 1-based line number for the function call or 0 if unknown.
  //
  pub fn get_line_number(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_line_number.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the 1-based column offset on the line for the function call or 0 if
  // unknown.
  //
  pub fn get_column(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_column.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the function was compiled using eval().
  //
  pub fn is_eval(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_eval.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the function was called as a constructor via "new".
  //
  pub fn is_constructor(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_constructor.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_v8stack_frame_t> for CefV8StackFrame {
  fn to_c(rust_object: CefV8StackFrame) -> *mut cef_v8stack_frame_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_v8stack_frame_t) -> CefV8StackFrame {
    CefV8StackFrame::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_v8stack_frame_t> for Option<CefV8StackFrame> {
  fn to_c(rust_object: Option<CefV8StackFrame>) -> *mut cef_v8stack_frame_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_v8stack_frame_t) -> Option<CefV8StackFrame> {
    if c_object.is_null() {
      None
    } else {
      Some(CefV8StackFrame::from_c_object_addref(c_object))
    }
  }
}

