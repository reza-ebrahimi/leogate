fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=main.rs");
  println!("cargo:rerun-if-changed=Cargo.lock");
  println!(
    "cargo:rustc-env=LD_LIBRARY_PATH={}",
    concat!(
      env!("LD_LIBRARY_PATH"),
      ":/opt/ros/melodic/lib",
      ":ros-native/devel/lib"
    )
  );
}
