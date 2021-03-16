mod ros_manager;
mod ros_manager_inner;
mod subscriber_stream;

pub use ros_manager::*;

#[derive(Clone, Debug)]
pub enum Message {
    InitializeRos,
    ShutdownRos,
    CreateNode {
        node: String,
    },
    DestroyNode {
        node: String,
    },
    Subscribe {
        topic: String,
        msg_type: String,
        notifier: Box<tokio::sync::mpsc::Sender<ChannelPayload>>,
    },
    ShutdownSubscriber {
        topic: String,
    },
    SpinOnce,
}
