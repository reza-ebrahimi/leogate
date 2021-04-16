use std::env;
use std::process::Command;

use bindgen;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=Cargo.lock");

  let out_dir = env::var("OUT_DIR").unwrap();
  println!("OUT_DIR: {}", out_dir);

  let bindings = bindgen::Builder::default()
    .header("../ros2-native/src/ros2-native/include/ros2.h")
    .clang_arg("-I/opt/ros/foxy/include")
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
    .write_to_file("src/ros2/ffi.rs")
    .expect("Couldn't write ffi bindings to file!");

  Command::new("bash")
    //.args(&["-i", "-c", "'source /home/ros2/leogate/ros2-native/install/setup.bash && colcon build'"])
    .args(&["-i", "-c", "'./build.sh'"])
    //.current_dir("../ros2-native")
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native=ros2-native/build/ros2-native");
  println!("cargo:rustc-link-lib=dylib=ros2-native");
  println!("cargo:rustc-flags=-l dylib=stdc++");
}
