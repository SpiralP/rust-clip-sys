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

#[test]
fn test_paths() {
  unsafe {
    use widestring::WideStr;

    const PATHS_CAPACITY: usize = 100;

    let mut paths: [clip_path; PATHS_CAPACITY] = std::mem::zeroed();

    let paths_length = clip_get_paths(paths.as_mut_ptr(), &(PATHS_CAPACITY as u64));
    assert!(paths_length > 0);

    println!("{}", paths_length);

    let paths: Vec<String> = paths[..paths_length as usize]
      .iter()
      .map(|path| {
        if path.wide {
          let buf = &path.buf.wide[..path.length as usize];

          WideStr::from_slice(buf).to_string().unwrap()
        } else {
          let buf = &path.buf.ansi[..path.length as usize];
          let buf = &*(buf as *const [i8] as *const [u8]);
          let buf = buf.to_vec();

          String::from_utf8(buf).unwrap()
        }
      })
      .collect();

    println!("{:#?}", paths);
  }
}
