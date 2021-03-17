use std::ffi::*;
use std::os::raw::*;

use super::callback::ChannelPayload;
use super::ffi;
use super::subscriber::Subscriber;

unsafe extern "C" fn __callback_fn(
    phantom_data: *const c_void,
    json: *const ffi::_json_payload,
    binary: *const ffi::_binary_payload,
) {
    let payload = if binary.is_null() {
        ChannelPayload {
            json: unsafe {
                CStr::from_ptr((*json).payload)
                    .to_str()
                    .unwrap()
                    .to_string()
            },
            binary: None,
        }
    } else {
        let binary_data = unsafe { std::slice::from_raw_parts((*binary).payload, (*binary).size) };

        ChannelPayload {
            json: "".to_string(),
            binary: Some(binary_data.to_vec()),
        }
    };

    let subscriber = phantom_data as *const Subscriber;
    (*subscriber).__call(payload);
}

pub struct NodeHandle {
    node_name: String,
    __handle: *mut ffi::node_handle,
}

impl NodeHandle {
    pub fn new(name: String) -> NodeHandle {
        unsafe {
            let ns = CString::new(name.as_str()).expect("CString::new failed");
            let nh = ffi::node_handle_create(ns.as_ptr());

            NodeHandle {
                node_name: name.clone(),
                __handle: nh,
            }
        }
    }

    pub fn subscribe(
        &mut self,
        topic: &String,
        topic_type: &String,
        queue_size: u32,
        notifier: Box<tokio::sync::mpsc::Sender<ChannelPayload>>,
    ) -> Box<Subscriber> {
        unsafe {
            let _topic = CString::new(topic.as_str()).expect("CString::new failed");
            let _topic_type = CString::new(topic_type.as_str()).expect("CString::new failed");

            let mut subscriber = Box::new(Subscriber::new(topic, topic_type, notifier));
            let subscriber_ptr: *const c_void = subscriber.as_ref() as *const _ as *const c_void;

            let ffi_subscriber: *mut ffi::subscriber = ffi::node_handle_subscribe(
                self.__handle,
                _topic.as_ptr(),
                _topic_type.as_ptr(),
                queue_size,
                subscriber_ptr,
                Some(__callback_fn),
            );

            subscriber.set_handle(ffi_subscriber);
            subscriber
        }
    }
}

impl Drop for NodeHandle {
    fn drop(&mut self) {
        unsafe {
            ffi::node_handle_destroy(self.__handle);
        }

        println!("[Drop] NodeHandle");
    }
}

unsafe impl Send for NodeHandle {}
unsafe impl Sync for NodeHandle {}
