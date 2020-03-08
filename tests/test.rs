use clip_sys::*;

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
  unsafe {
    use std::ffi::CString;
    let c_string = CString::new(in_string.clone()).unwrap();

    assert!(clip_set_text(c_string.as_ptr()));
  }

  // paste
  let out_string = unsafe {
    let c_str = clip_get_text();
    assert!(!c_str.is_null());

    c_str.to_string().unwrap()
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

    let mut img = clip_image::new();
    let ok = clip_get_image(&mut img);
    assert!(ok);

    println!("{:#?}", img.spec());
    println!("{:#?}", img.data());
  }
}
