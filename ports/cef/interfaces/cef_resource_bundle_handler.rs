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
// Structure used to implement a custom resource bundle structure. The functions
// of this structure may be called on multiple threads.
//
#[repr(C)]
pub struct _cef_resource_bundle_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called to retrieve a localized translation for the string specified by
  // |message_id|. To provide the translation set |string| to the translation
  // string and return true (1). To use the default translation return false
  // (0). Supported message IDs are listed in cef_pack_strings.h.
  //
  pub get_localized_string: Option<extern "C" fn(
      this: *mut cef_resource_bundle_handler_t, message_id: libc::c_int,
      string: *mut types::cef_string_t) -> libc::c_int>,

  //
  // Called to retrieve data for the resource specified by |resource_id|. To
  // provide the resource data set |data| and |data_size| to the data pointer
  // and size respectively and return true (1). To use the default resource data
  // return false (0). The resource data will not be copied and must remain
  // resident in memory. Supported resource IDs are listed in
  // cef_pack_resources.h.
  //
  pub get_data_resource: Option<extern "C" fn(
      this: *mut cef_resource_bundle_handler_t, resource_id: libc::c_int,
      data: *mut *mut libc::c_void,
      data_size: *mut libc::size_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_resource_bundle_handler_t = _cef_resource_bundle_handler_t;


//
// Structure used to implement a custom resource bundle structure. The functions
// of this structure may be called on multiple threads.
//
pub struct CefResourceBundleHandler {
  c_object: *mut cef_resource_bundle_handler_t,
}

impl Clone for CefResourceBundleHandler {
  fn clone(&self) -> CefResourceBundleHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefResourceBundleHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefResourceBundleHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefResourceBundleHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_resource_bundle_handler_t) -> CefResourceBundleHandler {
    CefResourceBundleHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_resource_bundle_handler_t) -> CefResourceBundleHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefResourceBundleHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_resource_bundle_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_resource_bundle_handler_t {
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
  // Called to retrieve a localized translation for the string specified by
  // |message_id|. To provide the translation set |string| to the translation
  // string and return true (1). To use the default translation return false
  // (0). Supported message IDs are listed in cef_pack_strings.h.
  //
  pub fn get_localized_string(&self, message_id: libc::c_int,
      string: *mut types::cef_string_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_localized_string.unwrap())(
          self.c_object,
          CefWrap::to_c(message_id),
          CefWrap::to_c(string)))
    }
  }

  //
  // Called to retrieve data for the resource specified by |resource_id|. To
  // provide the resource data set |data| and |data_size| to the data pointer
  // and size respectively and return true (1). To use the default resource data
  // return false (0). The resource data will not be copied and must remain
  // resident in memory. Supported resource IDs are listed in
  // cef_pack_resources.h.
  //
  pub fn get_data_resource(&self, resource_id: libc::c_int,
      data: &mut *mut libc::c_void,
      data_size: &mut libc::size_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_data_resource.unwrap())(
          self.c_object,
          CefWrap::to_c(resource_id),
          CefWrap::to_c(data),
          CefWrap::to_c(data_size)))
    }
  }
}

impl CefWrap<*mut cef_resource_bundle_handler_t> for CefResourceBundleHandler {
  fn to_c(rust_object: CefResourceBundleHandler) -> *mut cef_resource_bundle_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_resource_bundle_handler_t) -> CefResourceBundleHandler {
    CefResourceBundleHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_resource_bundle_handler_t> for Option<CefResourceBundleHandler> {
  fn to_c(rust_object: Option<CefResourceBundleHandler>) -> *mut cef_resource_bundle_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_resource_bundle_handler_t) -> Option<CefResourceBundleHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefResourceBundleHandler::from_c_object_addref(c_object))
    }
  }
}

