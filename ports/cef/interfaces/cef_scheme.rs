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
// Structure that manages custom scheme registrations.
//
#[repr(C)]
pub struct _cef_scheme_registrar_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Register a custom scheme. This function should not be called for the built-
  // in HTTP, HTTPS, FILE, FTP, ABOUT and DATA schemes.
  //
  // If |is_standard| is true (1) the scheme will be treated as a standard
  // scheme. Standard schemes are subject to URL canonicalization and parsing
  // rules as defined in the Common Internet Scheme Syntax RFC 1738 Section 3.1
  // available at http://www.ietf.org/rfc/rfc1738.txt
  //
  // In particular, the syntax for standard scheme URLs must be of the form:
  // <pre>
  //  [scheme]://[username]:[password]@[host]:[port]/[url-path]
  // </pre> Standard scheme URLs must have a host component that is a fully
  // qualified domain name as defined in Section 3.5 of RFC 1034 [13] and
  // Section 2.1 of RFC 1123. These URLs will be canonicalized to
  // "scheme://host/path" in the simplest case and
  // "scheme://username:password@host:port/path" in the most explicit case. For
  // example, "scheme:host/path" and "scheme:///host/path" will both be
  // canonicalized to "scheme://host/path". The origin of a standard scheme URL
  // is the combination of scheme, host and port (i.e., "scheme://host:port" in
  // the most explicit case).
  //
  // For non-standard scheme URLs only the "scheme:" component is parsed and
  // canonicalized. The remainder of the URL will be passed to the handler as-
  // is. For example, "scheme:///some%20text" will remain the same. Non-standard
  // scheme URLs cannot be used as a target for form submission.
  //
  // If |is_local| is true (1) the scheme will be treated as local (i.e., with
  // the same security rules as those applied to "file" URLs). Normal pages
  // cannot link to or access local URLs. Also, by default, local URLs can only
  // perform XMLHttpRequest calls to the same URL (origin + path) that
  // originated the request. To allow XMLHttpRequest calls from a local URL to
  // other URLs with the same origin set the
  // CefSettings.file_access_from_file_urls_allowed value to true (1). To allow
  // XMLHttpRequest calls from a local URL to all origins set the
  // CefSettings.universal_access_from_file_urls_allowed value to true (1).
  //
  // If |is_display_isolated| is true (1) the scheme will be treated as display-
  // isolated. This means that pages cannot display these URLs unless they are
  // from the same scheme. For example, pages in another origin cannot create
  // iframes or hyperlinks to URLs with this scheme.
  //
  // This function may be called on any thread. It should only be called once
  // per unique |scheme_name| value. If |scheme_name| is already registered or
  // if an error occurs this function will return false (0).
  //
  pub add_custom_scheme: Option<extern "C" fn(this: *mut cef_scheme_registrar_t,
      scheme_name: *const types::cef_string_t, is_standard: libc::c_int,
      is_local: libc::c_int, is_display_isolated: libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_scheme_registrar_t = _cef_scheme_registrar_t;


//
// Structure that manages custom scheme registrations.
//
pub struct CefSchemeRegistrar {
  c_object: *mut cef_scheme_registrar_t,
}

impl Clone for CefSchemeRegistrar {
  fn clone(&self) -> CefSchemeRegistrar{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefSchemeRegistrar {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefSchemeRegistrar {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefSchemeRegistrar {
  pub unsafe fn from_c_object(c_object: *mut cef_scheme_registrar_t) -> CefSchemeRegistrar {
    CefSchemeRegistrar {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_scheme_registrar_t) -> CefSchemeRegistrar {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefSchemeRegistrar {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_scheme_registrar_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_scheme_registrar_t {
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
  // Register a custom scheme. This function should not be called for the built-
  // in HTTP, HTTPS, FILE, FTP, ABOUT and DATA schemes.
  //
  // If |is_standard| is true (1) the scheme will be treated as a standard
  // scheme. Standard schemes are subject to URL canonicalization and parsing
  // rules as defined in the Common Internet Scheme Syntax RFC 1738 Section 3.1
  // available at http://www.ietf.org/rfc/rfc1738.txt
  //
  // In particular, the syntax for standard scheme URLs must be of the form:
  // <pre>
  //  [scheme]://[username]:[password]@[host]:[port]/[url-path]
  // </pre> Standard scheme URLs must have a host component that is a fully
  // qualified domain name as defined in Section 3.5 of RFC 1034 [13] and
  // Section 2.1 of RFC 1123. These URLs will be canonicalized to
  // "scheme://host/path" in the simplest case and
  // "scheme://username:password@host:port/path" in the most explicit case. For
  // example, "scheme:host/path" and "scheme:///host/path" will both be
  // canonicalized to "scheme://host/path". The origin of a standard scheme URL
  // is the combination of scheme, host and port (i.e., "scheme://host:port" in
  // the most explicit case).
  //
  // For non-standard scheme URLs only the "scheme:" component is parsed and
  // canonicalized. The remainder of the URL will be passed to the handler as-
  // is. For example, "scheme:///some%20text" will remain the same. Non-standard
  // scheme URLs cannot be used as a target for form submission.
  //
  // If |is_local| is true (1) the scheme will be treated as local (i.e., with
  // the same security rules as those applied to "file" URLs). Normal pages
  // cannot link to or access local URLs. Also, by default, local URLs can only
  // perform XMLHttpRequest calls to the same URL (origin + path) that
  // originated the request. To allow XMLHttpRequest calls from a local URL to
  // other URLs with the same origin set the
  // CefSettings.file_access_from_file_urls_allowed value to true (1). To allow
  // XMLHttpRequest calls from a local URL to all origins set the
  // CefSettings.universal_access_from_file_urls_allowed value to true (1).
  //
  // If |is_display_isolated| is true (1) the scheme will be treated as display-
  // isolated. This means that pages cannot display these URLs unless they are
  // from the same scheme. For example, pages in another origin cannot create
  // iframes or hyperlinks to URLs with this scheme.
  //
  // This function may be called on any thread. It should only be called once
  // per unique |scheme_name| value. If |scheme_name| is already registered or
  // if an error occurs this function will return false (0).
  //
  pub fn add_custom_scheme(&self, scheme_name: &[u16], is_standard: libc::c_int,
      is_local: libc::c_int,
      is_display_isolated: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_custom_scheme.unwrap())(
          self.c_object,
          CefWrap::to_c(scheme_name),
          CefWrap::to_c(is_standard),
          CefWrap::to_c(is_local),
          CefWrap::to_c(is_display_isolated)))
    }
  }
}

impl CefWrap<*mut cef_scheme_registrar_t> for CefSchemeRegistrar {
  fn to_c(rust_object: CefSchemeRegistrar) -> *mut cef_scheme_registrar_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_scheme_registrar_t) -> CefSchemeRegistrar {
    CefSchemeRegistrar::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_scheme_registrar_t> for Option<CefSchemeRegistrar> {
  fn to_c(rust_object: Option<CefSchemeRegistrar>) -> *mut cef_scheme_registrar_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_scheme_registrar_t) -> Option<CefSchemeRegistrar> {
    if c_object.is_null() {
      None
    } else {
      Some(CefSchemeRegistrar::from_c_object_addref(c_object))
    }
  }
}


//
// Structure that creates cef_resource_handler_t instances for handling scheme
// requests. The functions of this structure will always be called on the IO
// thread.
//
#[repr(C)]
pub struct _cef_scheme_handler_factory_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Return a new resource handler instance to handle the request or an NULL
  // reference to allow default handling of the request. |browser| and |frame|
  // will be the browser window and frame respectively that originated the
  // request or NULL if the request did not originate from a browser window (for
  // example, if the request came from cef_urlrequest_t). The |request| object
  // passed to this function will not contain cookie data.
  //
  pub create: Option<extern "C" fn(this: *mut cef_scheme_handler_factory_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      scheme_name: *const types::cef_string_t,
      request: *mut interfaces::cef_request_t) -> *mut interfaces::cef_resource_handler_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_scheme_handler_factory_t = _cef_scheme_handler_factory_t;


//
// Structure that creates cef_resource_handler_t instances for handling scheme
// requests. The functions of this structure will always be called on the IO
// thread.
//
pub struct CefSchemeHandlerFactory {
  c_object: *mut cef_scheme_handler_factory_t,
}

impl Clone for CefSchemeHandlerFactory {
  fn clone(&self) -> CefSchemeHandlerFactory{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefSchemeHandlerFactory {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefSchemeHandlerFactory {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefSchemeHandlerFactory {
  pub unsafe fn from_c_object(c_object: *mut cef_scheme_handler_factory_t) -> CefSchemeHandlerFactory {
    CefSchemeHandlerFactory {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_scheme_handler_factory_t) -> CefSchemeHandlerFactory {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefSchemeHandlerFactory {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_scheme_handler_factory_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_scheme_handler_factory_t {
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
  // Return a new resource handler instance to handle the request or an NULL
  // reference to allow default handling of the request. |browser| and |frame|
  // will be the browser window and frame respectively that originated the
  // request or NULL if the request did not originate from a browser window (for
  // example, if the request came from cef_urlrequest_t). The |request| object
  // passed to this function will not contain cookie data.
  //
  pub fn create(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, scheme_name: &[u16],
      request: interfaces::CefRequest) -> interfaces::CefResourceHandler {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).create.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(scheme_name),
          CefWrap::to_c(request)))
    }
  }
}

impl CefWrap<*mut cef_scheme_handler_factory_t> for CefSchemeHandlerFactory {
  fn to_c(rust_object: CefSchemeHandlerFactory) -> *mut cef_scheme_handler_factory_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_scheme_handler_factory_t) -> CefSchemeHandlerFactory {
    CefSchemeHandlerFactory::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_scheme_handler_factory_t> for Option<CefSchemeHandlerFactory> {
  fn to_c(rust_object: Option<CefSchemeHandlerFactory>) -> *mut cef_scheme_handler_factory_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_scheme_handler_factory_t) -> Option<CefSchemeHandlerFactory> {
    if c_object.is_null() {
      None
    } else {
      Some(CefSchemeHandlerFactory::from_c_object_addref(c_object))
    }
  }
}

