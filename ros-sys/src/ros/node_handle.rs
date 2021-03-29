use std::collections::HashMap;
use std::ffi::*;
use std::os::raw::*;
use std::sync::Arc;

use futures::lock::Mutex;

use super::callback::*;
use super::ffi;
use super::subscriber::*;

pub struct NodeHandle {
  node_name: String,
  sub_rc: Arc<Mutex<SubscriberRc>>,
  __handle: *mut ffi::node_handle,
}

impl NodeHandle {
  pub fn new(name: &str) -> NodeHandle {
    unsafe {
      let ns = CString::new(name).expect("CString::new failed");
      let nh = ffi::node_handle_create(ns.as_ptr());

      let rc = Arc::new(Mutex::new(SubscriberRc::new()));
      futures::executor::block_on(async {
        CHANNEL_MAMANGER.lock().await.set_rc(rc.clone());
      });

      NodeHandle {
        node_name: String::from(name),
        sub_rc: rc,
        __handle: nh,
      }
    }
  }

  pub fn subscribe(&mut self, topic: &str, msg_type: &str, queue_size: u32) -> ChannelStream {
    let mut gaurds = futures::executor::block_on(async {
      (self.sub_rc.lock().await, CHANNEL_MAMANGER.lock().await)
    });

    if gaurds.0.is_exists(topic) {
      return gaurds.1.channel(topic).subscribe();
    }

    let mut subscriber = Box::new(Subscriber::new(topic, msg_type));

    unsafe {
      let _topic = CString::new(topic).expect("CString::new failed");
      let _msg_type = CString::new(msg_type).expect("CString::new failed");
      let ffi_subscriber: *mut ffi::subscriber = ffi::node_handle_subscribe(
        self.__handle,
        _topic.as_ptr(),
        _msg_type.as_ptr(),
        queue_size,
        Some(__callback_fn),
      );

      subscriber.set_handle(ffi_subscriber);
    };

    gaurds.0.add_subscriber(topic, subscriber);
    gaurds.1.channel(topic).subscribe()
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
