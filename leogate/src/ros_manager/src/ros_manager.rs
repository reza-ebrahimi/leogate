pub use ros_sys::*;

use std::sync::Arc;

use futures::lock::Mutex;
use futures::stream::Stream;

use super::ros_manager_inner::RosManagerInner;
use super::subscriber_stream::SubscriberStream;

use super::Message;

pub struct RosManager {
  sender: Arc<Mutex<tokio::sync::mpsc::Sender<Message>>>,
  receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>,
  messages: Vec<Message>,
}

unsafe impl Send for RosManager {}
unsafe impl Sync for RosManager {}

impl RosManager {
  pub fn new() -> RosManager {
    let channel = tokio::sync::mpsc::channel::<Message>(2048);
    RosManager {
      sender: Arc::new(Mutex::new(channel.0.clone())),
      receiver: Arc::new(Mutex::new(channel.1)),
      messages: vec![],
    }
  }

  pub fn init(&mut self) -> &mut Self {
    self.messages.push(Message::InitializeRos);
    self
  }

  pub fn create_node(&mut self, name: &str) -> &mut Self {
    self.messages.push(Message::CreateNode {
      node: name.to_string(),
    });
    self
  }

  pub async fn subscribe(
    &self,
    topic: String,
    msg_type: String,
  ) -> impl Stream<Item = ChannelPayload> {
    let uuid = uuid::Uuid::new_v4();
    let stream = SubscriberStream::new(topic.clone(), self.sender_channel().await.clone(), uuid);
    self
      .send_message(Message::Subscribe {
        topic,
        msg_type,
        notifier_uuid: uuid.to_string(),
        notifier: Box::new(stream.sender_channel()),
      })
      .await;
    stream
  }

  pub async fn sender_channel(&self) -> tokio::sync::mpsc::Sender<Message> {
    let channel = self.sender.lock().await.clone();
    channel
  }

  pub async fn send_message(&self, msg: Message) {
    if let Err(_) = self.sender_channel().await.send(msg).await {
      println!("[RosManager::send_message] Receiver dropped");
    }
  }

  pub async fn start_spin() {
    if Ros::is_initialized() && Ros::is_started() {
      Ros::spin_once();
    }
  }

  pub async fn process_messages(
    inner: Arc<Mutex<RosManagerInner>>,
    receiver: Arc<Mutex<tokio::sync::mpsc::Receiver<Message>>>,
  ) {
    let cmd = receiver.lock().await.recv().await;
    if cmd.is_none() {
      return;
    }

    match cmd.unwrap() {
      Message::InitializeRos => {
        if !Ros::is_initialized() {
          Ros::init(&String::from("leogated"));
        }
        println!(
          "[RosManager::process_messages] Message::InitializeRos => Ros::is_initialized(): {}",
          Ros::is_initialized()
        );
      }
      Message::ShutdownRos => {
        if Ros::is_initialized() && Ros::is_started() {
          Ros::shutdown();
        }
        println!(
          "[RosManager::process_messages] Message::ShutdownRos => Ros::is_shuttingdown(): {}",
          Ros::is_shuttingdown()
        );
      }
      Message::CreateNode { node } => {
        inner.lock().await.n_handle = Some(NodeHandle::new(node));
        println!("[RosManager::process_messages] Message::CreateNode");
      }
      Message::Subscribe {
        topic,
        msg_type,
        notifier_uuid,
        notifier,
      } => {
        let mut gaurd = inner.lock().await;
        match gaurd.subscribers.get_mut(&topic) {
          Some(subscriber) => {
            subscriber.add_notifier(notifier_uuid.as_str(), notifier);
          }
          None => {
            let mut subscriber = gaurd
              .n_handle
              .as_mut()
              .unwrap()
              .subscribe(&topic, &msg_type, 32);
            subscriber.add_notifier(notifier_uuid.as_str(), notifier);
            gaurd.subscribers.insert(topic, subscriber);
          }
        }

        println!("[Message] Subscribe");
      }
      Message::ShutdownSubscriber {
        topic,
        notifier_uuid,
      } => {
        let mut gaurd = inner.lock().await;

        match gaurd.subscribers.get_mut(&topic) {
          Some(subscriber) => {
            if subscriber.notifiers_count() > 1 {
              if !subscriber.remove_notifier(notifier_uuid.as_str()) {
                println!(
                  "[RosManager::process_messages] Topic notifier failed to remove. 
                  Topic: {} Subscribers Size: {}",
                  topic,
                  gaurd.subscribers.len()
                );
              }
              return;
            }

            subscriber.shutdown();
            match gaurd.subscribers.remove(&topic) {
              Some(_) => {
                println!(
                  "[RosManager::process_messages] Topic removed successfully. 
                  Topic: {} Subscribers Size: {}",
                  topic,
                  gaurd.subscribers.len()
                )
              }
              None => {}
            }
          }
          None => println!("Topic Error: {}", topic),
        }
        println!("[Message] ShutdownSubscriber");
      }
      _ => {
        println!("[Message] _");
      }
    }
  }

  pub async fn spin(&self) {
    let receiver = self.receiver.clone();

    let _ = std::thread::spawn(move || {
      println!("ROS spinner thread started!");

      let inner = Arc::new(futures::lock::Mutex::new(RosManagerInner::new()));
      let rt = tokio::runtime::Runtime::new().unwrap();

      rt.block_on(async move {
        loop {
          let inner = inner.clone();
          let receiver = receiver.clone();

          tokio::select! {
            _ = Self::process_messages(inner, receiver) => {},
            _ = Self::start_spin() => {},
          }

          tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
      });

      rt.shutdown_background();

      println!("ROS spinner thread exited!");
    });

    for msg in self.messages.iter() {
      self.send_message(msg.clone()).await;
    }
  }
}
