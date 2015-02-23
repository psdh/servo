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
// Structure representing print settings.
//
#[repr(C)]
pub struct _cef_print_settings_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Returns true (1) if the values of this object are read-only. Some APIs may
  // expose read-only objects.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Returns a writable copy of this object.
  //
  pub copy: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> *mut interfaces::cef_print_settings_t>,

  //
  // Set the page orientation.
  //
  pub set_orientation: Option<extern "C" fn(this: *mut cef_print_settings_t,
      landscape: libc::c_int) -> ()>,

  //
  // Returns true (1) if the orientation is landscape.
  //
  pub is_landscape: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Set the printer printable area in device units. Some platforms already
  // provide flipped area. Set |landscape_needs_flip| to false (0) on those
  // platforms to avoid double flipping.
  //
  pub set_printer_printable_area: Option<extern "C" fn(
      this: *mut cef_print_settings_t,
      physical_size_device_units: *const types::cef_size_t,
      printable_area_device_units: *const types::cef_rect_t,
      landscape_needs_flip: libc::c_int) -> ()>,

  //
  // Set the device name.
  //
  pub set_device_name: Option<extern "C" fn(this: *mut cef_print_settings_t,
      name: *const types::cef_string_t) -> ()>,

  //
  // Get the device name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_device_name: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> types::cef_string_userfree_t>,

  //
  // Set the DPI (dots per inch).
  //
  pub set_dpi: Option<extern "C" fn(this: *mut cef_print_settings_t,
      dpi: libc::c_int) -> ()>,

  //
  // Get the DPI (dots per inch).
  //
  pub get_dpi: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Set the page ranges.
  //
  pub set_page_ranges: Option<extern "C" fn(this: *mut cef_print_settings_t,
      ranges_count: libc::size_t, ranges: *const types::cef_page_range_t) -> (
      )>,

  //
  // Returns the number of page ranges that currently exist.
  //
  pub get_page_ranges_count: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::size_t>,

  //
  // Retrieve the page ranges.
  //
  pub get_page_ranges: Option<extern "C" fn(this: *mut cef_print_settings_t,
      ranges_count: *mut libc::size_t,
      ranges: *mut types::cef_page_range_t) -> ()>,

  //
  // Set whether only the selection will be printed.
  //
  pub set_selection_only: Option<extern "C" fn(this: *mut cef_print_settings_t,
      selection_only: libc::c_int) -> ()>,

  //
  // Returns true (1) if only the selection will be printed.
  //
  pub is_selection_only: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Set whether pages will be collated.
  //
  pub set_collate: Option<extern "C" fn(this: *mut cef_print_settings_t,
      collate: libc::c_int) -> ()>,

  //
  // Returns true (1) if pages will be collated.
  //
  pub will_collate: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Set the color model.
  //
  pub set_color_model: Option<extern "C" fn(this: *mut cef_print_settings_t,
      model: types::cef_color_model_t) -> ()>,

  //
  // Get the color model.
  //
  pub get_color_model: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> types::cef_color_model_t>,

  //
  // Set the number of copies.
  //
  pub set_copies: Option<extern "C" fn(this: *mut cef_print_settings_t,
      copies: libc::c_int) -> ()>,

  //
  // Get the number of copies.
  //
  pub get_copies: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> libc::c_int>,

  //
  // Set the duplex mode.
  //
  pub set_duplex_mode: Option<extern "C" fn(this: *mut cef_print_settings_t,
      mode: types::cef_duplex_mode_t) -> ()>,

  //
  // Get the duplex mode.
  //
  pub get_duplex_mode: Option<extern "C" fn(
      this: *mut cef_print_settings_t) -> types::cef_duplex_mode_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_print_settings_t = _cef_print_settings_t;


//
// Structure representing print settings.
//
pub struct CefPrintSettings {
  c_object: *mut cef_print_settings_t,
}

