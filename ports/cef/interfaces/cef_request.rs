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
// Structure used to represent a web request. The functions of this structure
// may be called on any thread.
//
#[repr(C)]
pub struct _cef_request_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is read-only.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_request_t) -> libc::c_int>,

  //
  // Get the fully qualified URL.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_url: Option<extern "C" fn(
      this: *mut cef_request_t) -> types::cef_string_userfree_t>,

  //
  // Set the fully qualified URL.
  //
  pub set_url: Option<extern "C" fn(this: *mut cef_request_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Get the request function type. The value will default to POST if post data
  // is provided and GET otherwise.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_method: Option<extern "C" fn(
      this: *mut cef_request_t) -> types::cef_string_userfree_t>,

  //
  // Set the request function type.
  //
  pub set_method: Option<extern "C" fn(this: *mut cef_request_t,
      method: *const types::cef_string_t) -> ()>,

  //
  // Get the post data.
  //
  pub get_post_data: Option<extern "C" fn(
      this: *mut cef_request_t) -> *mut interfaces::cef_post_data_t>,

  //
  // Set the post data.
  //
  pub set_post_data: Option<extern "C" fn(this: *mut cef_request_t,
      postData: *mut interfaces::cef_post_data_t) -> ()>,

  //
  // Get the header values.
  //
  pub get_header_map: Option<extern "C" fn(this: *mut cef_request_t,
      headerMap: types::cef_string_multimap_t) -> ()>,

  //
  // Set the header values.
  //
  pub set_header_map: Option<extern "C" fn(this: *mut cef_request_t,
      headerMap: types::cef_string_multimap_t) -> ()>,

  //
  // Set all values at one time.
  //
  pub set: Option<extern "C" fn(this: *mut cef_request_t,
      url: *const types::cef_string_t, method: *const types::cef_string_t,
      postData: *mut interfaces::cef_post_data_t,
      headerMap: types::cef_string_multimap_t) -> ()>,

  //
  // Get the flags used in combination with cef_urlrequest_t. See
  // cef_urlrequest_flags_t for supported values.
  //
  pub get_flags: Option<extern "C" fn(this: *mut cef_request_t) -> libc::c_int>,

  //
  // Set the flags used in combination with cef_urlrequest_t.  See
  // cef_urlrequest_flags_t for supported values.
  //
  pub set_flags: Option<extern "C" fn(this: *mut cef_request_t,
      flags: libc::c_int) -> ()>,

  //
  // Set the URL to the first party for cookies used in combination with
  // cef_urlrequest_t.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_first_party_for_cookies: Option<extern "C" fn(
      this: *mut cef_request_t) -> types::cef_string_userfree_t>,

  //
  // Get the URL to the first party for cookies used in combination with
  // cef_urlrequest_t.
  //
  pub set_first_party_for_cookies: Option<extern "C" fn(
      this: *mut cef_request_t, url: *const types::cef_string_t) -> ()>,

  //
  // Get the resource type for this request. Only available in the browser
  // process.
  //
  pub get_resource_type: Option<extern "C" fn(
      this: *mut cef_request_t) -> types::cef_resource_type_t>,

  //
  // Get the transition type for this request. Only available in the browser
  // process and only applies to requests that represent a main frame or sub-
  // frame navigation.
  //
  pub get_transition_type: Option<extern "C" fn(
      this: *mut cef_request_t) -> types::cef_transition_type_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_request_t = _cef_request_t;


//
// Structure used to represent a web request. The functions of this structure
// may be called on any thread.
//
pub struct CefRequest {
  c_object: *mut cef_request_t,
}

