use std::pin::Pin;
use std::sync::Arc;

use futures::prelude::*;
use futures::{lock::Mutex, task, task::Poll};

use ros_manager::RosManager;
use ros_sys::ChannelPayload;

pub struct PointClout2Stream {
  ros: Arc<Mutex<RosManager>>,
  topic: String,
  msg_type: String,
  payload_stream: Option<Pin<Box<dyn Stream<Item = ChannelPayload> + Send>>>,
}

impl PointClout2Stream {
  pub fn new(ros: Arc<Mutex<RosManager>>, topic: String, msg_type: String) -> Self {
    Self {
      ros,
      topic,
      msg_type,
      payload_stream: None,
    }
  }

  async fn get_stream(&self) -> impl Stream<Item = ChannelPayload> {
    self
      .ros
      .lock()
      .await
      .subscribe(self.topic.clone(), self.msg_type.clone())
      .await
  }
}

impl Stream for PointClout2Stream {
  type Item = ChannelPayload;

  fn poll_next(mut self: Pin<&mut Self>, ctx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
    if self.payload_stream.is_none() {
      let stream = match Box::pin(self.get_stream()).poll_unpin(ctx) {
        Poll::Ready(stream) => Some(stream),
        Poll::Pending => None,
      };
      self.payload_stream = Some(Box::pin(stream.unwrap()));
    }

    if self.payload_stream.is_some() {
      return match self.payload_stream.as_mut().unwrap().poll_next_unpin(ctx) {
        Poll::Ready(Some(data)) => Poll::Ready(Some(data)),
        Poll::Ready(None) => Poll::Pending,
        Poll::Pending => Poll::Pending,
      };
    }

    Poll::Pending
  }
}
