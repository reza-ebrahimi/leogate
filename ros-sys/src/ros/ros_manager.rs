use futures::lock::Mutex;
use futures::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::*;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use super::ChannelPayload;
use super::Init;
use super::NodeHandle;
use super::Subscriber;

pub struct RosManager {
  node_handle: Option<NodeHandle>,
  shutdown_flag: Arc<AtomicBool>,
  spinner_thread: Option<std::thread::JoinHandle<()>>,
}

impl RosManager {
  pub fn new() -> Self {
    let mut manager = RosManager {
      node_handle: None,
      shutdown_flag: Arc::new(AtomicBool::new(false)),
      spinner_thread: None,
    };

    //manager.start_spinner();
    manager
  }

  pub fn create_node(&mut self, node_name: &str) {
    loop {
      if Init::is_initialized() {
        break;
      }

      std::thread::sleep(std::time::Duration::from_millis(1));
    }

    self.node_handle = Some(NodeHandle::new(node_name));
  }

  pub fn node_handle<'a>(&'a mut self) -> &'a mut NodeHandle {
    self.node_handle.as_mut().unwrap()
  }

  pub fn shutdown_ros(&self) {
    self.shutdown_flag.store(true, Ordering::Relaxed);
  }

  pub fn start_spinner(&mut self) {
    let shutdown_flag = self.shutdown_flag.clone();

    self.spinner_thread = Some(std::thread::spawn(move || {
      if !Init::is_initialized() {
        Init::init(&String::from("leogated"));
      }

      println!(
        "[RosManager::start_spinner] InitializeRos => Init::is_initialized(): {}",
        Init::is_initialized()
      );

      loop {
        if Init::is_initialized() && Init::is_started() {
          Init::spin_once();
        }

        if shutdown_flag.load(Ordering::Relaxed) {
          break;
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
      }

      if Init::is_initialized() {
        Init::shutdown();
      }

      println!(
        "[RosManager::start_spinner] ShutdownRos => Init::is_shuttingdown(): {}",
        Init::is_shuttingdown()
      );
    }));
  }
}

impl Drop for RosManager {
  fn drop(&mut self) {
    self.shutdown_ros();
    if let Some(handle) = self.spinner_thread.take() {
      handle.join().expect("failed to join spinner thread");
    }

    println!("[Drop] RosManager");
  }
}
