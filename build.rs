use cc;
use cmake;

fn main() {
  let dst = cmake::Config::new("clip").build_target("clip").build();
  println!("cargo:rustc-link-search=native={}/build", dst.display());
  println!("cargo:rustc-link-lib=static=clip");

  cc::Build::new()
    .include("clip")
    .file("clip_c_interface.cpp")
    .compile("clip_c_interface");

  // these have to come after cc::Build for some reason
  // linux, undefined reference to operator new
  println!("cargo:rustc-link-lib=stdc++");
  println!("cargo:rustc-link-lib=xcb");
  println!("cargo:rustc-link-lib=png");
}
