use std::pin::Pin;

use futures::prelude::*;
use futures::task::{Context, Poll};

use super::Message;
use ros_sys::ChannelPayload;

pub struct SubscriberStream {
  topic: String,
  spinner_sender: tokio::sync::mpsc::Sender<Message>,
  sender: tokio::sync::mpsc::Sender<ChannelPayload>,
  receiver: tokio::sync::mpsc::Receiver<ChannelPayload>,
  uuid: uuid::Uuid,
}

impl SubscriberStream {
  pub fn new(topic: String, sender: tokio::sync::mpsc::Sender<Message>, uuid: uuid::Uuid) -> Self {
    let (tx, rx) = tokio::sync::mpsc::channel::<ChannelPayload>(2048);
    SubscriberStream {
      topic,
      spinner_sender: sender,
      sender: tx,
      receiver: rx,
      uuid,
    }
  }

  pub fn sender_channel(&self) -> tokio::sync::mpsc::Sender<ChannelPayload> {
    self.sender.clone()
  }
}

impl Drop for SubscriberStream {
  fn drop(&mut self) {
    let msg = Message::ShutdownSubscriber {
      topic: self.topic.clone(),
      notifier_uuid: self.uuid.to_string(),
    };

    match futures::executor::block_on(self.spinner_sender.send(msg)) {
      Err(e) => {
        println!("[Drop] Error: {}", e)
      }
      _ => {}
    }

    println!("[Drop] SubscriberStream");
  }
}

impl Stream for SubscriberStream {
  type Item = ChannelPayload;

  fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    self.receiver.poll_recv(ctx)
  }
}