impl Clone for CefRequest {
  fn clone(&self) -> CefRequest{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefRequest {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefRequest {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefRequest {
  pub unsafe fn from_c_object(c_object: *mut cef_request_t) -> CefRequest {
    CefRequest {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_request_t) -> CefRequest {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefRequest {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_request_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_request_t {
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
  // Get the fully qualified URL.
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
  // Set the fully qualified URL.
  //
  pub fn set_url(&self, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_url.unwrap())(
          self.c_object,
          CefWrap::to_c(url)))
    }
  }

  //
  // Get the request function type. The value will default to POST if post data
  // is provided and GET otherwise.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_method(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_method.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the request function type.
  //
  pub fn set_method(&self, method: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_method.unwrap())(
          self.c_object,
          CefWrap::to_c(method)))
    }
  }

  //
  // Get the post data.
  //
  pub fn get_post_data(&self) -> interfaces::CefPostData {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_post_data.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the post data.
  //
  pub fn set_post_data(&self, postData: interfaces::CefPostData) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_post_data.unwrap())(
          self.c_object,
          CefWrap::to_c(postData)))
    }
  }

  //
  // Get the header values.
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
  // Set the header values.
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
  // Set all values at one time.
  //
  pub fn set(&self, url: &[u16], method: &[u16],
      postData: interfaces::CefPostData, headerMap: HashMap<String,
      Vec<String>>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(method),
          CefWrap::to_c(postData),
          CefWrap::to_c(headerMap)))
    }
  }

  //
  // Get the flags used in combination with cef_urlrequest_t. See
  // cef_urlrequest_flags_t for supported values.
  //
  pub fn get_flags(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_flags.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the flags used in combination with cef_urlrequest_t.  See
  // cef_urlrequest_flags_t for supported values.
  //
  pub fn set_flags(&self, flags: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_flags.unwrap())(
          self.c_object,
          CefWrap::to_c(flags)))
    }
  }

  //
  // Set the URL to the first party for cookies used in combination with
  // cef_urlrequest_t.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_first_party_for_cookies(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_first_party_for_cookies.unwrap())(
          self.c_object))
    }
  }

  //
  // Get the URL to the first party for cookies used in combination with
  // cef_urlrequest_t.
  //
  pub fn set_first_party_for_cookies(&self, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_first_party_for_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(url)))
    }
  }

  //
  // Get the resource type for this request. Only available in the browser
  // process.
  //
  pub fn get_resource_type(&self) -> types::cef_resource_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_resource_type.unwrap())(
          self.c_object))
    }
  }

  //
  // Get the transition type for this request. Only available in the browser
  // process and only applies to requests that represent a main frame or sub-
  // frame navigation.
  //
  pub fn get_transition_type(&self) -> types::cef_transition_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_transition_type.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_request_t object.
  //
  pub fn create() -> interfaces::CefRequest {
    unsafe {
      CefWrap::to_rust(
        ::request::cef_request_create(
))
    }
  }
}

