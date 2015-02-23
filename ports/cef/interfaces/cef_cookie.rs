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
// Structure used for managing cookies. The functions of this structure may be
// called on any thread unless otherwise indicated.
//
#[repr(C)]
pub struct _cef_cookie_manager_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Set the schemes supported by this manager. By default only "http" and
  // "https" schemes are supported. Must be called before any cookies are
  // accessed.
  //
  pub set_supported_schemes: Option<extern "C" fn(
      this: *mut cef_cookie_manager_t, schemes: types::cef_string_list_t) -> (
      )>,

  //
  // Visit all cookies. The returned cookies are ordered by longest path, then
  // by earliest creation date. Returns false (0) if cookies cannot be accessed.
  //
  pub visit_all_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      visitor: *mut interfaces::cef_cookie_visitor_t) -> libc::c_int>,

  //
  // Visit a subset of cookies. The results are filtered by the given url
  // scheme, host, domain and path. If |includeHttpOnly| is true (1) HTTP-only
  // cookies will also be included in the results. The returned cookies are
  // ordered by longest path, then by earliest creation date. Returns false (0)
  // if cookies cannot be accessed.
  //
  pub visit_url_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t, includeHttpOnly: libc::c_int,
      visitor: *mut interfaces::cef_cookie_visitor_t) -> libc::c_int>,

  //
  // Sets a cookie given a valid URL and explicit user-provided cookie
  // attributes. This function expects each attribute to be well-formed. It will
  // check for disallowed characters (e.g. the ';' character is disallowed
  // within the cookie value attribute) and will return false (0) without
  // setting the cookie if such characters are found. This function must be
  // called on the IO thread.
  //
  pub set_cookie: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t,
      cookie: *const interfaces::cef_cookie_t) -> libc::c_int>,

  //
  // Delete all cookies that match the specified parameters. If both |url| and
  // values |cookie_name| are specified all host and domain cookies matching
  // both will be deleted. If only |url| is specified all host cookies (but not
  // domain cookies) irrespective of path will be deleted. If |url| is NULL all
  // cookies for all hosts and domains will be deleted. Returns false (0) if a
  // non- NULL invalid URL is specified or if cookies cannot be accessed. This
  // function must be called on the IO thread.
  //
  pub delete_cookies: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      url: *const types::cef_string_t,
      cookie_name: *const types::cef_string_t) -> libc::c_int>,

  //
  // Sets the directory path that will be used for storing cookie data. If
  // |path| is NULL data will be stored in memory only. Otherwise, data will be
  // stored at the specified |path|. To persist session cookies (cookies without
  // an expiry date or validity interval) set |persist_session_cookies| to true
  // (1). Session cookies are generally intended to be transient and most Web
  // browsers do not persist them. Returns false (0) if cookies cannot be
  // accessed.
  //
  pub set_storage_path: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      path: *const types::cef_string_t,
      persist_session_cookies: libc::c_int) -> libc::c_int>,

  //
  // Flush the backing store (if any) to disk and execute the specified
  // |callback| on the IO thread when done. Returns false (0) if cookies cannot
  // be accessed.
  //
  pub flush_store: Option<extern "C" fn(this: *mut cef_cookie_manager_t,
      callback: *mut interfaces::cef_completion_callback_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_cookie_manager_t = _cef_cookie_manager_t;


//
// Structure used for managing cookies. The functions of this structure may be
// called on any thread unless otherwise indicated.
//
pub struct CefCookieManager {
  c_object: *mut cef_cookie_manager_t,
}

impl Clone for CefCookieManager {
  fn clone(&self) -> CefCookieManager{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefCookieManager {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefCookieManager {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefCookieManager {
  pub unsafe fn from_c_object(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    CefCookieManager {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefCookieManager {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_cookie_manager_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_cookie_manager_t {
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
  // Set the schemes supported by this manager. By default only "http" and
  // "https" schemes are supported. Must be called before any cookies are
  // accessed.
  //
  pub fn set_supported_schemes(&self, schemes: Vec<String>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_supported_schemes.unwrap())(
          self.c_object,
          CefWrap::to_c(schemes)))
    }
  }

  //
  // Visit all cookies. The returned cookies are ordered by longest path, then
  // by earliest creation date. Returns false (0) if cookies cannot be accessed.
  //
  pub fn visit_all_cookies(&self,
      visitor: interfaces::CefCookieVisitor) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit_all_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Visit a subset of cookies. The results are filtered by the given url
  // scheme, host, domain and path. If |includeHttpOnly| is true (1) HTTP-only
  // cookies will also be included in the results. The returned cookies are
  // ordered by longest path, then by earliest creation date. Returns false (0)
  // if cookies cannot be accessed.
  //
  pub fn visit_url_cookies(&self, url: &[u16], includeHttpOnly: libc::c_int,
      visitor: interfaces::CefCookieVisitor) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit_url_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(includeHttpOnly),
          CefWrap::to_c(visitor)))
    }
  }

  //
  // Sets a cookie given a valid URL and explicit user-provided cookie
  // attributes. This function expects each attribute to be well-formed. It will
  // check for disallowed characters (e.g. the ';' character is disallowed
  // within the cookie value attribute) and will return false (0) without
  // setting the cookie if such characters are found. This function must be
  // called on the IO thread.
  //
  pub fn set_cookie(&self, url: &[u16],
      cookie: &interfaces::CefCookie) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_cookie.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(cookie)))
    }
  }

  //
  // Delete all cookies that match the specified parameters. If both |url| and
  // values |cookie_name| are specified all host and domain cookies matching
  // both will be deleted. If only |url| is specified all host cookies (but not
  // domain cookies) irrespective of path will be deleted. If |url| is NULL all
  // cookies for all hosts and domains will be deleted. Returns false (0) if a
  // non- NULL invalid URL is specified or if cookies cannot be accessed. This
  // function must be called on the IO thread.
  //
  pub fn delete_cookies(&self, url: &[u16],
      cookie_name: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).delete_cookies.unwrap())(
          self.c_object,
          CefWrap::to_c(url),
          CefWrap::to_c(cookie_name)))
    }
  }

  //
  // Sets the directory path that will be used for storing cookie data. If
  // |path| is NULL data will be stored in memory only. Otherwise, data will be
  // stored at the specified |path|. To persist session cookies (cookies without
  // an expiry date or validity interval) set |persist_session_cookies| to true
  // (1). Session cookies are generally intended to be transient and most Web
  // browsers do not persist them. Returns false (0) if cookies cannot be
  // accessed.
  //
  pub fn set_storage_path(&self, path: &[u16],
      persist_session_cookies: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_storage_path.unwrap())(
          self.c_object,
          CefWrap::to_c(path),
          CefWrap::to_c(persist_session_cookies)))
    }
  }

  //
  // Flush the backing store (if any) to disk and execute the specified
  // |callback| on the IO thread when done. Returns false (0) if cookies cannot
  // be accessed.
  //
  pub fn flush_store(&self,
      callback: interfaces::CefCompletionCallback) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).flush_store.unwrap())(
          self.c_object,
          CefWrap::to_c(callback)))
    }
  }

  //
  // Returns the global cookie manager. By default data will be stored at
  // CefSettings.cache_path if specified or in memory otherwise.
  //
  pub fn get_global_manager() -> interfaces::CefCookieManager {
    unsafe {
      CefWrap::to_rust(
        ::cookie::cef_cookie_manager_get_global_manager(
))
    }
  }

  //
  // Creates a new cookie manager. If |path| is NULL data will be stored in
  // memory only. Otherwise, data will be stored at the specified |path|. To
  // persist session cookies (cookies without an expiry date or validity
  // interval) set |persist_session_cookies| to true (1). Session cookies are
  // generally intended to be transient and most Web browsers do not persist
  // them. Returns NULL if creation fails.
  //
  pub fn create_manager(path: &[u16],
      persist_session_cookies: libc::c_int) -> interfaces::CefCookieManager {
    unsafe {
      CefWrap::to_rust(
        ::cookie::cef_cookie_manager_create_manager(
          CefWrap::to_c(path),
          CefWrap::to_c(persist_session_cookies)))
    }
  }
}

