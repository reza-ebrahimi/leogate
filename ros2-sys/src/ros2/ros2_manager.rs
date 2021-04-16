use futures::lock::Mutex;
use futures::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::future::*;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

use super::*;

pub struct Ros2Manager {
  node: Option<Node>,
}

impl Ros2Manager {
  pub fn new() -> Self {
    Self { node: None }
  }

  pub fn create_node(&mut self, node_name: &str) {
    Init::init();
    self.node = Some(Node::new(node_name));
  }

  pub fn node<'a>(&'a mut self) -> &'a mut Node {
    self.node.as_mut().unwrap()
  }

  pub fn start_spinner(&mut self) {}
}

impl Drop for Ros2Manager {
  fn drop(&mut self) {
    Init::shutdown();

    println!("[Drop] Ros2Manager");
  }
}
