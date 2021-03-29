use std::collections::HashMap;
use std::ffi::*;
use std::os::raw::*;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::lock::Mutex;

use lazy_static::lazy_static;
use std::ffi::*;

use tokio::sync::watch;
use tokio_stream::{wrappers::WatchStream, Stream, StreamExt};

use futures::prelude::*;

use super::callback::*;
use super::ffi;
use super::Init;

pub unsafe extern "C" fn __callback_fn(
  topic: *const ::std::os::raw::c_char,
  json_payload: *const ffi::json_payload,
  bin_payload: *const ffi::binary_payload,
) {
  let payload = if bin_payload.is_null() {
    ChannelPayload {
      json: unsafe {
        CStr::from_ptr((*json_payload).payload)
          .to_str()
          .unwrap()
          .to_string()
      },
      binary: None,
    }
  } else {
    let binary_data =
      unsafe { std::slice::from_raw_parts((*bin_payload).payload, (*bin_payload).size) };

    ChannelPayload {
      json: "".to_string(),
      binary: Some(binary_data.to_vec()),
    }
  };

  let c_str = unsafe { CStr::from_ptr(topic) };
  let mut guard = futures::executor::block_on(async { CHANNEL_MAMANGER.lock().await });
  guard.channel(c_str.to_str().unwrap()).publish(payload);
}

#[derive(Debug)]
pub struct Channel {
  topic: String,
  sender: watch::Sender<ChannelPayload>,
  receiver: watch::Receiver<ChannelPayload>,
  rc: Arc<Mutex<SubscriberRc>>,
  stream_rc: Arc<AtomicUsize>,
}

impl Channel {
  pub fn publish(&self, payload: ChannelPayload) {
    self.sender.send(payload).unwrap();
  }

  pub fn subscribe(&self) -> ChannelStream {
    let stream_rc = self.stream_rc.clone();
    stream_rc.fetch_add(1, Ordering::SeqCst);

    ChannelStream {
      topic: self.topic.clone(),
      stream: WatchStream::new(self.receiver.clone()),
      rc: self.rc.clone(),
      stream_rc,
    }
  }
}

impl Drop for Channel {
  fn drop(&mut self) {
    println!("[Drop] Channel");
  }
}

#[derive(Debug)]
pub struct ChannelStream {
  topic: String,
  stream: WatchStream<ChannelPayload>,
  stream_rc: Arc<AtomicUsize>,
  rc: Arc<Mutex<SubscriberRc>>,
}

impl Stream for ChannelStream {
  type Item = ChannelPayload;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut stream: Pin<Box<dyn Stream<Item = ChannelPayload>>> = Box::pin(&mut self.stream);
    Stream::poll_next(stream.as_mut(), cx)
  }
}

impl Drop for ChannelStream {
  fn drop(&mut self) {
    self.stream_rc.fetch_sub(1, Ordering::SeqCst);

    if self.stream_rc.load(Ordering::SeqCst) <= 0 {
      futures::executor::block_on(async {
        let t = self.topic.as_str();
        self.rc.lock().await.remove_subscriber(t);
        CHANNEL_MAMANGER.lock().await.remove_channel(t);
      });
    }

    println!(
      "[Drop] ChannelStream => {}",
      self.stream_rc.load(Ordering::SeqCst)
    );
  }
}

struct ChannelFactory;

impl ChannelFactory {
  pub fn build(topic: &str, rc: Arc<Mutex<SubscriberRc>>) -> Channel {
    let (sender, receiver) = watch::channel(ChannelPayload::default());
    Channel {
      topic: String::from(topic),
      sender,
      receiver,
      rc,
      stream_rc: Arc::new(AtomicUsize::new(0)),
    }
  }
}

pub struct ChannelManager {
  channels: HashMap<String, Channel>,
  rc: Option<Arc<Mutex<SubscriberRc>>>,
}

impl ChannelManager {
  pub fn new() -> Self {
    Self {
      rc: None,
      channels: HashMap::new(),
    }
  }

  pub fn set_rc(&mut self, rc: Arc<Mutex<SubscriberRc>>) {
    self.rc = Some(rc);
  }

  pub fn channel<'a>(&'a mut self, topic: &'a str) -> &'a Channel {
    let t = String::from(topic);

    if !self.channels.contains_key(&t) {
      self.channels.insert(
        String::from(topic),
        ChannelFactory::build(topic, self.rc.as_mut().unwrap().clone()),
      );
    }

    return self.channels.get(&t).unwrap();
  }

  pub fn remove_channel(&mut self, topic: &str) {
    let t = String::from(topic);
    if self.channels.contains_key(&t) {
      self.channels.remove(&String::from(topic));
    }
  }
}

unsafe impl Send for ChannelManager {}

lazy_static! {
  pub static ref CHANNEL_MAMANGER: Arc<Mutex<ChannelManager>> =
    Arc::new(Mutex::new(ChannelManager::new()));
}

pub struct SubscriberRc {
  subscribers: HashMap<String, (i32, Box<Subscriber>)>,
}

impl SubscriberRc {
  pub fn new() -> Self {
    Self {
      subscribers: HashMap::new(),
    }
  }

  pub fn is_exists(&mut self, topic: &str) -> bool {
    for (t, _) in self.subscribers.iter_mut() {
      if t == topic {
        return true;
      }
    }

    false
  }

  pub fn add_subscriber(&mut self, topic: &str, sub: Box<Subscriber>) {
    let mut is_subscribed = false;
    for (t, (rc, subscriber)) in self.subscribers.iter_mut() {
      if t == topic {
        is_subscribed = true;
        *rc = *rc + 1;
      }
    }

    if !is_subscribed {
      self.subscribers.insert(String::from(topic), (1, sub));
    }
  }

  pub fn remove_subscriber(&mut self, topic: &str) {
    let mut should_removed = false;

    for (t, (rc, subscriber)) in self.subscribers.iter_mut() {
      if t == topic {
        *rc = *rc - 1;
        if *rc == 0 {
          should_removed = true;
          break;
        }
      }
    }

    if should_removed {
      self.subscribers.remove(&String::from(topic));
    }
  }
}

pub struct Subscriber {
  topic: String,
  data_type: String,
  __handle: *mut ffi::subscriber,
  __notifiers: HashMap<String, Box<tokio::sync::mpsc::Sender<ChannelPayload>>>,
}

impl Subscriber {
  pub fn new(topic: &str, topic_type: &str) -> Self {
    Self {
      topic: String::from(topic),
      data_type: String::from(topic_type),
      __handle: std::ptr::null_mut(),
      __notifiers: HashMap::new(),
    }
  }
  pub fn shutdown(&self) {
    unsafe {
      ffi::subscriber_shutdown(self.__handle);
    }
  }

  pub fn get_topic(&self) -> &str {
    unsafe {
      let _topic = ffi::subscriber_getTopic(self.__handle);
    }
    "TODO"
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

    println!("[Drop] Subscriber");
  }
}

unsafe impl Send for Subscriber {}