impl CefWrap<*mut cef_request_t> for CefRequest {
  fn to_c(rust_object: CefRequest) -> *mut cef_request_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_request_t) -> CefRequest {
    CefRequest::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_request_t> for Option<CefRequest> {
  fn to_c(rust_object: Option<CefRequest>) -> *mut cef_request_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_request_t) -> Option<CefRequest> {
    if c_object.is_null() {
      None
    } else {
      Some(CefRequest::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to represent post data for a web request. The functions of
// this structure may be called on any thread.
//
#[repr(C)]
pub struct _cef_post_data_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is read-only.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_post_data_t) -> libc::c_int>,

  //
  // Returns the number of existing post data elements.
  //
  pub get_element_count: Option<extern "C" fn(
      this: *mut cef_post_data_t) -> libc::size_t>,

  //
  // Retrieve the post data elements.
  //
  pub get_elements: Option<extern "C" fn(this: *mut cef_post_data_t,
      elements_count: *mut libc::size_t,
      elements: *mut *mut interfaces::cef_post_data_element_t) -> ()>,

  //
  // Remove the specified post data element.  Returns true (1) if the removal
  // succeeds.
  //
  pub remove_element: Option<extern "C" fn(this: *mut cef_post_data_t,
      element: *mut interfaces::cef_post_data_element_t) -> libc::c_int>,

  //
  // Add the specified post data element.  Returns true (1) if the add succeeds.
  //
  pub add_element: Option<extern "C" fn(this: *mut cef_post_data_t,
      element: *mut interfaces::cef_post_data_element_t) -> libc::c_int>,

  //
  // Remove all existing post data elements.
  //
  pub remove_elements: Option<extern "C" fn(this: *mut cef_post_data_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_post_data_t = _cef_post_data_t;


//
// Structure used to represent post data for a web request. The functions of
// this structure may be called on any thread.
//
pub struct CefPostData {
  c_object: *mut cef_post_data_t,
}

impl Clone for CefPostData {
  fn clone(&self) -> CefPostData{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPostData {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPostData {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPostData {
  pub unsafe fn from_c_object(c_object: *mut cef_post_data_t) -> CefPostData {
    CefPostData {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_post_data_t) -> CefPostData {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPostData {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_post_data_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_post_data_t {
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
  // Returns the number of existing post data elements.
  //
  pub fn get_element_count(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_element_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Retrieve the post data elements.
  //
  pub fn get_elements(&self, elements_count: *mut libc::size_t,
      elements: *mut interfaces::CefPostDataElement) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_elements.unwrap())(
          self.c_object,
          CefWrap::to_c(elements_count),
          CefWrap::to_c(elements)))
    }
  }

  //
  // Remove the specified post data element.  Returns true (1) if the removal
  // succeeds.
  //
  pub fn remove_element(&self,
      element: interfaces::CefPostDataElement) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove_element.unwrap())(
          self.c_object,
          CefWrap::to_c(element)))
    }
  }

  //
  // Add the specified post data element.  Returns true (1) if the add succeeds.
  //
  pub fn add_element(&self,
      element: interfaces::CefPostDataElement) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_element.unwrap())(
          self.c_object,
          CefWrap::to_c(element)))
    }
  }

  //
  // Remove all existing post data elements.
  //
  pub fn remove_elements(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove_elements.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_post_data_t object.
  //
  pub fn create() -> interfaces::CefPostData {
    unsafe {
      CefWrap::to_rust(
        ::request::cef_post_data_create(
))
    }
  }
}

impl CefWrap<*mut cef_post_data_t> for CefPostData {
  fn to_c(rust_object: CefPostData) -> *mut cef_post_data_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_post_data_t) -> CefPostData {
    CefPostData::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_post_data_t> for Option<CefPostData> {
  fn to_c(rust_object: Option<CefPostData>) -> *mut cef_post_data_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_post_data_t) -> Option<CefPostData> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPostData::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to represent a single element in the request post data. The
// functions of this structure may be called on any thread.
//
#[repr(C)]
pub struct _cef_post_data_element_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is read-only.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_post_data_element_t) -> libc::c_int>,

  //
  // Remove all contents from the post data element.
  //
  pub set_to_empty: Option<extern "C" fn(
      this: *mut cef_post_data_element_t) -> ()>,

  //
  // The post data element will represent a file.
  //
  pub set_to_file: Option<extern "C" fn(this: *mut cef_post_data_element_t,
      fileName: *const types::cef_string_t) -> ()>,

  //
  // The post data element will represent bytes.  The bytes passed in will be
  // copied.
  //
  pub set_to_bytes: Option<extern "C" fn(this: *mut cef_post_data_element_t,
      size: libc::size_t, bytes: *const ()) -> ()>,

  //
  // Return the type of this post data element.
  //
  pub get_type: Option<extern "C" fn(
      this: *mut cef_post_data_element_t) -> types::cef_postdataelement_type_t>,

  //
  // Return the file name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_file: Option<extern "C" fn(
      this: *mut cef_post_data_element_t) -> types::cef_string_userfree_t>,

  //
  // Return the number of bytes.
  //
  pub get_bytes_count: Option<extern "C" fn(
      this: *mut cef_post_data_element_t) -> libc::size_t>,

  //
  // Read up to |size| bytes into |bytes| and return the number of bytes
  // actually read.
  //
  pub get_bytes: Option<extern "C" fn(this: *mut cef_post_data_element_t,
      size: libc::size_t, bytes: *mut ()) -> libc::size_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_post_data_element_t = _cef_post_data_element_t;


//
// Structure used to represent a single element in the request post data. The
// functions of this structure may be called on any thread.
//
pub struct CefPostDataElement {
  c_object: *mut cef_post_data_element_t,
}

impl Clone for CefPostDataElement {
  fn clone(&self) -> CefPostDataElement{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPostDataElement {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPostDataElement {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPostDataElement {
  pub unsafe fn from_c_object(c_object: *mut cef_post_data_element_t) -> CefPostDataElement {
    CefPostDataElement {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_post_data_element_t) -> CefPostDataElement {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPostDataElement {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_post_data_element_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_post_data_element_t {
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
  // Remove all contents from the post data element.
  //
  pub fn set_to_empty(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_to_empty.unwrap())(
          self.c_object))
    }
  }

  //
  // The post data element will represent a file.
  //
  pub fn set_to_file(&self, fileName: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_to_file.unwrap())(
          self.c_object,
          CefWrap::to_c(fileName)))
    }
  }

  //
  // The post data element will represent bytes.  The bytes passed in will be
  // copied.
  //
  pub fn set_to_bytes(&self, size: libc::size_t, bytes: &()) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_to_bytes.unwrap())(
          self.c_object,
          CefWrap::to_c(size),
          CefWrap::to_c(bytes)))
    }
  }

  //
  // Return the type of this post data element.
  //
  pub fn get_type(&self) -> types::cef_postdataelement_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the file name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_file(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the number of bytes.
  //
  pub fn get_bytes_count(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_bytes_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Read up to |size| bytes into |bytes| and return the number of bytes
  // actually read.
  //
  pub fn get_bytes(&self, size: libc::size_t, bytes: &mut ()) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_bytes.unwrap())(
          self.c_object,
          CefWrap::to_c(size),
          CefWrap::to_c(bytes)))
    }
  }

  //
  // Create a new cef_post_data_element_t object.
  //
  pub fn create() -> interfaces::CefPostDataElement {
    unsafe {
      CefWrap::to_rust(
        ::request::cef_post_data_element_create(
))
    }
  }
}

impl CefWrap<*mut cef_post_data_element_t> for CefPostDataElement {
  fn to_c(rust_object: CefPostDataElement) -> *mut cef_post_data_element_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_post_data_element_t) -> CefPostDataElement {
    CefPostDataElement::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_post_data_element_t> for Option<CefPostDataElement> {
  fn to_c(rust_object: Option<CefPostDataElement>) -> *mut cef_post_data_element_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_post_data_element_t) -> Option<CefPostDataElement> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPostDataElement::from_c_object_addref(c_object))
    }
  }
}

