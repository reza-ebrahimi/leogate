use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::lock::Mutex;

use actix::prelude::*;
use actix_web_actors::ws;

use ros_manager::RosManager;

use super::client_message::ClientMessage;
use super::pointcloud2_stream::PointClout2Stream;

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

    fn process_json_message(&self, msg: &String, ctx: &mut ws::WebsocketContext<Self>) {
        let client: ClientMessage = serde_json::from_str::<ClientMessage>(msg).unwrap();
        match client.request.as_str() {
            "STREAM_READ" => {
                match client.body.kind.as_str() {
                    "POINTCLOUD2" => {
                        PointClout2Stream::new(
                            self.ros.clone(),
                            client.body.topic,
                            client.body.msg_type,
                        )
                        .into_actor(self)
                        .map(|response, _act, ctx| {
                            ctx.binary(response.binary.unwrap());
                        })
                        .finish()
                        .spawn(ctx);
                    }
                    _ => {}
                };
            }
            _ => {}
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
                self.process_json_message(&text, ctx);
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
