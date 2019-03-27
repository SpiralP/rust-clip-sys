use std::os::raw::c_int;

#[link(name = "clip_c_interface")]
extern "C" {
  fn test() -> c_int;
}

#[test]
fn test_test() {
  unsafe {
    println!("{}", test());
  }
}
