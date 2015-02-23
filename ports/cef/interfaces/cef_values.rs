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
// Structure representing a binary value. Can be used on any process and thread.
//
#[repr(C)]
pub struct _cef_binary_value_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_binary_value_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is currently owned by another object.
  //
  pub is_owned: Option<extern "C" fn(
      this: *mut cef_binary_value_t) -> libc::c_int>,

  //
  // Returns a copy of this object. The data in this object will also be copied.
  //
  pub copy: Option<extern "C" fn(
      this: *mut cef_binary_value_t) -> *mut interfaces::cef_binary_value_t>,

  //
  // Returns the data size.
  //
  pub get_size: Option<extern "C" fn(
      this: *mut cef_binary_value_t) -> libc::size_t>,

  //
  // Read up to |buffer_size| number of bytes into |buffer|. Reading begins at
  // the specified byte |data_offset|. Returns the number of bytes read.
  //
  pub get_data: Option<extern "C" fn(this: *mut cef_binary_value_t,
      buffer: *mut (), buffer_size: libc::size_t,
      data_offset: libc::size_t) -> libc::size_t>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_binary_value_t = _cef_binary_value_t;


//
// Structure representing a binary value. Can be used on any process and thread.
//
pub struct CefBinaryValue {
  c_object: *mut cef_binary_value_t,
}

impl Clone for CefBinaryValue {
  fn clone(&self) -> CefBinaryValue{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefBinaryValue {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefBinaryValue {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefBinaryValue {
  pub unsafe fn from_c_object(c_object: *mut cef_binary_value_t) -> CefBinaryValue {
    CefBinaryValue {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_binary_value_t) -> CefBinaryValue {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefBinaryValue {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_binary_value_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_binary_value_t {
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
  // Returns true (1) if this object is currently owned by another object.
  //
  pub fn is_owned(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_owned.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns a copy of this object. The data in this object will also be copied.
  //
  pub fn copy(&self) -> interfaces::CefBinaryValue {
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
  // Returns the data size.
  //
  pub fn get_size(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_size.unwrap())(
          self.c_object))
    }
  }

  //
  // Read up to |buffer_size| number of bytes into |buffer|. Reading begins at
  // the specified byte |data_offset|. Returns the number of bytes read.
  //
  pub fn get_data(&self, buffer: &mut (), buffer_size: libc::size_t,
      data_offset: libc::size_t) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_data.unwrap())(
          self.c_object,
          CefWrap::to_c(buffer),
          CefWrap::to_c(buffer_size),
          CefWrap::to_c(data_offset)))
    }
  }

  //
  // Creates a new object that is not owned by any other object. The specified
  // |data| will be copied.
  //
  pub fn create(data: &(),
      data_size: libc::size_t) -> interfaces::CefBinaryValue {
    unsafe {
      CefWrap::to_rust(
        ::values::cef_binary_value_create(
          CefWrap::to_c(data),
          CefWrap::to_c(data_size)))
    }
  }
}

impl CefWrap<*mut cef_binary_value_t> for CefBinaryValue {
  fn to_c(rust_object: CefBinaryValue) -> *mut cef_binary_value_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_binary_value_t) -> CefBinaryValue {
    CefBinaryValue::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_binary_value_t> for Option<CefBinaryValue> {
  fn to_c(rust_object: Option<CefBinaryValue>) -> *mut cef_binary_value_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_binary_value_t) -> Option<CefBinaryValue> {
    if c_object.is_null() {
      None
    } else {
      Some(CefBinaryValue::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a dictionary value. Can be used on any process and
// thread.
//
#[repr(C)]
pub struct _cef_dictionary_value_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_dictionary_value_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is currently owned by another object.
  //
  pub is_owned: Option<extern "C" fn(
      this: *mut cef_dictionary_value_t) -> libc::c_int>,

  //
  // Returns true (1) if the values of this object are read-only. Some APIs may
  // expose read-only objects.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_dictionary_value_t) -> libc::c_int>,

  //
  // Returns a writable copy of this object. If |exclude_NULL_children| is true
  // (1) any NULL dictionaries or lists will be excluded from the copy.
  //
  pub copy: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      exclude_empty_children: libc::c_int) -> *mut interfaces::cef_dictionary_value_t>,

  //
  // Returns the number of values.
  //
  pub get_size: Option<extern "C" fn(
      this: *mut cef_dictionary_value_t) -> libc::size_t>,

  //
  // Removes all values. Returns true (1) on success.
  //
  pub clear: Option<extern "C" fn(
      this: *mut cef_dictionary_value_t) -> libc::c_int>,

  //
  // Returns true (1) if the current dictionary has a value for the given key.
  //
  pub has_key: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Reads all keys for this dictionary into the specified vector.
  //
  pub get_keys: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      keys: types::cef_string_list_t) -> libc::c_int>,

  //
  // Removes the value at the specified key. Returns true (1) is the value was
  // removed successfully.
  //
  pub remove: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Returns the value type for the specified key.
  //
  pub get_type: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> interfaces::cef_value_type_t>,

  //
  // Returns the value at the specified key as type bool.
  //
  pub get_bool: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Returns the value at the specified key as type int.
  //
  pub get_int: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Returns the value at the specified key as type double.
  //
  pub get_double: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_double>,

  //
  // Returns the value at the specified key as type string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_string: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> types::cef_string_userfree_t>,

  //
  // Returns the value at the specified key as type binary.
  //
  pub get_binary: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> *mut interfaces::cef_binary_value_t>,

  //
  // Returns the value at the specified key as type dictionary.
  //
  pub get_dictionary: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> *mut interfaces::cef_dictionary_value_t>,

  //
  // Returns the value at the specified key as type list.
  //
  pub get_list: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> *mut interfaces::cef_list_value_t>,

  //
  // Sets the value at the specified key as type null. Returns true (1) if the
  // value was set successfully.
  //
  pub set_null: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t) -> libc::c_int>,

  //
  // Sets the value at the specified key as type bool. Returns true (1) if the
  // value was set successfully.
  //
  pub set_bool: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t, value: libc::c_int) -> libc::c_int>,

  //
  // Sets the value at the specified key as type int. Returns true (1) if the
  // value was set successfully.
  //
  pub set_int: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t, value: libc::c_int) -> libc::c_int>,

  //
  // Sets the value at the specified key as type double. Returns true (1) if the
  // value was set successfully.
  //
  pub set_double: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t, value: libc::c_double) -> libc::c_int>,

