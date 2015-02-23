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
// Structure used to represent a web response. The functions of this structure
// may be called on any thread.
//
#[repr(C)]
pub struct _cef_response_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is read-only.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_response_t) -> libc::c_int>,

  //
  // Get the response status code.
  //
  pub get_status: Option<extern "C" fn(
      this: *mut cef_response_t) -> libc::c_int>,

  //
  // Set the response status code.
  //
  pub set_status: Option<extern "C" fn(this: *mut cef_response_t,
      status: libc::c_int) -> ()>,

  //
  // Get the response status text.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_status_text: Option<extern "C" fn(
      this: *mut cef_response_t) -> types::cef_string_userfree_t>,

  //
  // Set the response status text.
  //
  pub set_status_text: Option<extern "C" fn(this: *mut cef_response_t,
      statusText: *const types::cef_string_t) -> ()>,

  //
  // Get the response mime type.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_mime_type: Option<extern "C" fn(
      this: *mut cef_response_t) -> types::cef_string_userfree_t>,

  //
  // Set the response mime type.
  //
  pub set_mime_type: Option<extern "C" fn(this: *mut cef_response_t,
      mimeType: *const types::cef_string_t) -> ()>,

  //
  // Get the value for the specified response header field.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_header: Option<extern "C" fn(this: *mut cef_response_t,
      name: *const types::cef_string_t) -> types::cef_string_userfree_t>,

  //
  // Get all response header fields.
  //
  pub get_header_map: Option<extern "C" fn(this: *mut cef_response_t,
      headerMap: types::cef_string_multimap_t) -> ()>,

  //
  // Set all response header fields.
  //
  pub set_header_map: Option<extern "C" fn(this: *mut cef_response_t,
      headerMap: types::cef_string_multimap_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_response_t = _cef_response_t;


//
// Structure used to represent a web response. The functions of this structure
// may be called on any thread.
//
pub struct CefResponse {
  c_object: *mut cef_response_t,
}

impl Clone for CefResponse {
  fn clone(&self) -> CefResponse{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefResponse {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefResponse {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefResponse {
  pub unsafe fn from_c_object(c_object: *mut cef_response_t) -> CefResponse {
    CefResponse {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_response_t) -> CefResponse {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefResponse {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_response_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_response_t {
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
  // Returns true (1) if this object is read-only.
  //
  pub fn is_read_only(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_read_only.unwrap())(
          self.c_object))
    }
  }

  //
  // Get the response status code.
  //
  pub fn get_status(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_status.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the response status code.
  //
  pub fn set_status(&self, status: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_status.unwrap())(
          self.c_object,
          CefWrap::to_c(status)))
    }
  }

  //
  // Get the response status text.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_status_text(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_status_text.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the response status text.
  //
  pub fn set_status_text(&self, statusText: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_status_text.unwrap())(
          self.c_object,
          CefWrap::to_c(statusText)))
    }
  }

  //
  // Get the response mime type.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_mime_type(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_mime_type.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the response mime type.
  //
  pub fn set_mime_type(&self, mimeType: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_mime_type.unwrap())(
          self.c_object,
          CefWrap::to_c(mimeType)))
    }
  }

  //
  // Get the value for the specified response header field.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_header(&self, name: &[u16]) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_header.unwrap())(
          self.c_object,
          CefWrap::to_c(name)))
    }
  }

  //
  // Get all response header fields.
  //
  pub fn get_header_map(&self, headerMap: HashMap<String,Vec<String>>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_header_map.unwrap())(
          self.c_object,
          CefWrap::to_c(headerMap)))
    }
  }

  //
  // Set all response header fields.
  //
  pub fn set_header_map(&self, headerMap: HashMap<String,Vec<String>>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_header_map.unwrap())(
          self.c_object,
          CefWrap::to_c(headerMap)))
    }
  }

  //
  // Create a new cef_response_t object.
  //
  pub fn create() -> interfaces::CefResponse {
    unsafe {
      CefWrap::to_rust(
        ::response::cef_response_create(
))
    }
  }
}

impl CefWrap<*mut cef_response_t> for CefResponse {
  fn to_c(rust_object: CefResponse) -> *mut cef_response_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_response_t) -> CefResponse {
    CefResponse::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_response_t> for Option<CefResponse> {
  fn to_c(rust_object: Option<CefResponse>) -> *mut cef_response_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_response_t) -> Option<CefResponse> {
    if c_object.is_null() {
      None
    } else {
      Some(CefResponse::from_c_object_addref(c_object))
    }
  }
}

