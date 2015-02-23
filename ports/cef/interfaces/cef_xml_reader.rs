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
// Structure that supports the reading of XML data via the libxml streaming API.
// The functions of this structure should only be called on the thread that
// creates the object.
//
#[repr(C)]
pub struct _cef_xml_reader_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Moves the cursor to the next node in the document. This function must be
  // called at least once to set the current cursor position. Returns true (1)
  // if the cursor position was set successfully.
  //
  pub move_to_next_node: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Close the document. This should be called directly to ensure that cleanup
  // occurs on the correct thread.
  //
  pub close: Option<extern "C" fn(this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns true (1) if an error has been reported by the XML parser.
  //
  pub has_error: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns the error string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_error: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,


  // The below functions retrieve data for the node at the current cursor
  // position.

  //
  // Returns the node type.
  //
  pub get_type: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_xml_node_type_t>,

  //
  // Returns the node depth. Depth starts at 0 for the root node.
  //
  pub get_depth: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns the local name. See http://www.w3.org/TR/REC-xml-names/#NT-
  // LocalPart for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_local_name: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the namespace prefix. See http://www.w3.org/TR/REC-xml-names/ for
  // additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_prefix: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the qualified name, equal to (Prefix:)LocalName. See
  // http://www.w3.org/TR/REC-xml-names/#ns-qualnames for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_qualified_name: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the URI defining the namespace associated with the node. See
  // http://www.w3.org/TR/REC-xml-names/ for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_namespace_uri: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the base URI of the node. See http://www.w3.org/TR/xmlbase/ for
  // additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_base_uri: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the xml:lang scope within which the node resides. See
  // http://www.w3.org/TR/REC-xml/#sec-lang-tag for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_xml_lang: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns true (1) if the node represents an NULL element. <a/> is considered
  // NULL but <a></a> is not.
  //
  pub is_empty_element: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns true (1) if the node has a text value.
  //
  pub has_value: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns the text value.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_value: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns true (1) if the node has attributes.
  //
  pub has_attributes: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Returns the number of attributes.
  //
  pub get_attribute_count: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::size_t>,

  //
  // Returns the value of the attribute at the specified 0-based index.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_attribute_byindex: Option<extern "C" fn(this: *mut cef_xml_reader_t,
      index: libc::c_int) -> types::cef_string_userfree_t>,

  //
  // Returns the value of the attribute with the specified qualified name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_attribute_byqname: Option<extern "C" fn(this: *mut cef_xml_reader_t,
      qualifiedName: *const types::cef_string_t) -> types::cef_string_userfree_t>,

  //
  // Returns the value of the attribute with the specified local name and
  // namespace URI.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_attribute_bylname: Option<extern "C" fn(this: *mut cef_xml_reader_t,
      localName: *const types::cef_string_t,
      namespaceURI: *const types::cef_string_t) -> types::cef_string_userfree_t>,

  //
  // Returns an XML representation of the current node's children.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_inner_xml: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns an XML representation of the current node including its children.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub get_outer_xml: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> types::cef_string_userfree_t>,

  //
  // Returns the line number for the current node.
  //
  pub get_line_number: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,


  // Attribute nodes are not traversed by default. The below functions can be
  // used to move the cursor to an attribute node. move_to_carrying_element()
  // can be called afterwards to return the cursor to the carrying element. The
  // depth of an attribute node will be 1 + the depth of the carrying element.

  //
  // Moves the cursor to the attribute at the specified 0-based index. Returns
  // true (1) if the cursor position was set successfully.
  //
  pub move_to_attribute_byindex: Option<extern "C" fn(
      this: *mut cef_xml_reader_t, index: libc::c_int) -> libc::c_int>,

  //
  // Moves the cursor to the attribute with the specified qualified name.
  // Returns true (1) if the cursor position was set successfully.
  //
  pub move_to_attribute_byqname: Option<extern "C" fn(
      this: *mut cef_xml_reader_t,
      qualifiedName: *const types::cef_string_t) -> libc::c_int>,

  //
  // Moves the cursor to the attribute with the specified local name and
  // namespace URI. Returns true (1) if the cursor position was set
  // successfully.
  //
  pub move_to_attribute_bylname: Option<extern "C" fn(
      this: *mut cef_xml_reader_t, localName: *const types::cef_string_t,
      namespaceURI: *const types::cef_string_t) -> libc::c_int>,

  //
  // Moves the cursor to the first attribute in the current element. Returns
  // true (1) if the cursor position was set successfully.
  //
  pub move_to_first_attribute: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Moves the cursor to the next attribute in the current element. Returns true
  // (1) if the cursor position was set successfully.
  //
  pub move_to_next_attribute: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // Moves the cursor back to the carrying element. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub move_to_carrying_element: Option<extern "C" fn(
      this: *mut cef_xml_reader_t) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_xml_reader_t = _cef_xml_reader_t;


