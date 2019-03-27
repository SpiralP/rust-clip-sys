use cc;
use cmake;

fn main() {
  let dst = cmake::Config::new("clip").build_target("clip").build();

  #[cfg(target_os = "linux")]
  {
    println!("cargo:rustc-link-search=native={}/build", dst.display());
  }

  #[cfg(target_os = "windows")]
  {
    println!(
      "cargo:rustc-link-search=native={}/build/Debug",
      dst.display()
    );
  }

  println!("cargo:rustc-link-lib=static=clip");

  cc::Build::new()
    .include("clip")
    .file("clip_c_interface.cpp")
    .compile("clip_c_interface");

  // these have to come after cc::Build for some reason
  #[cfg(target_os = "linux")]
  {
    // linux, undefined reference to operator new
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=png");
  }

  #[cfg(target_os = "windows")]
  {
    println!("cargo:rustc-link-lib=User32");
  }
}
