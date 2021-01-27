use std::ffi::*;
use std::os::raw::*;

use super::ffi;

pub struct Subscriber {
    topic: String,
    data_type: String,
    __handle: *mut ffi::subscriber,
}

impl Subscriber {
    pub fn new(topic: &String, topic_type: &String) -> Subscriber {
        Subscriber {
            topic: topic.clone(),
            data_type: topic_type.clone(),
            __handle: std::ptr::null_mut(),
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            ffi::subscriber_shutdown(self.__handle);
        }
    }

    pub fn get_topic(&self) -> String {
        unsafe {
            let topic = ffi::subscriber_getTopic(self.__handle);
        }
        String::from("TODO")
    }

    pub fn get_num_publishers(&self) -> u32 {
        unsafe { ffi::subscriber_getNumPublishers(self.__handle) }
    }

    pub unsafe fn set_handle(&mut self, h: *mut ffi::subscriber) {
        self.__handle = h;
    }
}