  //
  // Sets the value at the specified key as type string. Returns true (1) if the
  // value was set successfully.
  //
  pub set_string: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t,
      value: *const types::cef_string_t) -> libc::c_int>,

  //
  // Sets the value at the specified key as type binary. Returns true (1) if the
  // value was set successfully. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub set_binary: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t,
      value: *mut interfaces::cef_binary_value_t) -> libc::c_int>,

  //
  // Sets the value at the specified key as type dict. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub set_dictionary: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t,
      value: *mut interfaces::cef_dictionary_value_t) -> libc::c_int>,

  //
  // Sets the value at the specified key as type list. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub set_list: Option<extern "C" fn(this: *mut cef_dictionary_value_t,
      key: *const types::cef_string_t,
      value: *mut interfaces::cef_list_value_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_dictionary_value_t = _cef_dictionary_value_t;


//
// Structure representing a dictionary value. Can be used on any process and
// thread.
//
pub struct CefDictionaryValue {
  c_object: *mut cef_dictionary_value_t,
}

impl Clone for CefDictionaryValue {
  fn clone(&self) -> CefDictionaryValue{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDictionaryValue {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDictionaryValue {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDictionaryValue {
  pub unsafe fn from_c_object(c_object: *mut cef_dictionary_value_t) -> CefDictionaryValue {
    CefDictionaryValue {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_dictionary_value_t) -> CefDictionaryValue {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDictionaryValue {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_dictionary_value_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_dictionary_value_t {
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
  // Returns true (1) if this object is currently owned by another object.
  //
  pub fn is_owned(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_owned.unwrap())(
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
  // Returns a writable copy of this object. If |exclude_NULL_children| is true
  // (1) any NULL dictionaries or lists will be excluded from the copy.
  //
  pub fn copy(&self,
      exclude_empty_children: libc::c_int) -> interfaces::CefDictionaryValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).copy.unwrap())(
          self.c_object,
          CefWrap::to_c(exclude_empty_children)))
    }
  }

  //
  // Returns the number of values.
  //
  pub fn get_size(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_size.unwrap())(
          self.c_object))
    }
  }

  //
  // Removes all values. Returns true (1) on success.
  //
  pub fn clear(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).clear.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the current dictionary has a value for the given key.
  //
  pub fn has_key(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_key.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Reads all keys for this dictionary into the specified vector.
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
  // Removes the value at the specified key. Returns true (1) is the value was
  // removed successfully.
  //
  pub fn remove(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value type for the specified key.
  //
  pub fn get_type(&self, key: &[u16]) -> interfaces::CefValueType {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type bool.
  //
  pub fn get_bool(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_bool.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type int.
  //
  pub fn get_int(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_int.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type double.
  //
  pub fn get_double(&self, key: &[u16]) -> libc::c_double {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_double.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_string(&self, key: &[u16]) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_string.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type binary.
  //
  pub fn get_binary(&self, key: &[u16]) -> interfaces::CefBinaryValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_binary.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type dictionary.
  //
  pub fn get_dictionary(&self, key: &[u16]) -> interfaces::CefDictionaryValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_dictionary.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Returns the value at the specified key as type list.
  //
  pub fn get_list(&self, key: &[u16]) -> interfaces::CefListValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_list.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Sets the value at the specified key as type null. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_null(&self, key: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_null.unwrap())(
          self.c_object,
          CefWrap::to_c(key)))
    }
  }

  //
  // Sets the value at the specified key as type bool. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_bool(&self, key: &[u16], value: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_bool.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type int. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_int(&self, key: &[u16], value: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_int.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type double. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_double(&self, key: &[u16], value: libc::c_double) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_double.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type string. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_string(&self, key: &[u16], value: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_string.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type binary. Returns true (1) if the
  // value was set successfully. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub fn set_binary(&self, key: &[u16],
      value: interfaces::CefBinaryValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_binary.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type dict. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub fn set_dictionary(&self, key: &[u16],
      value: interfaces::CefDictionaryValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_dictionary.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified key as type list. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub fn set_list(&self, key: &[u16],
      value: interfaces::CefListValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_list.unwrap())(
          self.c_object,
          CefWrap::to_c(key),
          CefWrap::to_c(value)))
    }
  }

  //
  // Creates a new object that is not owned by any other object.
  //
  pub fn create() -> interfaces::CefDictionaryValue {
    unsafe {
      CefWrap::to_rust(
        ::values::cef_dictionary_value_create(
))
    }
  }
}

impl CefWrap<*mut cef_dictionary_value_t> for CefDictionaryValue {
  fn to_c(rust_object: CefDictionaryValue) -> *mut cef_dictionary_value_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_dictionary_value_t) -> CefDictionaryValue {
    CefDictionaryValue::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_dictionary_value_t> for Option<CefDictionaryValue> {
  fn to_c(rust_object: Option<CefDictionaryValue>) -> *mut cef_dictionary_value_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_dictionary_value_t) -> Option<CefDictionaryValue> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDictionaryValue::from_c_object_addref(c_object))
    }
  }
}


