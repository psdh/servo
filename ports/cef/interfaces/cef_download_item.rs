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
// Structure used to represent a download item.
//
#[repr(C)]
pub struct _cef_download_item_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> libc::c_int>,

  //
  // Returns true (1) if the download is in progress.
  //
  pub is_in_progress: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> libc::c_int>,

  //
  // Returns true (1) if the download is complete.
  //
  pub is_complete: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> libc::c_int>,

  //
  // Returns true (1) if the download has been canceled or interrupted.
  //
  pub is_canceled: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> libc::c_int>,

  //
  // Returns a simple speed estimate in bytes/s.
  //
  pub get_current_speed: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> i64>,

  //
  // Returns the rough percent complete or -1 if the receive total size is
  // unknown.
  //
  pub get_percent_complete: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> libc::c_int>,

  //
  // Returns the total number of bytes.
  //
  pub get_total_bytes: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> i64>,

  //
  // Returns the number of received bytes.
  //
  pub get_received_bytes: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> i64>,

  //
  // Returns the time that the download started.
  //
  pub get_start_time: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_time_t>,

  //
  // Returns the time that the download ended.
  //
  pub get_end_time: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_time_t>,

  //
  // Returns the full path to the downloaded or downloading file.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_full_path: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_string_userfree_t>,

  //
  // Returns the unique identifier for this download.
  //
  pub get_id: Option<extern "C" fn(this: *mut cef_download_item_t) -> u32>,

  //
  // Returns the URL.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_url: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_string_userfree_t>,

  //
  // Returns the suggested file name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_suggested_file_name: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_string_userfree_t>,

  //
  // Returns the content disposition.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_content_disposition: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_string_userfree_t>,

  //
  // Returns the mime type.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_mime_type: Option<extern "C" fn(
      this: *mut cef_download_item_t) -> types::cef_string_userfree_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_download_item_t = _cef_download_item_t;


//
// Structure used to represent a download item.
//
pub struct CefDownloadItem {
  c_object: *mut cef_download_item_t,
}

impl Clone for CefDownloadItem {
  fn clone(&self) -> CefDownloadItem{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDownloadItem {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDownloadItem {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDownloadItem {
  pub unsafe fn from_c_object(c_object: *mut cef_download_item_t) -> CefDownloadItem {
    CefDownloadItem {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_download_item_t) -> CefDownloadItem {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDownloadItem {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_download_item_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_download_item_t {
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
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
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
  // Returns true (1) if the download is in progress.
  //
  pub fn is_in_progress(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_in_progress.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the download is complete.
  //
  pub fn is_complete(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_complete.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the download has been canceled or interrupted.
  //
  pub fn is_canceled(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_canceled.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns a simple speed estimate in bytes/s.
  //
  pub fn get_current_speed(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_current_speed.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the rough percent complete or -1 if the receive total size is
  // unknown.
  //
  pub fn get_percent_complete(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_percent_complete.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the total number of bytes.
  //
  pub fn get_total_bytes(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_total_bytes.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the number of received bytes.
  //
  pub fn get_received_bytes(&self) -> i64 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_received_bytes.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the time that the download started.
  //
  pub fn get_start_time(&self) -> types::cef_time_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_start_time.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the time that the download ended.
  //
  pub fn get_end_time(&self) -> types::cef_time_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_end_time.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the full path to the downloaded or downloading file.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_full_path(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_full_path.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the unique identifier for this download.
  //
  pub fn get_id(&self) -> u32 {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_id.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the URL.
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
  // Returns the suggested file name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_suggested_file_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_suggested_file_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the content disposition.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_content_disposition(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_content_disposition.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the mime type.
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
}

impl CefWrap<*mut cef_download_item_t> for CefDownloadItem {
  fn to_c(rust_object: CefDownloadItem) -> *mut cef_download_item_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_download_item_t) -> CefDownloadItem {
    CefDownloadItem::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_download_item_t> for Option<CefDownloadItem> {
  fn to_c(rust_object: Option<CefDownloadItem>) -> *mut cef_download_item_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_download_item_t) -> Option<CefDownloadItem> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDownloadItem::from_c_object_addref(c_object))
    }
  }
}

