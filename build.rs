use std::{env, path::PathBuf};

fn main() {
  println!("cargo:rerun-if-changed=./src/interface.cpp");

  let dst = cmake::Config::new("clip").build_target("clip").build();

  #[cfg(not(target_os = "windows"))]
  {
    println!("cargo:rustc-link-search=native={}/build", dst.display());
  }

  #[cfg(target_os = "windows")]
  {
    #[cfg(debug_assertions)]
    {
      println!(
        "cargo:rustc-link-search=native={}/build/Debug",
        dst.display()
      );
    }

    #[cfg(not(debug_assertions))]
    {
      println!(
        "cargo:rustc-link-search=native={}/build/Release",
        dst.display()
      );
    }
  }

  println!("cargo:rustc-link-lib=static=clip");

  let mut config = cc::Build::new();
  config.cpp(true);

  #[cfg(not(target_os = "windows"))]
  {
    config.flag("-std=c++14");
  }

  config.include("clip");
  config.file("src/interface.cpp");
  config.compile("interface");

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

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header("./src/interface.cpp")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .whitelist_function("clip::empty_format")
    .whitelist_function("clip::text_format")
    .whitelist_function("clip::image_format")
    .whitelist_function("clip::has")
    // .whitelist_function("clip::get_image")
    // .whitelist_type("clip::image.*")
    // .whitelist_function("clip::image.*")
    .whitelist_function("clip_.*")
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