//
// Structure representing a list value. Can be used on any process and thread.
//
#[repr(C)]
pub struct _cef_list_value_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is valid. Do not call any other functions
  // if this function returns false (0).
  //
  pub is_valid: Option<extern "C" fn(
      this: *mut cef_list_value_t) -> libc::c_int>,

  //
  // Returns true (1) if this object is currently owned by another object.
  //
  pub is_owned: Option<extern "C" fn(
      this: *mut cef_list_value_t) -> libc::c_int>,

  //
  // Returns true (1) if the values of this object are read-only. Some APIs may
  // expose read-only objects.
  //
  pub is_read_only: Option<extern "C" fn(
      this: *mut cef_list_value_t) -> libc::c_int>,

  //
  // Returns a writable copy of this object.
  //
  pub copy: Option<extern "C" fn(
      this: *mut cef_list_value_t) -> *mut interfaces::cef_list_value_t>,

  //
  // Sets the number of values. If the number of values is expanded all new
  // value slots will default to type null. Returns true (1) on success.
  //
  pub set_size: Option<extern "C" fn(this: *mut cef_list_value_t,
      size: libc::size_t) -> libc::c_int>,

  //
  // Returns the number of values.
  //
  pub get_size: Option<extern "C" fn(
      this: *mut cef_list_value_t) -> libc::size_t>,

  //
  // Removes all values. Returns true (1) on success.
  //
  pub clear: Option<extern "C" fn(this: *mut cef_list_value_t) -> libc::c_int>,

  //
  // Removes the value at the specified index.
  //
  pub remove: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Returns the value type at the specified index.
  //
  pub get_type: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> interfaces::cef_value_type_t>,

  //
  // Returns the value at the specified index as type bool.
  //
  pub get_bool: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Returns the value at the specified index as type int.
  //
  pub get_int: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Returns the value at the specified index as type double.
  //
  pub get_double: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> libc::c_double>,

  //
  // Returns the value at the specified index as type string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_string: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> types::cef_string_userfree_t>,

  //
  // Returns the value at the specified index as type binary.
  //
  pub get_binary: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> *mut interfaces::cef_binary_value_t>,

  //
  // Returns the value at the specified index as type dictionary.
  //
  pub get_dictionary: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> *mut interfaces::cef_dictionary_value_t>,

  //
  // Returns the value at the specified index as type list.
  //
  pub get_list: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> *mut interfaces::cef_list_value_t>,

  //
  // Sets the value at the specified index as type null. Returns true (1) if the
  // value was set successfully.
  //
  pub set_null: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Sets the value at the specified index as type bool. Returns true (1) if the
  // value was set successfully.
  //
  pub set_bool: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int, value: libc::c_int) -> libc::c_int>,

  //
  // Sets the value at the specified index as type int. Returns true (1) if the
  // value was set successfully.
  //
  pub set_int: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int, value: libc::c_int) -> libc::c_int>,

  //
  // Sets the value at the specified index as type double. Returns true (1) if
  // the value was set successfully.
  //
  pub set_double: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int, value: libc::c_double) -> libc::c_int>,

  //
  // Sets the value at the specified index as type string. Returns true (1) if
  // the value was set successfully.
  //
  pub set_string: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int, value: *const types::cef_string_t) -> libc::c_int>,

  //
  // Sets the value at the specified index as type binary. Returns true (1) if
  // the value was set successfully. After calling this function the |value|
  // object will no longer be valid. If |value| is currently owned by another
  // object then the value will be copied and the |value| reference will not
  // change. Otherwise, ownership will be transferred to this object and the
  // |value| reference will be invalidated.
  //
  pub set_binary: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int,
      value: *mut interfaces::cef_binary_value_t) -> libc::c_int>,

  //
  // Sets the value at the specified index as type dict. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub set_dictionary: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int,
      value: *mut interfaces::cef_dictionary_value_t) -> libc::c_int>,

  //
  // Sets the value at the specified index as type list. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub set_list: Option<extern "C" fn(this: *mut cef_list_value_t,
      index: libc::c_int,
      value: *mut interfaces::cef_list_value_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_list_value_t = _cef_list_value_t;


