use cc;
use cmake;

fn main() {
  let dst = cmake::Config::new("clip").build_target("clip").build();

  #[cfg(not(target_os = "windows"))]
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
    .cpp(true)
    .flag("-std=c++11")
    .include("clip")
    .file("clip_c_interface.cpp")
    .compile("clip_c_interface");

  #[cfg(target_os = "linux")]
  {
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=png");
  }

  #[cfg(target_os = "windows")]
  {
    println!("cargo:rustc-link-lib=User32");
  }

  #[cfg(target_os = "macos")]
  {
    println!("cargo:rustc-link-lib=framework=AppKit");
  }
}
