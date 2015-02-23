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
// Implement this structure for asynchronous task execution. If the task is
// posted successfully and if the associated message loop is still running then
// the execute() function will be called on the target thread. If the task fails
// to post then the task object may be destroyed on the source thread instead of
// the target thread. For this reason be cautious when performing work in the
// task object destructor.
//
#[repr(C)]
pub struct _cef_task_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Method that will be executed on the target thread.
  //
  pub execute: Option<extern "C" fn(this: *mut cef_task_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_task_t = _cef_task_t;


//
// Implement this structure for asynchronous task execution. If the task is
// posted successfully and if the associated message loop is still running then
// the execute() function will be called on the target thread. If the task fails
// to post then the task object may be destroyed on the source thread instead of
// the target thread. For this reason be cautious when performing work in the
// task object destructor.
//
pub struct CefTask {
  c_object: *mut cef_task_t,
}

impl Clone for CefTask {
  fn clone(&self) -> CefTask{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefTask {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefTask {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefTask {
  pub unsafe fn from_c_object(c_object: *mut cef_task_t) -> CefTask {
    CefTask {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_task_t) -> CefTask {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefTask {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_task_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_task_t {
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
  // Method that will be executed on the target thread.
  //
  pub fn execute(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).execute.unwrap())(
          self.c_object))
    }
  }
}

impl CefWrap<*mut cef_task_t> for CefTask {
  fn to_c(rust_object: CefTask) -> *mut cef_task_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_task_t) -> CefTask {
    CefTask::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_task_t> for Option<CefTask> {
  fn to_c(rust_object: Option<CefTask>) -> *mut cef_task_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_task_t) -> Option<CefTask> {
    if c_object.is_null() {
      None
    } else {
      Some(CefTask::from_c_object_addref(c_object))
    }
  }
}


//
// Structure that asynchronously executes tasks on the associated thread. It is
// safe to call the functions of this structure on any thread.
//
// CEF maintains multiple internal threads that are used for handling different
// types of tasks in different processes. The cef_thread_id_t definitions in
// cef_types.h list the common CEF threads. Task runners are also available for
// other CEF threads as appropriate (for example, V8 WebWorker threads).
//
#[repr(C)]
pub struct _cef_task_runner_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Returns true (1) if this object is pointing to the same task runner as
  // |that| object.
  //
  pub is_same: Option<extern "C" fn(this: *mut cef_task_runner_t,
      that: *mut interfaces::cef_task_runner_t) -> libc::c_int>,

  //
  // Returns true (1) if this task runner belongs to the current thread.
  //
  pub belongs_to_current_thread: Option<extern "C" fn(
      this: *mut cef_task_runner_t) -> libc::c_int>,

  //
  // Returns true (1) if this task runner is for the specified CEF thread.
  //
  pub belongs_to_thread: Option<extern "C" fn(this: *mut cef_task_runner_t,
      threadId: types::cef_thread_id_t) -> libc::c_int>,

  //
  // Post a task for execution on the thread associated with this task runner.
  // Execution will occur asynchronously.
  //
  pub post_task: Option<extern "C" fn(this: *mut cef_task_runner_t,
      task: *mut interfaces::cef_task_t) -> libc::c_int>,

  //
  // Post a task for delayed execution on the thread associated with this task
  // runner. Execution will occur asynchronously. Delayed tasks are not
  // supported on V8 WebWorker threads and will be executed without the
  // specified delay.
  //
  pub post_delayed_task: Option<extern "C" fn(this: *mut cef_task_runner_t,
      task: *mut interfaces::cef_task_t, delay_ms: i64) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_task_runner_t = _cef_task_runner_t;


//
// Structure that asynchronously executes tasks on the associated thread. It is
// safe to call the functions of this structure on any thread.
//
// CEF maintains multiple internal threads that are used for handling different
// types of tasks in different processes. The cef_thread_id_t definitions in
// cef_types.h list the common CEF threads. Task runners are also available for
// other CEF threads as appropriate (for example, V8 WebWorker threads).
//
pub struct CefTaskRunner {
  c_object: *mut cef_task_runner_t,
}

impl Clone for CefTaskRunner {
  fn clone(&self) -> CefTaskRunner{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefTaskRunner {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefTaskRunner {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefTaskRunner {
  pub unsafe fn from_c_object(c_object: *mut cef_task_runner_t) -> CefTaskRunner {
    CefTaskRunner {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_task_runner_t) -> CefTaskRunner {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefTaskRunner {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_task_runner_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_task_runner_t {
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
  // Returns true (1) if this object is pointing to the same task runner as
  // |that| object.
  //
  pub fn is_same(&self, that: interfaces::CefTaskRunner) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_same.unwrap())(
          self.c_object,
          CefWrap::to_c(that)))
    }
  }

  //
  // Returns true (1) if this task runner belongs to the current thread.
  //
  pub fn belongs_to_current_thread(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).belongs_to_current_thread.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if this task runner is for the specified CEF thread.
  //
  pub fn belongs_to_thread(&self,
      threadId: types::cef_thread_id_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).belongs_to_thread.unwrap())(
          self.c_object,
          CefWrap::to_c(threadId)))
    }
  }

  //
  // Post a task for execution on the thread associated with this task runner.
  // Execution will occur asynchronously.
  //
  pub fn post_task(&self, task: interfaces::CefTask) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).post_task.unwrap())(
          self.c_object,
          CefWrap::to_c(task)))
    }
  }

  //
  // Post a task for delayed execution on the thread associated with this task
  // runner. Execution will occur asynchronously. Delayed tasks are not
  // supported on V8 WebWorker threads and will be executed without the
  // specified delay.
  //
  pub fn post_delayed_task(&self, task: interfaces::CefTask,
      delay_ms: i64) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).post_delayed_task.unwrap())(
          self.c_object,
          CefWrap::to_c(task),
          CefWrap::to_c(delay_ms)))
    }
  }

  //
  // Returns the task runner for the current thread. Only CEF threads will have
  // task runners. An NULL reference will be returned if this function is called
  // on an invalid thread.
  //
  pub fn get_for_current_thread() -> interfaces::CefTaskRunner {
    unsafe {
      CefWrap::to_rust(
        ::task::cef_task_runner_get_for_current_thread(
))
    }
  }

  //
  // Returns the task runner for the specified CEF thread.
  //
  pub fn get_for_thread(
      threadId: types::cef_thread_id_t) -> interfaces::CefTaskRunner {
    unsafe {
      CefWrap::to_rust(
        ::task::cef_task_runner_get_for_thread(
          CefWrap::to_c(threadId)))
    }
  }
}

impl CefWrap<*mut cef_task_runner_t> for CefTaskRunner {
  fn to_c(rust_object: CefTaskRunner) -> *mut cef_task_runner_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_task_runner_t) -> CefTaskRunner {
    CefTaskRunner::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_task_runner_t> for Option<CefTaskRunner> {
  fn to_c(rust_object: Option<CefTaskRunner>) -> *mut cef_task_runner_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_task_runner_t) -> Option<CefTaskRunner> {
    if c_object.is_null() {
      None
    } else {
      Some(CefTaskRunner::from_c_object_addref(c_object))
    }
  }
}

