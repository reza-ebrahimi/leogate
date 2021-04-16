use super::ffi;

pub struct Init;

impl Init {
  pub fn init() {
    unsafe {
      ffi::ros2_init();
    }
  }

  pub fn shutdown() {
    unsafe {
      ffi::ros2_shutdown();
    }
  }

  pub fn ok() -> bool {
    unsafe { ffi::ros2_ok() }
  }
}
