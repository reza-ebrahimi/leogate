use std::collections::HashMap;

use ros_sys::*;

pub struct RosManagerInner {
  pub n_handle: Option<NodeHandle>,
  pub subscribers: HashMap<String, Box<Subscriber>>,
}

impl RosManagerInner {
  pub fn new() -> RosManagerInner {
    RosManagerInner {
      n_handle: None,
      subscribers: HashMap::new(),
    }
  }
}
