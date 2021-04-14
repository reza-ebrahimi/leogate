use futures::lock::Mutex;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct TokioRuntime {
  rt: Arc<Mutex<Runtime>>,
}

impl TokioRuntime {
  pub fn new() -> Self {
    Self {
      rt: Arc::new(Mutex::new(Runtime::new().unwrap())),
    }
  }

  pub fn handle(&self) -> Arc<Mutex<Runtime>> {
    self.rt.clone()
  }
}
