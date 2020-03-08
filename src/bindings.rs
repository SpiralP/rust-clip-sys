#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use crate::helpers::take_zeroed;
use failure::{Error, Fail};
use std::{ops::Deref, os::raw::c_char, slice};

#[derive(Debug, Fail)]
pub enum FFIError {
  #[fail(display = "pointer is null")]
  Null,
}

impl FFICharPtr {
  pub fn to_string(&self) -> Result<String, Error> {
    if self.is_null() {
      return Err(FFIError::Null.into());
    }

    let c_str: &[u8] =
      unsafe { slice::from_raw_parts(self.ptr as *const u8, self.length as usize) };

    Ok(String::from_utf8(c_str.to_vec())?)
  }
}
impl Deref for FFICharPtr {
  type Target = *mut c_char;

  fn deref(&self) -> &Self::Target {
    &self.ptr
  }
}
impl Drop for FFICharPtr {
  fn drop(&mut self) {
    if !self.is_null() {
      unsafe {
        FFICharPtr_delete(take_zeroed(self));
      }
    }
  }
}

impl clip_image {
  pub fn data(&self) -> *const u8 {
    self.m_data as *const u8
  }

  pub fn spec(&self) -> &clip_image_spec {
    &self.m_spec
  }

  pub fn is_valid(&self) -> bool {
    !self.m_data.is_null()
  }
}
