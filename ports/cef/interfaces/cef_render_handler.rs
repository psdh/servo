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
// Implement this structure to handle events when window rendering is disabled.
// The functions of this structure will be called on the UI thread.
//
#[repr(C)]
pub struct _cef_render_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called to retrieve the root window rectangle in screen coordinates. Return
  // true (1) if the rectangle was provided.
  //
  pub get_root_screen_rect: Option<extern "C" fn(
      this: *mut cef_render_handler_t, browser: *mut interfaces::cef_browser_t,
      rect: *mut types::cef_rect_t) -> libc::c_int>,

  //
  // Called to retrieve the view rectangle which is relative to screen
  // coordinates. Return true (1) if the rectangle was provided.
  //
  pub get_view_rect: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      rect: *mut types::cef_rect_t) -> libc::c_int>,

  //
  // Called to retrieve the translation from view coordinates to actual screen
  // coordinates. Return true (1) if the screen coordinates were provided.
  //
  pub get_screen_point: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t, viewX: libc::c_int,
      viewY: libc::c_int, screenX: *mut libc::c_int,
      screenY: *mut libc::c_int) -> libc::c_int>,

  //
  // Called to allow the client to fill in the CefScreenInfo object with
  // appropriate values. Return true (1) if the |screen_info| structure has been
  // modified.
  //
  // If the screen info rectangle is left NULL the rectangle from GetViewRect
  // will be used. If the rectangle is still NULL or invalid popups may not be
  // drawn correctly.
  //
  pub get_screen_info: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      screen_info: *mut interfaces::cef_screen_info_t) -> libc::c_int>,

  //
  // Called when the browser wants to show or hide the popup widget. The popup
  // should be shown if |show| is true (1) and hidden if |show| is false (0).
  //
  pub on_popup_show: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t, show: libc::c_int) -> ()>,

  //
  // Called when the browser wants to move or resize the popup widget. |rect|
  // contains the new location and size.
  //
  pub on_popup_size: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      rect: *const types::cef_rect_t) -> ()>,

  //
  // Called when an element should be painted. |type| indicates whether the
  // element is the view or the popup widget. |buffer| contains the pixel data
  // for the whole image. |dirtyRects| contains the set of rectangles that need
  // to be repainted. On Windows |buffer| will be |width|*|height|*4 bytes in
  // size and represents a BGRA image with an upper-left origin.
  //
  pub on_paint: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      ty: types::cef_paint_element_type_t, dirtyRects_count: libc::size_t,
      dirtyRects: *const types::cef_rect_t, buffer: *const (),
      width: libc::c_int, height: libc::c_int) -> ()>,

  //
  // Called when the browser window's cursor has changed.
  //
  pub on_cursor_change: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      cursor: types::cef_cursor_handle_t) -> ()>,

  //
  // Called when the user starts dragging content in the web view. Contextual
  // information about the dragged content is supplied by |drag_data|. OS APIs
  // that run a system message loop may be used within the StartDragging call.
  //
  // Return false (0) to abort the drag operation. Don't call any of
  // cef_browser_host_t::DragSource*Ended* functions after returning false (0).
  //
  // Return true (1) to handle the drag operation. Call
  // cef_browser_host_t::DragSourceEndedAt and DragSourceSystemDragEnded either
  // synchronously or asynchronously to inform the web view that the drag
  // operation has ended.
  //
  pub start_dragging: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      drag_data: *mut interfaces::cef_drag_data_t,
      allowed_ops: types::cef_drag_operations_mask_t, x: libc::c_int,
      y: libc::c_int) -> libc::c_int>,

  //
  // Called when the web view wants to update the mouse cursor during a drag &
  // drop operation. |operation| describes the allowed operation (none, move,
  // copy, link).
  //
  pub update_drag_cursor: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      operation: types::cef_drag_operations_mask_t) -> ()>,

  //
  // Called when the scroll offset has changed.
  //
  pub on_scroll_offset_changed: Option<extern "C" fn(
      this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // Called to retrieve the backing size of the view rectangle which is relative
  // to screen coordinates. On HiDPI displays, the backing size can differ from
  // the view size as returned by |GetViewRect|. Return true (1) if the
  // rectangle was provided.
  //
  pub get_backing_rect: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t,
      rect: *mut types::cef_rect_t) -> libc::c_int>,

  //
  // Called when an element should be presented (e.g. double buffers should page
  // flip). This is called only during accelerated compositing.
  //
  pub on_present: Option<extern "C" fn(this: *mut cef_render_handler_t,
      browser: *mut interfaces::cef_browser_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_render_handler_t = _cef_render_handler_t;


//
// Implement this structure to handle events when window rendering is disabled.
// The functions of this structure will be called on the UI thread.
//
pub struct CefRenderHandler {
  c_object: *mut cef_render_handler_t,
}

impl Clone for CefRenderHandler {
  fn clone(&self) -> CefRenderHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefRenderHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefRenderHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefRenderHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_render_handler_t) -> CefRenderHandler {
    CefRenderHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_render_handler_t) -> CefRenderHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefRenderHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_render_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_render_handler_t {
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
  // Called to retrieve the root window rectangle in screen coordinates. Return
  // true (1) if the rectangle was provided.
  //
  pub fn get_root_screen_rect(&self, browser: interfaces::CefBrowser,
      rect: &mut types::cef_rect_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_root_screen_rect.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(rect)))
    }
  }

  //
  // Called to retrieve the view rectangle which is relative to screen
  // coordinates. Return true (1) if the rectangle was provided.
  //
  pub fn get_view_rect(&self, browser: interfaces::CefBrowser,
      rect: &mut types::cef_rect_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_view_rect.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(rect)))
    }
  }

  //
  // Called to retrieve the translation from view coordinates to actual screen
  // coordinates. Return true (1) if the screen coordinates were provided.
  //
  pub fn get_screen_point(&self, browser: interfaces::CefBrowser,
      viewX: libc::c_int, viewY: libc::c_int, screenX: &mut libc::c_int,
      screenY: &mut libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_screen_point.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(viewX),
          CefWrap::to_c(viewY),
          CefWrap::to_c(screenX),
          CefWrap::to_c(screenY)))
    }
  }

  //
  // Called to allow the client to fill in the CefScreenInfo object with
  // appropriate values. Return true (1) if the |screen_info| structure has been
  // modified.
  //
  // If the screen info rectangle is left NULL the rectangle from GetViewRect
  // will be used. If the rectangle is still NULL or invalid popups may not be
  // drawn correctly.
  //
  pub fn get_screen_info(&self, browser: interfaces::CefBrowser,
      screen_info: &mut interfaces::CefScreenInfo) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_screen_info.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(screen_info)))
    }
  }

  //
  // Called when the browser wants to show or hide the popup widget. The popup
  // should be shown if |show| is true (1) and hidden if |show| is false (0).
  //
  pub fn on_popup_show(&self, browser: interfaces::CefBrowser,
      show: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_popup_show.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(show)))
    }
  }

  //
  // Called when the browser wants to move or resize the popup widget. |rect|
  // contains the new location and size.
  //
  pub fn on_popup_size(&self, browser: interfaces::CefBrowser,
      rect: &types::cef_rect_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_popup_size.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(rect)))
    }
  }

  //
  // Called when an element should be painted. |type| indicates whether the
  // element is the view or the popup widget. |buffer| contains the pixel data
  // for the whole image. |dirtyRects| contains the set of rectangles that need
  // to be repainted. On Windows |buffer| will be |width|*|height|*4 bytes in
  // size and represents a BGRA image with an upper-left origin.
  //
  pub fn on_paint(&self, browser: interfaces::CefBrowser,
      ty: types::cef_paint_element_type_t, dirtyRects_count: libc::size_t,
      dirtyRects: *const types::cef_rect_t, buffer: &(), width: libc::c_int,
      height: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_paint.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(ty),
          CefWrap::to_c(dirtyRects_count),
          CefWrap::to_c(dirtyRects),
          CefWrap::to_c(buffer),
          CefWrap::to_c(width),
          CefWrap::to_c(height)))
    }
  }

  //
  // Called when the browser window's cursor has changed.
  //
  pub fn on_cursor_change(&self, browser: interfaces::CefBrowser,
      cursor: types::cef_cursor_handle_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_cursor_change.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(cursor)))
    }
  }

  //
  // Called when the user starts dragging content in the web view. Contextual
  // information about the dragged content is supplied by |drag_data|. OS APIs
  // that run a system message loop may be used within the StartDragging call.
  //
  // Return false (0) to abort the drag operation. Don't call any of
  // cef_browser_host_t::DragSource*Ended* functions after returning false (0).
  //
  // Return true (1) to handle the drag operation. Call
  // cef_browser_host_t::DragSourceEndedAt and DragSourceSystemDragEnded either
  // synchronously or asynchronously to inform the web view that the drag
  // operation has ended.
  //
  pub fn start_dragging(&self, browser: interfaces::CefBrowser,
      drag_data: interfaces::CefDragData,
      allowed_ops: types::cef_drag_operations_mask_t, x: libc::c_int,
      y: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).start_dragging.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(drag_data),
          CefWrap::to_c(allowed_ops),
          CefWrap::to_c(x),
          CefWrap::to_c(y)))
    }
  }

  //
  // Called when the web view wants to update the mouse cursor during a drag &
  // drop operation. |operation| describes the allowed operation (none, move,
  // copy, link).
  //
  pub fn update_drag_cursor(&self, browser: interfaces::CefBrowser,
      operation: types::cef_drag_operations_mask_t) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).update_drag_cursor.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(operation)))
    }
  }

  //
  // Called when the scroll offset has changed.
  //
  pub fn on_scroll_offset_changed(&self, browser: interfaces::CefBrowser) -> (
      ) {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_scroll_offset_changed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }

  //
  // Called to retrieve the backing size of the view rectangle which is relative
  // to screen coordinates. On HiDPI displays, the backing size can differ from
  // the view size as returned by |GetViewRect|. Return true (1) if the
  // rectangle was provided.
  //
  pub fn get_backing_rect(&self, browser: interfaces::CefBrowser,
      rect: &mut types::cef_rect_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_backing_rect.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(rect)))
    }
  }

  //
  // Called when an element should be presented (e.g. double buffers should page
  // flip). This is called only during accelerated compositing.
  //
  pub fn on_present(&self, browser: interfaces::CefBrowser) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_present.unwrap())(
          self.c_object,
          CefWrap::to_c(browser)))
    }
  }
}

impl CefWrap<*mut cef_render_handler_t> for CefRenderHandler {
  fn to_c(rust_object: CefRenderHandler) -> *mut cef_render_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_render_handler_t) -> CefRenderHandler {
    CefRenderHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_render_handler_t> for Option<CefRenderHandler> {
  fn to_c(rust_object: Option<CefRenderHandler>) -> *mut cef_render_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_render_handler_t) -> Option<CefRenderHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefRenderHandler::from_c_object_addref(c_object))
    }
  }
}

