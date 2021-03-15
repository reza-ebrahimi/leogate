use super::ffi;

pub fn disable_all_thread_signals() {
    unsafe {
        ffi::ros_disableAllSignalsInThisThread();
    }
}
pub struct Ros;

impl Ros {
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
            let mut len: i32 = 0;
            let slice = ffi::ros_getDefaultMasterURI(&mut len);
            let slice = std::slice::from_raw_parts(slice, len as usize);
            String::from_utf8(slice.to_owned()).unwrap()
        }
    }

    pub fn getGlobalCallbackQueue() {
        unimplemented!();
    }

    pub fn getROSArg() {
        unimplemented!();
    }
}
