use std::ffi::*;
use std::os::raw::*;

use super::callback::ChannelPayload;
use super::ffi;

pub struct Subscriber {
    pub topic: String,
    data_type: String,
    __handle: *mut ffi::subscriber,
    __notifier: Box<tokio::sync::mpsc::Sender<ChannelPayload>>,
}

impl Subscriber {
    pub fn new(
        topic: &String,
        topic_type: &String,
        notifier: Box<tokio::sync::mpsc::Sender<ChannelPayload>>,
    ) -> Subscriber {
        Subscriber {
            topic: topic.clone(),
            data_type: topic_type.clone(),
            __handle: std::ptr::null_mut(),
            __notifier: notifier,
        }
    }

    pub fn shutdown(&self) {
        unsafe {
            ffi::subscriber_shutdown(self.__handle);
        }
    }

    pub fn get_topic(&self) -> String {
        unsafe {
            let _topic = ffi::subscriber_getTopic(self.__handle);
        }
        String::from("TODO")
    }

    pub fn get_num_publishers(&self) -> u32 {
        unsafe { ffi::subscriber_getNumPublishers(self.__handle) }
    }

    pub unsafe fn set_handle(&mut self, h: *mut ffi::subscriber) {
        self.__handle = h;
    }

    pub fn __call(&self, payload: ChannelPayload) {
        if !self.__notifier.is_closed() {
            futures::executor::block_on(self.__notifier.clone().send(payload)).unwrap();
        }
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        self.shutdown();

        unsafe {
            ffi::subscriber_destroy(self.__handle);
        }

        println!("[Drop] Subscriber");
    }
}

unsafe impl Send for Subscriber {}
unsafe impl Sync for Subscriber {}
