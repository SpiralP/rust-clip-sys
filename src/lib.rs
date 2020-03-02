mod bindings;

pub use crate::bindings::*;

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
    if !clip_has(clip_image_format()) {
      eprintln!("skipping image test, no image in clipboard");
      return;
    }

    let img = clip_get_image();
    assert!(!img.is_null());

    println!("{:#?}", clip_get_image_spec(img));
    println!("{:#?}", clip_get_image_data(img));

    clip_delete_image(img);
  }
}
