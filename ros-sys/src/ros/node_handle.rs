use std::ffi::*;
use std::os::raw::*;

use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Result};

use super::callback::Callback;
use super::ffi;
use super::subscriber::Subscriber;

unsafe extern "C" fn __callback_fn(phantom_data: *const c_void, msg: *const c_char) {
    let raw_str = CStr::from_ptr(msg).to_str().unwrap();
    let handle = phantom_data as *const NodeHandle;

    (*handle).__call(&String::from(raw_str));
}

pub struct NodeHandle<'a> {
    node_name: String,
    __handle: *mut ffi::node_handle,
    __callback: Option<Box<dyn Callback + 'a>>,
}

impl NodeHandle<'_> {
    pub fn new(name: &String) -> NodeHandle {
        unsafe {
            let ns = CString::new(name.as_str()).expect("CString::new failed");
            let nh = ffi::node_handle_create(ns.as_ptr());

            NodeHandle {
                node_name: name.clone(),
                __handle: nh,
                __callback: None,
            }
        }
    }

    pub fn __call(&self, mgs: &String) {
        self.__callback.as_ref().unwrap().as_ref().callback(mgs);
    }

    pub fn subscribe(
        &mut self,
        topic: &String,
        topic_type: &String,
        queue_size: u32,
        cb_object: Box<dyn Callback>,
    ) -> Subscriber {
        unsafe {
            let _topic = CString::new(topic.as_str()).expect("CString::new failed");
            let _topic_type = CString::new(topic_type.as_str()).expect("CString::new failed");
            let _ptr: *mut c_void = self as *mut _ as *mut c_void;

            self.__callback = Some(cb_object);

            let _sub: *mut ffi::subscriber = ffi::node_handle_subscribe(
                self.__handle,
                _topic.as_ptr(),
                _topic_type.as_ptr(),
                queue_size,
                _ptr,
                Some(__callback_fn),
            );

            let mut sub = Subscriber::new(topic, topic_type);
            sub.set_handle(_sub);
            sub
        }
    }
}