//
// Structure that supports the reading of XML data via the libxml streaming API.
// The functions of this structure should only be called on the thread that
// creates the object.
//
pub struct CefXmlReader {
  c_object: *mut cef_xml_reader_t,
}

impl Clone for CefXmlReader {
  fn clone(&self) -> CefXmlReader{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefXmlReader {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefXmlReader {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefXmlReader {
  pub unsafe fn from_c_object(c_object: *mut cef_xml_reader_t) -> CefXmlReader {
    CefXmlReader {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_xml_reader_t) -> CefXmlReader {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefXmlReader {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_xml_reader_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_xml_reader_t {
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
  // Moves the cursor to the next node in the document. This function must be
  // called at least once to set the current cursor position. Returns true (1)
  // if the cursor position was set successfully.
  //
  pub fn move_to_next_node(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_next_node.unwrap())(
          self.c_object))
    }
  }

  //
  // Close the document. This should be called directly to ensure that cleanup
  // occurs on the correct thread.
  //
  pub fn close(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).close.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if an error has been reported by the XML parser.
  //
  pub fn has_error(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_error.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the error string.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_error(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_error.unwrap())(
          self.c_object))
    }
  }


  // The below functions retrieve data for the node at the current cursor
  // position.

  //
  // Returns the node type.
  //
  pub fn get_type(&self) -> types::cef_xml_node_type_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_type.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the node depth. Depth starts at 0 for the root node.
  //
  pub fn get_depth(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_depth.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the local name. See http://www.w3.org/TR/REC-xml-names/#NT-
  // LocalPart for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_local_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_local_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the namespace prefix. See http://www.w3.org/TR/REC-xml-names/ for
  // additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_prefix(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_prefix.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the qualified name, equal to (Prefix:)LocalName. See
  // http://www.w3.org/TR/REC-xml-names/#ns-qualnames for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_qualified_name(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_qualified_name.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the URI defining the namespace associated with the node. See
  // http://www.w3.org/TR/REC-xml-names/ for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_namespace_uri(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_namespace_uri.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the base URI of the node. See http://www.w3.org/TR/xmlbase/ for
  // additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_base_uri(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_base_uri.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the xml:lang scope within which the node resides. See
  // http://www.w3.org/TR/REC-xml/#sec-lang-tag for additional details.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_xml_lang(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_xml_lang.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the node represents an NULL element. <a/> is considered
  // NULL but <a></a> is not.
  //
  pub fn is_empty_element(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).is_empty_element.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the node has a text value.
  //
  pub fn has_value(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the text value.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_value(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_value.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns true (1) if the node has attributes.
  //
  pub fn has_attributes(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).has_attributes.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the number of attributes.
  //
  pub fn get_attribute_count(&self) -> libc::size_t {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_attribute_count.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the value of the attribute at the specified 0-based index.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_attribute_byindex(&self, index: libc::c_int) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_attribute_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Returns the value of the attribute with the specified qualified name.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_attribute_byqname(&self, qualifiedName: &[u16]) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_attribute_byqname.unwrap())(
          self.c_object,
          CefWrap::to_c(qualifiedName)))
    }
  }

  //
  // Returns the value of the attribute with the specified local name and
  // namespace URI.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_attribute_bylname(&self, localName: &[u16],
      namespaceURI: &[u16]) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_attribute_bylname.unwrap())(
          self.c_object,
          CefWrap::to_c(localName),
          CefWrap::to_c(namespaceURI)))
    }
  }

  //
  // Returns an XML representation of the current node's children.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_inner_xml(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_inner_xml.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns an XML representation of the current node including its children.
  //
  // The resulting string must be freed by calling cef_string_userfree_free().
  pub fn get_outer_xml(&self) -> String {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_outer_xml.unwrap())(
          self.c_object))
    }
  }

  //
  // Returns the line number for the current node.
  //
  pub fn get_line_number(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).get_line_number.unwrap())(
          self.c_object))
    }
  }


  // Attribute nodes are not traversed by default. The below functions can be
  // used to move the cursor to an attribute node. move_to_carrying_element()
  // can be called afterwards to return the cursor to the carrying element. The
  // depth of an attribute node will be 1 + the depth of the carrying element.

  //
  // Moves the cursor to the attribute at the specified 0-based index. Returns
  // true (1) if the cursor position was set successfully.
  //
  pub fn move_to_attribute_byindex(&self, index: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_attribute_byindex.unwrap())(
          self.c_object,
          CefWrap::to_c(index)))
    }
  }

  //
  // Moves the cursor to the attribute with the specified qualified name.
  // Returns true (1) if the cursor position was set successfully.
  //
  pub fn move_to_attribute_byqname(&self,
      qualifiedName: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_attribute_byqname.unwrap())(
          self.c_object,
          CefWrap::to_c(qualifiedName)))
    }
  }

  //
  // Moves the cursor to the attribute with the specified local name and
  // namespace URI. Returns true (1) if the cursor position was set
  // successfully.
  //
  pub fn move_to_attribute_bylname(&self, localName: &[u16],
      namespaceURI: &[u16]) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_attribute_bylname.unwrap())(
          self.c_object,
          CefWrap::to_c(localName),
          CefWrap::to_c(namespaceURI)))
    }
  }

  //
  // Moves the cursor to the first attribute in the current element. Returns
  // true (1) if the cursor position was set successfully.
  //
  pub fn move_to_first_attribute(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_first_attribute.unwrap())(
          self.c_object))
    }
  }

  //
  // Moves the cursor to the next attribute in the current element. Returns true
  // (1) if the cursor position was set successfully.
  //
  pub fn move_to_next_attribute(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_next_attribute.unwrap())(
          self.c_object))
    }
  }

  //
  // Moves the cursor back to the carrying element. Returns true (1) if the
  // cursor position was set successfully.
  //
  pub fn move_to_carrying_element(&self) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).move_to_carrying_element.unwrap())(
          self.c_object))
    }
  }

  //
  // Create a new cef_xml_reader_t object. The returned object's functions can
  // only be called from the thread that created the object.
  //
  pub fn create(stream: interfaces::CefStreamReader,
      encodingType: types::cef_xml_encoding_type_t,
      URI: &[u16]) -> interfaces::CefXmlReader {
    unsafe {
      CefWrap::to_rust(
        ::xml_reader::cef_xml_reader_create(
          CefWrap::to_c(stream),
          CefWrap::to_c(encodingType),
          CefWrap::to_c(URI)))
    }
  }
}

impl CefWrap<*mut cef_xml_reader_t> for CefXmlReader {
  fn to_c(rust_object: CefXmlReader) -> *mut cef_xml_reader_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_xml_reader_t) -> CefXmlReader {
    CefXmlReader::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_xml_reader_t> for Option<CefXmlReader> {
  fn to_c(rust_object: Option<CefXmlReader>) -> *mut cef_xml_reader_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_xml_reader_t) -> Option<CefXmlReader> {
    if c_object.is_null() {
      None
    } else {
      Some(CefXmlReader::from_c_object_addref(c_object))
    }
  }
}

