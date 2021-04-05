use super::ffi;
use std::ffi::CStr;

pub fn disable_all_thread_signals() {
  unsafe {
    ffi::ros_disableAllSignalsInThisThread();
  }
}

pub struct Init;

impl Init {
  pub fn init(name: &String) {
    unsafe {
      ffi::ros_init(name.as_ptr(), 0);
    }
  }

  pub fn is_initialized() -> bool {
    unsafe { ffi::ros_isInitialized() }
  }

  pub fn ok() -> bool {
    unsafe { ffi::ros_ok() }
  }

  pub fn shutdown() {
    unsafe {
      ffi::ros_shutdown();
    }
  }

  pub fn request_shutdown() {
    unsafe {
      ffi::ros_requestShutdown();
    }
  }

  pub fn wait_for_shutdown() {
    unsafe {
      ffi::ros_waitForShutdown();
    }
  }

  pub fn is_shuttingdown() -> bool {
    unsafe { ffi::ros_isShuttingDown() }
  }

  pub fn spin() {
    unsafe {
      ffi::ros_spin();
    }
  }

  pub fn spin_once() {
    unsafe {
      ffi::ros_spinOnce();
    }
  }

  pub fn start() {
    unsafe {
      ffi::ros_start();
    }
  }

  pub fn is_started() -> bool {
    unsafe { ffi::ros_isStarted() }
  }

  pub fn get_default_master_uri() -> String {
    unsafe {
      CStr::from_ptr(ffi::ros_getDefaultMasterURI())
        .to_string_lossy()
        .into_owned()
    }
  }

  pub fn getGlobalCallbackQueue() {
    unimplemented!();
  }

  pub fn getROSArg() {
    unimplemented!();
  }
}