impl CefWrap<*mut cef_cookie_manager_t> for CefCookieManager {
  fn to_c(rust_object: CefCookieManager) -> *mut cef_cookie_manager_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_manager_t) -> CefCookieManager {
    CefCookieManager::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_cookie_manager_t> for Option<CefCookieManager> {
  fn to_c(rust_object: Option<CefCookieManager>) -> *mut cef_cookie_manager_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_manager_t) -> Option<CefCookieManager> {
    if c_object.is_null() {
      None
    } else {
      Some(CefCookieManager::from_c_object_addref(c_object))
    }
  }
}


//
// Structure to implement for visiting cookie values. The functions of this
// structure will always be called on the IO thread.
//
#[repr(C)]
pub struct _cef_cookie_visitor_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be called once for each cookie. |count| is the 0-based
  // index for the current cookie. |total| is the total number of cookies. Set
  // |deleteCookie| to true (1) to delete the cookie currently being visited.
  // Return false (0) to stop visiting cookies. This function may never be
  // called if no cookies are found.
  //
  pub visit: Option<extern "C" fn(this: *mut cef_cookie_visitor_t,
      cookie: *const interfaces::cef_cookie_t, count: libc::c_int,
      total: libc::c_int, deleteCookie: *mut libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_cookie_visitor_t = _cef_cookie_visitor_t;


//
// Structure to implement for visiting cookie values. The functions of this
// structure will always be called on the IO thread.
//
pub struct CefCookieVisitor {
  c_object: *mut cef_cookie_visitor_t,
}

impl Clone for CefCookieVisitor {
  fn clone(&self) -> CefCookieVisitor{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefCookieVisitor {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefCookieVisitor {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefCookieVisitor {
  pub unsafe fn from_c_object(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    CefCookieVisitor {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefCookieVisitor {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_cookie_visitor_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_cookie_visitor_t {
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
  // Method that will be called once for each cookie. |count| is the 0-based
  // index for the current cookie. |total| is the total number of cookies. Set
  // |deleteCookie| to true (1) to delete the cookie currently being visited.
  // Return false (0) to stop visiting cookies. This function may never be
  // called if no cookies are found.
  //
  pub fn visit(&self, cookie: &interfaces::CefCookie, count: libc::c_int,
      total: libc::c_int, deleteCookie: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).visit.unwrap())(
          self.c_object,
          CefWrap::to_c(cookie),
          CefWrap::to_c(count),
          CefWrap::to_c(total),
          CefWrap::to_c(deleteCookie)))
    }
  }
}

impl CefWrap<*mut cef_cookie_visitor_t> for CefCookieVisitor {
  fn to_c(rust_object: CefCookieVisitor) -> *mut cef_cookie_visitor_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_visitor_t) -> CefCookieVisitor {
    CefCookieVisitor::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_cookie_visitor_t> for Option<CefCookieVisitor> {
  fn to_c(rust_object: Option<CefCookieVisitor>) -> *mut cef_cookie_visitor_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_cookie_visitor_t) -> Option<CefCookieVisitor> {
    if c_object.is_null() {
      None
    } else {
      Some(CefCookieVisitor::from_c_object_addref(c_object))
    }
  }
}

