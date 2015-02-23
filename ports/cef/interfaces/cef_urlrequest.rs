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
// Structure used to make a URL request. URL requests are not associated with a
// browser instance so no cef_client_t callbacks will be executed. URL requests
// can be created on any valid CEF thread in either the browser or render
// process. Once created the functions of the URL request object must be
// accessed on the same thread that created it.
//
#[repr(C)]
pub struct _cef_urlrequest_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns the request object used to create this URL request. The returned
  // object is read-only and should not be modified.
  //
  pub get_request: Option<extern "C" fn(
      this: *mut cef_urlrequest_t) -> *mut interfaces::cef_request_t>,

  //
  // Returns the client.
  //
  pub get_client: Option<extern "C" fn(
      this: *mut cef_urlrequest_t) -> *mut interfaces::cef_urlrequest_client_t>,

  //
  // Returns the request status.
  //
  pub get_request_status: Option<extern "C" fn(
      this: *mut cef_urlrequest_t) -> types::cef_urlrequest_status_t>,

  //
  // Returns the request error if status is UR_CANCELED or UR_FAILED, or 0
  // otherwise.
  //
  pub get_request_error: Option<extern "C" fn(
      this: *mut cef_urlrequest_t) -> types::cef_errorcode_t>,

  //
  // Returns the response, or NULL if no response information is available.
  // Response information will only be available after the upload has completed.
  // The returned object is read-only and should not be modified.
  //
  pub get_response: Option<extern "C" fn(
      this: *mut cef_urlrequest_t) -> *mut interfaces::cef_response_t>,

  //
  // Cancel the request.
  //
  pub cancel: Option<extern "C" fn(this: *mut cef_urlrequest_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_urlrequest_t = _cef_urlrequest_t;


//
// Structure used to make a URL request. URL requests are not associated with a
// browser instance so no cef_client_t callbacks will be executed. URL requests
// can be created on any valid CEF thread in either the browser or render
// process. Once created the functions of the URL request object must be
// accessed on the same thread that created it.
//
pub struct CefURLRequest {
  c_object: *mut cef_urlrequest_t,
}

impl Clone for CefURLRequest {
  fn clone(&self) -> CefURLRequest{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefURLRequest {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefURLRequest {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefURLRequest {
  pub unsafe fn from_c_object(c_object: *mut cef_urlrequest_t) -> CefURLRequest {
    CefURLRequest {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_urlrequest_t) -> CefURLRequest {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefURLRequest {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_urlrequest_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_urlrequest_t {
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
  // Returns the request object used to create this URL request. The returned
  // object is read-only and should not be modified.
  //
  pub fn get_request(&self) -> interfaces::CefRequest {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_request.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the client.
  //
  pub fn get_client(&self) -> interfaces::CefURLRequestClient {
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
  // Returns the request status.
  //
  pub fn get_request_status(&self) -> types::cef_urlrequest_status_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_request_status.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the request error if status is UR_CANCELED or UR_FAILED, or 0
  // otherwise.
  //
  pub fn get_request_error(&self) -> types::cef_errorcode_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_request_error.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the response, or NULL if no response information is available.
  // Response information will only be available after the upload has completed.
  // The returned object is read-only and should not be modified.
  //
  pub fn get_response(&self) -> interfaces::CefResponse {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_response.unwrap())(
          self.c_object))
    }
  }

  //
  // Cancel the request.
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

  //
  // Create a new URL request. Only GET, POST, HEAD, DELETE and PUT request
  // functions are supported. Multiple post data elements are not supported and
  // elements of type PDE_TYPE_FILE are only supported for requests originating
  // from the browser process. Requests originating from the render process will
  // receive the same handling as requests originating from Web content -- if
  // the response contains Content-Disposition or Mime-Type header values that
  // would not normally be rendered then the response may receive special
  // handling inside the browser (for example, via the file download code path
  // instead of the URL request code path). The |request| object will be marked
  // as read-only after calling this function.
  //
  pub fn create(request: interfaces::CefRequest,
      client: interfaces::CefURLRequestClient) -> interfaces::CefURLRequest {
    unsafe {
      CefWrap::to_rust(
        ::urlrequest::cef_urlrequest_create(
          CefWrap::to_c(request),
          CefWrap::to_c(client)))
    }
  }
}

impl CefWrap<*mut cef_urlrequest_t> for CefURLRequest {
  fn to_c(rust_object: CefURLRequest) -> *mut cef_urlrequest_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_urlrequest_t) -> CefURLRequest {
    CefURLRequest::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_urlrequest_t> for Option<CefURLRequest> {
  fn to_c(rust_object: Option<CefURLRequest>) -> *mut cef_urlrequest_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_urlrequest_t) -> Option<CefURLRequest> {
    if c_object.is_null() {
      None
    } else {
      Some(CefURLRequest::from_c_object_addref(c_object))
    }
  }
}


//
// Structure that should be implemented by the cef_urlrequest_t client. The
// functions of this structure will be called on the same thread that created
// the request unless otherwise documented.
//
#[repr(C)]
pub struct _cef_urlrequest_client_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Notifies the client that the request has completed. Use the
  // cef_urlrequest_t::GetRequestStatus function to determine if the request was
  // successful or not.
  //
  pub on_request_complete: Option<extern "C" fn(
      this: *mut cef_urlrequest_client_t,
      request: *mut interfaces::cef_urlrequest_t) -> ()>,

  //
  // Notifies the client of upload progress. |current| denotes the number of
  // bytes sent so far and |total| is the total size of uploading data (or -1 if
  // chunked upload is enabled). This function will only be called if the
  // UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
  //
  pub on_upload_progress: Option<extern "C" fn(
      this: *mut cef_urlrequest_client_t,
      request: *mut interfaces::cef_urlrequest_t, current: u64,
      total: u64) -> ()>,

  //
  // Notifies the client of download progress. |current| denotes the number of
  // bytes received up to the call and |total| is the expected total size of the
  // response (or -1 if not determined).
  //
  pub on_download_progress: Option<extern "C" fn(
      this: *mut cef_urlrequest_client_t,
      request: *mut interfaces::cef_urlrequest_t, current: u64,
      total: u64) -> ()>,

  //
  // Called when some part of the response is read. |data| contains the current
  // bytes received since the last call. This function will not be called if the
  // UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
  //
  pub on_download_data: Option<extern "C" fn(this: *mut cef_urlrequest_client_t,
      request: *mut interfaces::cef_urlrequest_t, data: *const (),
      data_length: libc::size_t) -> ()>,

  //
  // Called on the IO thread when the browser needs credentials from the user.
  // |isProxy| indicates whether the host is a proxy server. |host| contains the
  // hostname and |port| contains the port number. Return true (1) to continue
  // the request and call cef_auth_callback_t::cont() when the authentication
  // information is available. Return false (0) to cancel the request. This
  // function will only be called for requests initiated from the browser
  // process.
  //
  pub get_auth_credentials: Option<extern "C" fn(
      this: *mut cef_urlrequest_client_t, isProxy: libc::c_int,
      host: *const types::cef_string_t, port: libc::c_int,
      realm: *const types::cef_string_t, scheme: *const types::cef_string_t,
      callback: *mut interfaces::cef_auth_callback_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_urlrequest_client_t = _cef_urlrequest_client_t;


//
// Structure that should be implemented by the cef_urlrequest_t client. The
// functions of this structure will be called on the same thread that created
// the request unless otherwise documented.
//
pub struct CefURLRequestClient {
  c_object: *mut cef_urlrequest_client_t,
}

impl Clone for CefURLRequestClient {
  fn clone(&self) -> CefURLRequestClient{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefURLRequestClient {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefURLRequestClient {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefURLRequestClient {
  pub unsafe fn from_c_object(c_object: *mut cef_urlrequest_client_t) -> CefURLRequestClient {
    CefURLRequestClient {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_urlrequest_client_t) -> CefURLRequestClient {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefURLRequestClient {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_urlrequest_client_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_urlrequest_client_t {
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
  // Notifies the client that the request has completed. Use the
  // cef_urlrequest_t::GetRequestStatus function to determine if the request was
  // successful or not.
  //
  pub fn on_request_complete(&self, request: interfaces::CefURLRequest) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_request_complete.unwrap())(
          self.c_object,
          CefWrap::to_c(request)))
    }
  }

  //
  // Notifies the client of upload progress. |current| denotes the number of
  // bytes sent so far and |total| is the total size of uploading data (or -1 if
  // chunked upload is enabled). This function will only be called if the
  // UR_FLAG_REPORT_UPLOAD_PROGRESS flag is set on the request.
  //
  pub fn on_upload_progress(&self, request: interfaces::CefURLRequest,
      current: u64, total: u64) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_upload_progress.unwrap())(
          self.c_object,
          CefWrap::to_c(request),
          CefWrap::to_c(current),
          CefWrap::to_c(total)))
    }
  }

  //
  // Notifies the client of download progress. |current| denotes the number of
  // bytes received up to the call and |total| is the expected total size of the
  // response (or -1 if not determined).
  //
  pub fn on_download_progress(&self, request: interfaces::CefURLRequest,
      current: u64, total: u64) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_download_progress.unwrap())(
          self.c_object,
          CefWrap::to_c(request),
          CefWrap::to_c(current),
          CefWrap::to_c(total)))
    }
  }

  //
  // Called when some part of the response is read. |data| contains the current
  // bytes received since the last call. This function will not be called if the
  // UR_FLAG_NO_DOWNLOAD_DATA flag is set on the request.
  //
  pub fn on_download_data(&self, request: interfaces::CefURLRequest, data: &(),
      data_length: libc::size_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_download_data.unwrap())(
          self.c_object,
          CefWrap::to_c(request),
          CefWrap::to_c(data),
          CefWrap::to_c(data_length)))
    }
  }

  //
  // Called on the IO thread when the browser needs credentials from the user.
  // |isProxy| indicates whether the host is a proxy server. |host| contains the
  // hostname and |port| contains the port number. Return true (1) to continue
  // the request and call cef_auth_callback_t::cont() when the authentication
  // information is available. Return false (0) to cancel the request. This
  // function will only be called for requests initiated from the browser
  // process.
  //
  pub fn get_auth_credentials(&self, isProxy: libc::c_int, host: &[u16],
      port: libc::c_int, realm: &[u16], scheme: &[u16],
      callback: interfaces::CefAuthCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_auth_credentials.unwrap())(
          self.c_object,
          CefWrap::to_c(isProxy),
          CefWrap::to_c(host),
          CefWrap::to_c(port),
          CefWrap::to_c(realm),
          CefWrap::to_c(scheme),
          CefWrap::to_c(callback)))
    }
  }
}

impl CefWrap<*mut cef_urlrequest_client_t> for CefURLRequestClient {
  fn to_c(rust_object: CefURLRequestClient) -> *mut cef_urlrequest_client_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_urlrequest_client_t) -> CefURLRequestClient {
    CefURLRequestClient::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_urlrequest_client_t> for Option<CefURLRequestClient> {
  fn to_c(rust_object: Option<CefURLRequestClient>) -> *mut cef_urlrequest_client_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_urlrequest_client_t) -> Option<CefURLRequestClient> {
    if c_object.is_null() {
      None
    } else {
      Some(CefURLRequestClient::from_c_object_addref(c_object))
    }
  }
}

