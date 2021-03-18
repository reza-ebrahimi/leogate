use std::collections::HashMap;
use std::ffi::*;
use std::os::raw::*;

use super::callback::ChannelPayload;
use super::ffi;

pub struct Subscriber {
  pub topic: String,
  data_type: String,
  __handle: *mut ffi::subscriber,
  __notifiers: HashMap<String, Box<tokio::sync::mpsc::Sender<ChannelPayload>>>,
}

impl Subscriber {
  pub fn new(topic: &String, topic_type: &String) -> Subscriber {
    Subscriber {
      topic: topic.clone(),
      data_type: topic_type.clone(),
      __handle: std::ptr::null_mut(),
      __notifiers: HashMap::new(),
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

  pub fn add_notifier(
    &mut self,
    uuid: &str,
    notifier: Box<tokio::sync::mpsc::Sender<ChannelPayload>>,
  ) {
    self.__notifiers.insert(String::from(uuid), notifier);
  }

  pub fn remove_notifier(&mut self, uuid: &str) -> bool {
    if self.__notifiers.remove(uuid).is_none() {
      return false;
    }
    true
  }

  pub fn notifiers_count(&self) -> usize {
    self.__notifiers.len()
  }

  pub fn __call(&self, payload: ChannelPayload) {
    for (_, notifier) in self.__notifiers.iter() {
      if !notifier.is_closed() {
        futures::executor::block_on(notifier.clone().send(payload.clone())).unwrap();
      }
    }
  }
}

impl Drop for Subscriber {
  fn drop(&mut self) {
    unsafe {
      ffi::subscriber_shutdown(self.__handle);
      ffi::subscriber_destroy(self.__handle);
    }

    self.__notifiers.clear();

    println!("[Drop] Subscriber");
  }
}

unsafe impl Send for Subscriber {}
unsafe impl Sync for Subscriber {}