//
// Structure representing a list value. Can be used on any process and thread.
//
pub struct CefListValue {
  c_object: *mut cef_list_value_t,
}

impl Clone for CefListValue {
  fn clone(&self) -> CefListValue{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefListValue {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefListValue {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefListValue {
  pub unsafe fn from_c_object(c_object: *mut cef_list_value_t) -> CefListValue {
    CefListValue {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_list_value_t) -> CefListValue {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefListValue {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_list_value_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_list_value_t {
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
  // Returns true (1) if this object is currently owned by another object.
  //
  pub fn is_owned(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_owned.unwrap())(
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
  pub fn copy(&self) -> interfaces::CefListValue {
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
  // Sets the number of values. If the number of values is expanded all new
  // value slots will default to type null. Returns true (1) on success.
  //
  pub fn set_size(&self, size: libc::size_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_size.unwrap())(
          self.c_object,
          CefWrap::to_c(size)))
    }
  }

  //
  // Returns the number of values.
  //
  pub fn get_size(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_size.unwrap())(
          self.c_object))
    }
  }

  //
  // Removes all values. Returns true (1) on success.
  //
  pub fn clear(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).clear.unwrap())(
          self.c_object))
    }
  }

  //
  // Removes the value at the specified index.
  //
  pub fn remove(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value type at the specified index.
  //
  pub fn get_type(&self, index: libc::c_int) -> interfaces::CefValueType {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type bool.
  //
  pub fn get_bool(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_bool.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type int.
  //
  pub fn get_int(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_int.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type double.
  //
  pub fn get_double(&self, index: libc::c_int) -> libc::c_double {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_double.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_string(&self, index: libc::c_int) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_string.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type binary.
  //
  pub fn get_binary(&self, index: libc::c_int) -> interfaces::CefBinaryValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_binary.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type dictionary.
  //
  pub fn get_dictionary(&self,
      index: libc::c_int) -> interfaces::CefDictionaryValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_dictionary.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value at the specified index as type list.
  //
  pub fn get_list(&self, index: libc::c_int) -> interfaces::CefListValue {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_list.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Sets the value at the specified index as type null. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_null(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_null.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Sets the value at the specified index as type bool. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_bool(&self, index: libc::c_int,
      value: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_bool.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type int. Returns true (1) if the
  // value was set successfully.
  //
  pub fn set_int(&self, index: libc::c_int, value: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_int.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type double. Returns true (1) if
  // the value was set successfully.
  //
  pub fn set_double(&self, index: libc::c_int,
      value: libc::c_double) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_double.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type string. Returns true (1) if
  // the value was set successfully.
  //
  pub fn set_string(&self, index: libc::c_int, value: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_string.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type binary. Returns true (1) if
  // the value was set successfully. After calling this function the |value|
  // object will no longer be valid. If |value| is currently owned by another
  // object then the value will be copied and the |value| reference will not
  // change. Otherwise, ownership will be transferred to this object and the
  // |value| reference will be invalidated.
  //
  pub fn set_binary(&self, index: libc::c_int,
      value: interfaces::CefBinaryValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_binary.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type dict. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub fn set_dictionary(&self, index: libc::c_int,
      value: interfaces::CefDictionaryValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_dictionary.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Sets the value at the specified index as type list. Returns true (1) if the
  // value was set successfully. After calling this function the |value| object
  // will no longer be valid. If |value| is currently owned by another object
  // then the value will be copied and the |value| reference will not change.
  // Otherwise, ownership will be transferred to this object and the |value|
  // reference will be invalidated.
  //
  pub fn set_list(&self, index: libc::c_int,
      value: interfaces::CefListValue) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_list.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(value)))
    }
  }

  //
  // Creates a new object that is not owned by any other object.
  //
  pub fn create() -> interfaces::CefListValue {
    unsafe {
      CefWrap::to_rust(
        ::values::cef_list_value_create(
))
    }
  }
}

impl CefWrap<*mut cef_list_value_t> for CefListValue {
  fn to_c(rust_object: CefListValue) -> *mut cef_list_value_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_list_value_t) -> CefListValue {
    CefListValue::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_list_value_t> for Option<CefListValue> {
  fn to_c(rust_object: Option<CefListValue>) -> *mut cef_list_value_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_list_value_t) -> Option<CefListValue> {
    if c_object.is_null() {
      None
    } else {
      Some(CefListValue::from_c_object_addref(c_object))
    }
  }
}

