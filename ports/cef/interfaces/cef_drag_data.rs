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
// Structure used to represent drag data. The functions of this structure may be
// called on any thread.
//
#[repr(C)]
pub struct _cef_drag_data_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns a copy of the current object.
  //
  pub clone: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> *mut interfaces::cef_drag_data_t>,

  //
  // Returns true (1) if this object is read-only.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> libc::c_int>,

  //
  // Returns true (1) if the drag data is a link.
  //
  pub is_link: Option<extern "C" fn(this: *mut cef_drag_data_t) -> libc::c_int>,

  //
  // Returns true (1) if the drag data is a text or html fragment.
  //
  pub is_fragment: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> libc::c_int>,

  //
  // Returns true (1) if the drag data is a file.
  //
  pub is_file: Option<extern "C" fn(this: *mut cef_drag_data_t) -> libc::c_int>,

  //
  // Return the link URL that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_link_url: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the title associated with the link being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_link_title: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the metadata, if any, associated with the link being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_link_metadata: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the plain text fragment that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_fragment_text: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the text/html fragment that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_fragment_html: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the base URL that the fragment came from. This value is used for
  // resolving relative URLs and may be NULL.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_fragment_base_url: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Return the name of the file being dragged out of the browser window.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_file_name: Option<extern "C" fn(
      this: *mut cef_drag_data_t) -> types::cef_string_userfree_t>,

  //
  // Write the contents of the file being dragged out of the web view into
  // |writer|. Returns the number of bytes sent to |writer|. If |writer| is NULL
  // this function will return the size of the file contents in bytes. Call
  // get_file_name() to get a suggested name for the file.
  //
  pub get_file_contents: Option<extern "C" fn(this: *mut cef_drag_data_t,
      writer: *mut interfaces::cef_stream_writer_t) -> libc::size_t>,

  //
  // Retrieve the list of file names that are being dragged into the browser
  // window.
  //
  pub get_file_names: Option<extern "C" fn(this: *mut cef_drag_data_t,
      names: types::cef_string_list_t) -> libc::c_int>,

  //
  // Set the link URL that is being dragged.
  //
  pub set_link_url: Option<extern "C" fn(this: *mut cef_drag_data_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Set the title associated with the link being dragged.
  //
  pub set_link_title: Option<extern "C" fn(this: *mut cef_drag_data_t,
      title: *const types::cef_string_t) -> ()>,

  //
  // Set the metadata associated with the link being dragged.
  //
  pub set_link_metadata: Option<extern "C" fn(this: *mut cef_drag_data_t,
      data: *const types::cef_string_t) -> ()>,

  //
  // Set the plain text fragment that is being dragged.
  //
  pub set_fragment_text: Option<extern "C" fn(this: *mut cef_drag_data_t,
      text: *const types::cef_string_t) -> ()>,

  //
  // Set the text/html fragment that is being dragged.
  //
  pub set_fragment_html: Option<extern "C" fn(this: *mut cef_drag_data_t,
      html: *const types::cef_string_t) -> ()>,

  //
  // Set the base URL that the fragment came from.
  //
  pub set_fragment_base_url: Option<extern "C" fn(this: *mut cef_drag_data_t,
      base_url: *const types::cef_string_t) -> ()>,

  //
  // Reset the file contents. You should do this before calling
  // cef_browser_host_t::DragTargetDragEnter as the web view does not allow us
  // to drag in this kind of data.
  //
  pub reset_file_contents: Option<extern "C" fn(this: *mut cef_drag_data_t) -> (
      )>,

  //
  // Add a file that is being dragged into the webview.
  //
  pub add_file: Option<extern "C" fn(this: *mut cef_drag_data_t,
      path: *const types::cef_string_t,
      display_name: *const types::cef_string_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_drag_data_t = _cef_drag_data_t;


//
// Structure used to represent drag data. The functions of this structure may be
// called on any thread.
//
pub struct CefDragData {
  c_object: *mut cef_drag_data_t,
}

impl Clone for CefDragData {
  fn clone(&self) -> CefDragData{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDragData {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDragData {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDragData {
  pub unsafe fn from_c_object(c_object: *mut cef_drag_data_t) -> CefDragData {
    CefDragData {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_drag_data_t) -> CefDragData {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDragData {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_drag_data_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_drag_data_t {
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
  // Returns a copy of the current object.
  //
  pub fn clone(&self) -> interfaces::CefDragData {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).clone.unwrap())(
          self.c_object))
    }
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
  // Returns true (1) if the drag data is a link.
  //
  pub fn is_link(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_link.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the drag data is a text or html fragment.
  //
  pub fn is_fragment(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_fragment.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the drag data is a file.
  //
  pub fn is_file(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_file.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the link URL that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_link_url(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_link_url.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the title associated with the link being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_link_title(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_link_title.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the metadata, if any, associated with the link being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_link_metadata(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_link_metadata.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the plain text fragment that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_fragment_text(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_fragment_text.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the text/html fragment that is being dragged.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_fragment_html(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_fragment_html.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the base URL that the fragment came from. This value is used for
  // resolving relative URLs and may be NULL.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_fragment_base_url(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_fragment_base_url.unwrap())(
          self.c_object))
    }
  }

  //
  // Return the name of the file being dragged out of the browser window.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_file_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Write the contents of the file being dragged out of the web view into
  // |writer|. Returns the number of bytes sent to |writer|. If |writer| is NULL
  // this function will return the size of the file contents in bytes. Call
  // get_file_name() to get a suggested name for the file.
  //
  pub fn get_file_contents(&self,
      writer: interfaces::CefStreamWriter) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_contents.unwrap())(
          self.c_object,
          CefWrap::to_c(writer)))
    }
  }

  //
  // Retrieve the list of file names that are being dragged into the browser
  // window.
  //
  pub fn get_file_names(&self, names: Vec<String>) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_file_names.unwrap())(
          self.c_object,
          CefWrap::to_c(names)))
    }
  }

  //
  // Set the link URL that is being dragged.
  //
  pub fn set_link_url(&self, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_link_url.unwrap())(
          self.c_object,
          CefWrap::to_c(url)))
    }
  }

  //
  // Set the title associated with the link being dragged.
  //
  pub fn set_link_title(&self, title: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_link_title.unwrap())(
          self.c_object,
          CefWrap::to_c(title)))
    }
  }

  //
  // Set the metadata associated with the link being dragged.
  //
  pub fn set_link_metadata(&self, data: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_link_metadata.unwrap())(
          self.c_object,
          CefWrap::to_c(data)))
    }
  }

  //
  // Set the plain text fragment that is being dragged.
  //
  pub fn set_fragment_text(&self, text: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_fragment_text.unwrap())(
          self.c_object,
          CefWrap::to_c(text)))
    }
  }

  //
  // Set the text/html fragment that is being dragged.
  //
  pub fn set_fragment_html(&self, html: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_fragment_html.unwrap())(
          self.c_object,
          CefWrap::to_c(html)))
    }
  }

  //
  // Set the base URL that the fragment came from.
  //
  pub fn set_fragment_base_url(&self, base_url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_fragment_base_url.unwrap())(
          self.c_object,
          CefWrap::to_c(base_url)))
    }
  }

  //
  // Reset the file contents. You should do this before calling
  // cef_browser_host_t::DragTargetDragEnter as the web view does not allow us
  // to drag in this kind of data.
  //
  pub fn reset_file_contents(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).reset_file_contents.unwrap())(
          self.c_object))
    }
  }

  //
  // Add a file that is being dragged into the webview.
  //
  pub fn add_file(&self, path: &[u16], display_name: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_file.unwrap())(
          self.c_object,
          CefWrap::to_c(path),
          CefWrap::to_c(display_name)))
    }
  }

  //
  // Create a new cef_drag_data_t object.
  //
  pub fn create() -> interfaces::CefDragData {
    unsafe {
      CefWrap::to_rust(
        ::drag_data::cef_drag_data_create(
))
    }
  }
}

impl CefWrap<*mut cef_drag_data_t> for CefDragData {
  fn to_c(rust_object: CefDragData) -> *mut cef_drag_data_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_drag_data_t) -> CefDragData {
    CefDragData::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_drag_data_t> for Option<CefDragData> {
  fn to_c(rust_object: Option<CefDragData>) -> *mut cef_drag_data_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_drag_data_t) -> Option<CefDragData> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDragData::from_c_object_addref(c_object))
    }
  }
}

