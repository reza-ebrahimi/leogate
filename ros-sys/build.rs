use std::env;
use std::process::Command;

use bindgen;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=Cargo.lock");

  let out_dir = env::var("OUT_DIR").unwrap();
  println!("OUT_DIR: {}", out_dir);

  let bindings = bindgen::Builder::default()
    .header("../ros-native/src/ros-native/include/ros.h")
    .clang_arg("-I/opt/ros/melodic/include/")
    .clang_arg("-I/usr/include")
    .clang_arg("-I/usr/include/c++")
    .clang_arg("-I/usr/include/c++/7")
    .clang_arg("-I/usr/include/x86_64-linux-gnu/c++/7")
    .size_t_is_usize(true)
    .derive_debug(false)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate_inline_functions(true)
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file("src/ros/ffi.rs")
    .expect("Couldn't write ffi bindings to file!");

  Command::new("bash")
    .args(&["-i", "-c", "'catkin_make'"])
    .current_dir("../ros-native")
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native=ros-native/devel/lib");
  println!("cargo:rustc-link-lib=dylib=ros-native");
  println!("cargo:rustc-flags=-l dylib=stdc++");
}
