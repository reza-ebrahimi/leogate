use std::{
  sync::Arc,
  time::{Duration, Instant},
};

use futures::lock::Mutex;

use actix::prelude::*;
use actix_web_actors::ws;

use super::client_request::*;

use ros_sys::RosManager;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct BinaryWebsocketHandler {
  ros: Arc<Mutex<RosManager>>,
  last_heartbeat: Instant,
}

impl BinaryWebsocketHandler {
  pub fn new(ros: Arc<Mutex<RosManager>>) -> Self {
    Self {
      ros,
      last_heartbeat: Instant::now(),
    }
  }

  fn send_heartbeats(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      if Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT {
        ctx.stop();
      }
      ctx.ping(b"");
    });
  }

  fn process_json_message(&self, msg: &str, ctx: &mut ws::WebsocketContext<Self>) {
    let client: ClientRequest = serde_json::from_str::<ClientRequest>(msg).unwrap();
    match client.request {
      RequestResource::StreamRead(_) => {
        match client.body.kind {
          RequestKind::PointCloud2(_) => {
            let stream = {
              let mut ros = futures::executor::block_on(async { self.ros.lock().await });
              ros.node_handle().subscribe(
                client.body.topic.as_str(),
                client.body.msg_type.as_str(),
                1024,
              )
            };
            stream
              .into_actor(self)
              .map(|response, _act, ctx| {
                if !response.binary.is_none() {
                  ctx.binary(response.binary.unwrap());
                }
              })
              .finish()
              .spawn(ctx);
          }
        };
      }
    }
  }
}

impl Actor for BinaryWebsocketHandler {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.send_heartbeats(ctx);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for BinaryWebsocketHandler {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.last_heartbeat = Instant::now();
        ctx.pong(&msg);
      }
      Ok(ws::Message::Pong(_)) => {
        self.last_heartbeat = Instant::now();
      }
      Ok(ws::Message::Text(text)) => {
        self.process_json_message(text.as_str(), ctx);
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        ctx.stop();
      }
      _ => ctx.stop(),
    }
  }
}
