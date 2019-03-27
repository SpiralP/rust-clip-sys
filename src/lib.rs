use std::os::raw::{c_char, c_ulong, c_void};

pub type ClipFormat = u8;

/// must call `clip_delete_image()`
pub type ClipImage = *const c_void;
pub type ClipImageData = *const c_char;

#[derive(Debug)]
#[repr(C)]
pub struct ClipImageSpec {
  pub width: c_ulong,
  pub height: c_ulong,
  pub bits_per_pixel: c_ulong,
  pub bytes_per_row: c_ulong,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub alpha_mask: c_ulong,
  pub red_shift: c_ulong,
  pub green_shift: c_ulong,
  pub blue_shift: c_ulong,
  pub alpha_shift: c_ulong,
}

#[link(name = "clip_c_interface")]
extern "C" {
  pub fn clip_empty_format() -> ClipFormat;
  pub fn clip_text_format() -> ClipFormat;
  pub fn clip_image_format() -> ClipFormat;

  pub fn clip_has(format: ClipFormat) -> bool;

  pub fn clip_set_text(text: *const c_char) -> bool;
  pub fn clip_get_text() -> *const c_char;
  pub fn clip_delete_text(text: *const c_char);

  pub fn clip_set_image(img: ClipImage) -> bool;
  pub fn clip_get_image() -> ClipImage;

  pub fn clip_create_image_from_data_spec(data: ClipImageData, spec: ClipImageSpec) -> ClipImage;
  pub fn clip_create_image_from_spec(spec: ClipImageSpec) -> ClipImage;

  pub fn clip_delete_image(img: ClipImage);

  pub fn clip_get_image_spec(img: ClipImage) -> ClipImageSpec;
  pub fn clip_get_image_data(img: ClipImage) -> ClipImageData;
}

#[test]
fn test_linking() {
  unsafe {
    assert_eq!(clip_empty_format(), 0);
    assert_eq!(clip_text_format(), 1);
    assert_eq!(clip_image_format(), 2);
  }
}

#[test]
fn test_has() {
  unsafe {
    assert!(
      clip_has(clip_empty_format())
        || clip_has(clip_text_format())
        || clip_has(clip_image_format())
    );
  }
}

#[test]
fn test_text() {
  let in_string = "bap".to_string();

  // copy
  {
    use std::ffi::CString;

    unsafe {
      let c_string = CString::new(in_string.clone()).unwrap();
      let raw = c_string.into_raw();

      assert!(clip_set_text(raw));

      CString::from_raw(raw);
    }
  }

  // paste
  let out_string = {
    use std::ffi::CStr;

    unsafe {
      let c_str = clip_get_text();
      assert!(!c_str.is_null());

      let out_string = CStr::from_ptr(c_str).to_str().unwrap().to_string();

      clip_delete_text(c_str);

      out_string
    }
  };

  assert_eq!(out_string, in_string);
}

#[test]
fn test_get_image() {
  unsafe {
    assert!(clip_has(clip_image_format()));

    let img = clip_get_image();
    assert!(!img.is_null());

    println!("{:#?}", clip_get_image_spec(img));
    println!("{:#?}", clip_get_image_data(img));

    clip_delete_image(img);
  }
}
