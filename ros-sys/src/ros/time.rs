use async_graphql::SimpleObject;

use super::ffi;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct RosTime {
  time_now_sec: f64,
  wall_now_sec: f64,
}

impl RosTime {
  pub fn new() -> Self {
    Self {
      time_now_sec: Time::now_to_sec(),
      wall_now_sec: WallTime::now_to_sec(),
    }
  }
}

pub struct Time;
pub struct WallTime;

impl Time {
  pub fn now_to_sec() -> f64 {
    unsafe { ffi::time_now_toSec() }
  }

  pub fn now_to_nsec() -> u64 {
    unsafe { ffi::time_now_toNSec() }
  }

  pub fn init() {
    unsafe {
      ffi::time_init();
    }
  }

  pub fn shutdown() {
    unsafe {
      ffi::time_shutdown();
    }
  }

  pub fn use_system_time() {
    unsafe {
      ffi::time_useSystemTime();
    }
  }

  pub fn is_sime_time() -> bool {
    unsafe { ffi::time_isSimTime() }
  }

  pub fn is_system_time() -> bool {
    unsafe { ffi::time_isSystemTime() }
  }

  pub fn is_valid() -> bool {
    unsafe { ffi::time_isValid() }
  }

  pub fn wait_for_valid() {
    unsafe {
      ffi::time_waitForValid();
    }
  }
}

impl WallTime {
  pub fn now_to_sec() -> f64 {
    unsafe { ffi::walltime_now_toSec() }
  }

  pub fn now_to_nsec() -> u64 {
    unsafe { ffi::walltime_now_toNSec() }
  }

  pub fn is_system_time() -> bool {
    unsafe { ffi::walltime_isSystemTime() }
  }
}
