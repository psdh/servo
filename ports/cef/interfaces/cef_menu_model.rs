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
// Supports creation and modification of menus. See cef_menu_id_t for the
// command ids that have default implementations. All user-defined command ids
// should be between MENU_ID_USER_FIRST and MENU_ID_USER_LAST. The functions of
// this structure can only be accessed on the browser process the UI thread.
//
#[repr(C)]
pub struct _cef_menu_model_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Clears the menu. Returns true (1) on success.
  //
  pub clear: Option<extern "C" fn(this: *mut cef_menu_model_t) -> libc::c_int>,

  //
  // Returns the number of items in this menu.
  //
  pub get_count: Option<extern "C" fn(
      this: *mut cef_menu_model_t) -> libc::c_int>,

  //
  // Add a separator to the menu. Returns true (1) on success.
  //
  pub add_separator: Option<extern "C" fn(
      this: *mut cef_menu_model_t) -> libc::c_int>,

  //
  // Add an item to the menu. Returns true (1) on success.
  //
  pub add_item: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int,
      label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Add a check item to the menu. Returns true (1) on success.
  //
  pub add_check_item: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int,
      label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Add a radio item to the menu. Only a single item with the specified
  // |group_id| can be checked at a time. Returns true (1) on success.
  //
  pub add_radio_item: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, label: *const types::cef_string_t,
      group_id: libc::c_int) -> libc::c_int>,

  //
  // Add a sub-menu to the menu. The new sub-menu is returned.
  //
  pub add_sub_menu: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int,
      label: *const types::cef_string_t) -> *mut interfaces::cef_menu_model_t>,

  //
  // Insert a separator in the menu at the specified |index|. Returns true (1)
  // on success.
  //
  pub insert_separator_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Insert an item in the menu at the specified |index|. Returns true (1) on
  // success.
  //
  pub insert_item_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, command_id: libc::c_int,
      label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Insert a check item in the menu at the specified |index|. Returns true (1)
  // on success.
  //
  pub insert_check_item_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, command_id: libc::c_int,
      label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Insert a radio item in the menu at the specified |index|. Only a single
  // item with the specified |group_id| can be checked at a time. Returns true
  // (1) on success.
  //
  pub insert_radio_item_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, command_id: libc::c_int,
      label: *const types::cef_string_t,
      group_id: libc::c_int) -> libc::c_int>,

  //
  // Insert a sub-menu in the menu at the specified |index|. The new sub-menu is
  // returned.
  //
  pub insert_sub_menu_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, command_id: libc::c_int,
      label: *const types::cef_string_t) -> *mut interfaces::cef_menu_model_t>,

  //
  // Removes the item with the specified |command_id|. Returns true (1) on
  // success.
  //
  pub remove: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Removes the item at the specified |index|. Returns true (1) on success.
  //
  pub remove_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Returns the index associated with the specified |command_id| or -1 if not
  // found due to the command id not existing in the menu.
  //
  pub get_index_of: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns the command id at the specified |index| or -1 if not found due to
  // invalid range or the index being a separator.
  //
  pub get_command_id_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Sets the command id at the specified |index|. Returns true (1) on success.
  //
  pub set_command_id_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns the label for the specified |command_id| or NULL if not found.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_label: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> types::cef_string_userfree_t>,

  //
  // Returns the label at the specified |index| or NULL if not found due to
  // invalid range or the index being a separator.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_label_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> types::cef_string_userfree_t>,

  //
  // Sets the label for the specified |command_id|. Returns true (1) on success.
  //
  pub set_label: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int,
      label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Set the label at the specified |index|. Returns true (1) on success.
  //
  pub set_label_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, label: *const types::cef_string_t) -> libc::c_int>,

  //
  // Returns the item type for the specified |command_id|.
  //
  pub get_type: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> types::cef_menu_item_type_t>,

  //
  // Returns the item type at the specified |index|.
  //
  pub get_type_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> types::cef_menu_item_type_t>,

  //
  // Returns the group id for the specified |command_id| or -1 if invalid.
  //
  pub get_group_id: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns the group id at the specified |index| or -1 if invalid.
  //
  pub get_group_id_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Sets the group id for the specified |command_id|. Returns true (1) on
  // success.
  //
  pub set_group_id: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, group_id: libc::c_int) -> libc::c_int>,

  //
  // Sets the group id at the specified |index|. Returns true (1) on success.
  //
  pub set_group_id_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, group_id: libc::c_int) -> libc::c_int>,

  //
  // Returns the submenu for the specified |command_id| or NULL if invalid.
  //
  pub get_sub_menu: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> *mut interfaces::cef_menu_model_t>,

  //
  // Returns the submenu at the specified |index| or NULL if invalid.
  //
  pub get_sub_menu_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> *mut interfaces::cef_menu_model_t>,

  //
  // Returns true (1) if the specified |command_id| is visible.
  //
  pub is_visible: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |index| is visible.
  //
  pub is_visible_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Change the visibility of the specified |command_id|. Returns true (1) on
  // success.
  //
  pub set_visible: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, visible: libc::c_int) -> libc::c_int>,

  //
  // Change the visibility at the specified |index|. Returns true (1) on
  // success.
  //
  pub set_visible_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, visible: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |command_id| is enabled.
  //
  pub is_enabled: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |index| is enabled.
  //
  pub is_enabled_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Change the enabled status of the specified |command_id|. Returns true (1)
  // on success.
  //
  pub set_enabled: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, enabled: libc::c_int) -> libc::c_int>,

  //
  // Change the enabled status at the specified |index|. Returns true (1) on
  // success.
  //
  pub set_enabled_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, enabled: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |command_id| is checked. Only applies to
  // check and radio items.
  //
  pub is_checked: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |index| is checked. Only applies to check
  // and radio items.
  //
  pub is_checked_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Check the specified |command_id|. Only applies to check and radio items.
  // Returns true (1) on success.
  //
  pub set_checked: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, checked: libc::c_int) -> libc::c_int>,

  //
  // Check the specified |index|. Only applies to check and radio items. Returns
  // true (1) on success.
  //
  pub set_checked_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, checked: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |command_id| has a keyboard accelerator
  // assigned.
  //
  pub has_accelerator: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Returns true (1) if the specified |index| has a keyboard accelerator
  // assigned.
  //
  pub has_accelerator_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Set the keyboard accelerator for the specified |command_id|. |key_code| can
  // be any virtual key or character value. Returns true (1) on success.
  //
  pub set_accelerator: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, key_code: libc::c_int,
      shift_pressed: libc::c_int, ctrl_pressed: libc::c_int,
      alt_pressed: libc::c_int) -> libc::c_int>,

  //
  // Set the keyboard accelerator at the specified |index|. |key_code| can be
  // any virtual key or character value. Returns true (1) on success.
  //
  pub set_accelerator_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, key_code: libc::c_int, shift_pressed: libc::c_int,
      ctrl_pressed: libc::c_int, alt_pressed: libc::c_int) -> libc::c_int>,

  //
  // Remove the keyboard accelerator for the specified |command_id|. Returns
  // true (1) on success.
  //
  pub remove_accelerator: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int) -> libc::c_int>,

  //
  // Remove the keyboard accelerator at the specified |index|. Returns true (1)
  // on success.
  //
  pub remove_accelerator_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int) -> libc::c_int>,

  //
  // Retrieves the keyboard accelerator for the specified |command_id|. Returns
  // true (1) on success.
  //
  pub get_accelerator: Option<extern "C" fn(this: *mut cef_menu_model_t,
      command_id: libc::c_int, key_code: *mut libc::c_int,
      shift_pressed: *mut libc::c_int, ctrl_pressed: *mut libc::c_int,
      alt_pressed: *mut libc::c_int) -> libc::c_int>,

  //
  // Retrieves the keyboard accelerator for the specified |index|. Returns true
  // (1) on success.
  //
  pub get_accelerator_at: Option<extern "C" fn(this: *mut cef_menu_model_t,
      index: libc::c_int, key_code: *mut libc::c_int,
      shift_pressed: *mut libc::c_int, ctrl_pressed: *mut libc::c_int,
      alt_pressed: *mut libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_menu_model_t = _cef_menu_model_t;


//
// Supports creation and modification of menus. See cef_menu_id_t for the
// command ids that have default implementations. All user-defined command ids
// should be between MENU_ID_USER_FIRST and MENU_ID_USER_LAST. The functions of
// this structure can only be accessed on the browser process the UI thread.
//
pub struct CefMenuModel {
  c_object: *mut cef_menu_model_t,
}

impl Clone for CefMenuModel {
  fn clone(&self) -> CefMenuModel{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefMenuModel {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefMenuModel {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefMenuModel {
  pub unsafe fn from_c_object(c_object: *mut cef_menu_model_t) -> CefMenuModel {
    CefMenuModel {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_menu_model_t) -> CefMenuModel {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefMenuModel {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_menu_model_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_menu_model_t {
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
  // Clears the menu. Returns true (1) on success.
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
  // Returns the number of items in this menu.
  //
  pub fn get_count(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Add a separator to the menu. Returns true (1) on success.
  //
  pub fn add_separator(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_separator.unwrap())(
          self.c_object))
    }
  }

  //
  // Add an item to the menu. Returns true (1) on success.
  //
  pub fn add_item(&self, command_id: libc::c_int,
      label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_item.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Add a check item to the menu. Returns true (1) on success.
  //
  pub fn add_check_item(&self, command_id: libc::c_int,
      label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_check_item.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Add a radio item to the menu. Only a single item with the specified
  // |group_id| can be checked at a time. Returns true (1) on success.
  //
  pub fn add_radio_item(&self, command_id: libc::c_int, label: &[u16],
      group_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_radio_item.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(label),
          CefWrap::to_c(group_id)))
    }
  }

  //
  // Add a sub-menu to the menu. The new sub-menu is returned.
  //
  pub fn add_sub_menu(&self, command_id: libc::c_int,
      label: &[u16]) -> interfaces::CefMenuModel {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).add_sub_menu.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Insert a separator in the menu at the specified |index|. Returns true (1)
  // on success.
  //
  pub fn insert_separator_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).insert_separator_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Insert an item in the menu at the specified |index|. Returns true (1) on
  // success.
  //
  pub fn insert_item_at(&self, index: libc::c_int, command_id: libc::c_int,
      label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).insert_item_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Insert a check item in the menu at the specified |index|. Returns true (1)
  // on success.
  //
  pub fn insert_check_item_at(&self, index: libc::c_int,
      command_id: libc::c_int, label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).insert_check_item_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Insert a radio item in the menu at the specified |index|. Only a single
  // item with the specified |group_id| can be checked at a time. Returns true
  // (1) on success.
  //
  pub fn insert_radio_item_at(&self, index: libc::c_int,
      command_id: libc::c_int, label: &[u16],
      group_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).insert_radio_item_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(command_id),
          CefWrap::to_c(label),
          CefWrap::to_c(group_id)))
    }
  }

  //
  // Insert a sub-menu in the menu at the specified |index|. The new sub-menu is
  // returned.
  //
  pub fn insert_sub_menu_at(&self, index: libc::c_int, command_id: libc::c_int,
      label: &[u16]) -> interfaces::CefMenuModel {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).insert_sub_menu_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Removes the item with the specified |command_id|. Returns true (1) on
  // success.
  //
  pub fn remove(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Removes the item at the specified |index|. Returns true (1) on success.
  //
  pub fn remove_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the index associated with the specified |command_id| or -1 if not
  // found due to the command id not existing in the menu.
  //
  pub fn get_index_of(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_index_of.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the command id at the specified |index| or -1 if not found due to
  // invalid range or the index being a separator.
  //
  pub fn get_command_id_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_command_id_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Sets the command id at the specified |index|. Returns true (1) on success.
  //
  pub fn set_command_id_at(&self, index: libc::c_int,
      command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_command_id_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the label for the specified |command_id| or NULL if not found.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_label(&self, command_id: libc::c_int) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_label.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the label at the specified |index| or NULL if not found due to
  // invalid range or the index being a separator.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_label_at(&self, index: libc::c_int) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_label_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Sets the label for the specified |command_id|. Returns true (1) on success.
  //
  pub fn set_label(&self, command_id: libc::c_int,
      label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_label.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(label)))
    }
  }

  //
  // Set the label at the specified |index|. Returns true (1) on success.
  //
  pub fn set_label_at(&self, index: libc::c_int, label: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_label_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(label)))
    }
  }

  //
  // Returns the item type for the specified |command_id|.
  //
  pub fn get_type(&self,
      command_id: libc::c_int) -> types::cef_menu_item_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the item type at the specified |index|.
  //
  pub fn get_type_at(&self, index: libc::c_int) -> types::cef_menu_item_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the group id for the specified |command_id| or -1 if invalid.
  //
  pub fn get_group_id(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_group_id.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the group id at the specified |index| or -1 if invalid.
  //
  pub fn get_group_id_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_group_id_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Sets the group id for the specified |command_id|. Returns true (1) on
  // success.
  //
  pub fn set_group_id(&self, command_id: libc::c_int,
      group_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_group_id.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(group_id)))
    }
  }

  //
  // Sets the group id at the specified |index|. Returns true (1) on success.
  //
  pub fn set_group_id_at(&self, index: libc::c_int,
      group_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_group_id_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(group_id)))
    }
  }

  //
  // Returns the submenu for the specified |command_id| or NULL if invalid.
  //
  pub fn get_sub_menu(&self,
      command_id: libc::c_int) -> interfaces::CefMenuModel {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_sub_menu.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns the submenu at the specified |index| or NULL if invalid.
  //
  pub fn get_sub_menu_at(&self,
      index: libc::c_int) -> interfaces::CefMenuModel {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_sub_menu_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns true (1) if the specified |command_id| is visible.
  //
  pub fn is_visible(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_visible.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns true (1) if the specified |index| is visible.
  //
  pub fn is_visible_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_visible_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Change the visibility of the specified |command_id|. Returns true (1) on
  // success.
  //
  pub fn set_visible(&self, command_id: libc::c_int,
      visible: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_visible.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(visible)))
    }
  }

  //
  // Change the visibility at the specified |index|. Returns true (1) on
  // success.
  //
  pub fn set_visible_at(&self, index: libc::c_int,
      visible: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_visible_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(visible)))
    }
  }

  //
  // Returns true (1) if the specified |command_id| is enabled.
  //
  pub fn is_enabled(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_enabled.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns true (1) if the specified |index| is enabled.
  //
  pub fn is_enabled_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_enabled_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Change the enabled status of the specified |command_id|. Returns true (1)
  // on success.
  //
  pub fn set_enabled(&self, command_id: libc::c_int,
      enabled: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_enabled.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(enabled)))
    }
  }

  //
  // Change the enabled status at the specified |index|. Returns true (1) on
  // success.
  //
  pub fn set_enabled_at(&self, index: libc::c_int,
      enabled: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_enabled_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(enabled)))
    }
  }

  //
  // Returns true (1) if the specified |command_id| is checked. Only applies to
  // check and radio items.
  //
  pub fn is_checked(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_checked.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns true (1) if the specified |index| is checked. Only applies to check
  // and radio items.
  //
  pub fn is_checked_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_checked_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Check the specified |command_id|. Only applies to check and radio items.
  // Returns true (1) on success.
  //
  pub fn set_checked(&self, command_id: libc::c_int,
      checked: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_checked.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(checked)))
    }
  }

  //
  // Check the specified |index|. Only applies to check and radio items. Returns
  // true (1) on success.
  //
  pub fn set_checked_at(&self, index: libc::c_int,
      checked: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_checked_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(checked)))
    }
  }

  //
  // Returns true (1) if the specified |command_id| has a keyboard accelerator
  // assigned.
  //
  pub fn has_accelerator(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_accelerator.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Returns true (1) if the specified |index| has a keyboard accelerator
  // assigned.
  //
  pub fn has_accelerator_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_accelerator_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Set the keyboard accelerator for the specified |command_id|. |key_code| can
  // be any virtual key or character value. Returns true (1) on success.
  //
  pub fn set_accelerator(&self, command_id: libc::c_int, key_code: libc::c_int,
      shift_pressed: libc::c_int, ctrl_pressed: libc::c_int,
      alt_pressed: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_accelerator.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(key_code),
          CefWrap::to_c(shift_pressed),
          CefWrap::to_c(ctrl_pressed),
          CefWrap::to_c(alt_pressed)))
    }
  }

  //
  // Set the keyboard accelerator at the specified |index|. |key_code| can be
  // any virtual key or character value. Returns true (1) on success.
  //
  pub fn set_accelerator_at(&self, index: libc::c_int, key_code: libc::c_int,
      shift_pressed: libc::c_int, ctrl_pressed: libc::c_int,
      alt_pressed: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).set_accelerator_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(key_code),
          CefWrap::to_c(shift_pressed),
          CefWrap::to_c(ctrl_pressed),
          CefWrap::to_c(alt_pressed)))
    }
  }

  //
  // Remove the keyboard accelerator for the specified |command_id|. Returns
  // true (1) on success.
  //
  pub fn remove_accelerator(&self, command_id: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove_accelerator.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id)))
    }
  }

  //
  // Remove the keyboard accelerator at the specified |index|. Returns true (1)
  // on success.
  //
  pub fn remove_accelerator_at(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).remove_accelerator_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Retrieves the keyboard accelerator for the specified |command_id|. Returns
  // true (1) on success.
  //
  pub fn get_accelerator(&self, command_id: libc::c_int,
      key_code: &mut libc::c_int, shift_pressed: &mut libc::c_int,
      ctrl_pressed: &mut libc::c_int,
      alt_pressed: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_accelerator.unwrap())(
          self.c_object,
          CefWrap::to_c(command_id),
          CefWrap::to_c(key_code),
          CefWrap::to_c(shift_pressed),
          CefWrap::to_c(ctrl_pressed),
          CefWrap::to_c(alt_pressed)))
    }
  }

  //
  // Retrieves the keyboard accelerator for the specified |index|. Returns true
  // (1) on success.
  //
  pub fn get_accelerator_at(&self, index: libc::c_int,
      key_code: &mut libc::c_int, shift_pressed: &mut libc::c_int,
      ctrl_pressed: &mut libc::c_int,
      alt_pressed: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_accelerator_at.unwrap())(
          self.c_object,
          CefWrap::to_c(index),
          CefWrap::to_c(key_code),
          CefWrap::to_c(shift_pressed),
          CefWrap::to_c(ctrl_pressed),
          CefWrap::to_c(alt_pressed)))
    }
  }
}

impl CefWrap<*mut cef_menu_model_t> for CefMenuModel {
  fn to_c(rust_object: CefMenuModel) -> *mut cef_menu_model_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_menu_model_t) -> CefMenuModel {
    CefMenuModel::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_menu_model_t> for Option<CefMenuModel> {
  fn to_c(rust_object: Option<CefMenuModel>) -> *mut cef_menu_model_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_menu_model_t) -> Option<CefMenuModel> {
    if c_object.is_null() {
      None
    } else {
      Some(CefMenuModel::from_c_object_addref(c_object))
    }
  }
}

