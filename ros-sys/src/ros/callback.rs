use actix::prelude::*;

pub trait Callback: Sync + Send {
  fn callback(&mut self, d: &String);
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct ChannelPayload {
  pub json: String,
  pub binary: Option<Vec<u8>>,
}
