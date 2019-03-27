use std::os::raw::{c_char, c_ulong, c_void};

pub type ClipFormat = u8;

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

  pub fn clip_get_image() -> ClipImage;
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
fn test_has_empty() {
  unsafe {
    assert!(clip_has(clip_empty_format()));
  }
}

#[test]
fn test_has_text() {
  unsafe {
    assert!(clip_has(clip_text_format()));
  }
}

#[test]
fn test_has_image() {
  unsafe {
    assert!(clip_has(clip_image_format()));
  }
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