impl Clone for CefPrintSettings {
  fn clone(&self) -> CefPrintSettings{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefPrintSettings {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefPrintSettings {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefPrintSettings {
  pub unsafe fn from_c_object(c_object: *mut cef_print_settings_t) -> CefPrintSettings {
    CefPrintSettings {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_print_settings_t) -> CefPrintSettings {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefPrintSettings {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_print_settings_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_print_settings_t {
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
  // Returns true (1) if the values of this object are read-only. Some APIs may
  // expose read-only objects.
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
  // Returns a writable copy of this object.
  //
  pub fn copy(&self) -> interfaces::CefPrintSettings {
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
  // Set the page orientation.
  //
  pub fn set_orientation(&self, landscape: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_orientation.unwrap())(
          self.c_object,
          CefWrap::to_c(landscape)))
    }
  }

  //
  // Returns true (1) if the orientation is landscape.
  //
  pub fn is_landscape(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_landscape.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the printer printable area in device units. Some platforms already
  // provide flipped area. Set |landscape_needs_flip| to false (0) on those
  // platforms to avoid double flipping.
  //
  pub fn set_printer_printable_area(&self,
      physical_size_device_units: &types::cef_size_t,
      printable_area_device_units: &types::cef_rect_t,
      landscape_needs_flip: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_printer_printable_area.unwrap())(
          self.c_object,
          CefWrap::to_c(physical_size_device_units),
          CefWrap::to_c(printable_area_device_units),
          CefWrap::to_c(landscape_needs_flip)))
    }
  }

  //
  // Set the device name.
  //
  pub fn set_device_name(&self, name: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_device_name.unwrap())(
          self.c_object,
          CefWrap::to_c(name)))
    }
  }

  //
  // Get the device name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_device_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_device_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the DPI (dots per inch).
  //
  pub fn set_dpi(&self, dpi: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_dpi.unwrap())(
          self.c_object,
          CefWrap::to_c(dpi)))
    }
  }

  //
  // Get the DPI (dots per inch).
  //
  pub fn get_dpi(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_dpi.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the page ranges.
  //
  pub fn set_page_ranges(&self, ranges_count: libc::size_t,
      ranges: *const types::cef_page_range_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_page_ranges.unwrap())(
          self.c_object,
          CefWrap::to_c(ranges_count),
          CefWrap::to_c(ranges)))
    }
  }

  //
  // Returns the number of page ranges that currently exist.
  //
  pub fn get_page_ranges_count(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_page_ranges_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Retrieve the page ranges.
  //
  pub fn get_page_ranges(&self, ranges_count: *mut libc::size_t,
      ranges: *mut types::cef_page_range_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_page_ranges.unwrap())(
          self.c_object,
          CefWrap::to_c(ranges_count),
          CefWrap::to_c(ranges)))
    }
  }

  //
  // Set whether only the selection will be printed.
  //
  pub fn set_selection_only(&self, selection_only: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_selection_only.unwrap())(
          self.c_object,
          CefWrap::to_c(selection_only)))
    }
  }

  //
  // Returns true (1) if only the selection will be printed.
  //
  pub fn is_selection_only(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_selection_only.unwrap())(
          self.c_object))
    }
  }

  //
  // Set whether pages will be collated.
  //
  pub fn set_collate(&self, collate: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_collate.unwrap())(
          self.c_object,
          CefWrap::to_c(collate)))
    }
  }

  //
  // Returns true (1) if pages will be collated.
  //
  pub fn will_collate(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).will_collate.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the color model.
  //
  pub fn set_color_model(&self, model: types::cef_color_model_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_color_model.unwrap())(
          self.c_object,
          CefWrap::to_c(model)))
    }
  }

  //
  // Get the color model.
  //
  pub fn get_color_model(&self) -> types::cef_color_model_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_color_model.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the number of copies.
  //
  pub fn set_copies(&self, copies: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_copies.unwrap())(
          self.c_object,
          CefWrap::to_c(copies)))
    }
  }

  //
  // Get the number of copies.
  //
  pub fn get_copies(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_copies.unwrap())(
          self.c_object))
    }
  }

  //
  // Set the duplex mode.
  //
  pub fn set_duplex_mode(&self, mode: types::cef_duplex_mode_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_duplex_mode.unwrap())(
          self.c_object,
          CefWrap::to_c(mode)))
    }
  }

  //
  // Get the duplex mode.
  //
  pub fn get_duplex_mode(&self) -> types::cef_duplex_mode_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_duplex_mode.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_print_settings_t object.
  //
  pub fn create() -> interfaces::CefPrintSettings {
    unsafe {
      CefWrap::to_rust(
        ::print_settings::cef_print_settings_create(
))
    }
  }
}

impl CefWrap<*mut cef_print_settings_t> for CefPrintSettings {
  fn to_c(rust_object: CefPrintSettings) -> *mut cef_print_settings_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_print_settings_t) -> CefPrintSettings {
    CefPrintSettings::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_print_settings_t> for Option<CefPrintSettings> {
  fn to_c(rust_object: Option<CefPrintSettings>) -> *mut cef_print_settings_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_print_settings_t) -> Option<CefPrintSettings> {
    if c_object.is_null() {
      None
    } else {
      Some(CefPrintSettings::from_c_object_addref(c_object))
    }
  }
}

