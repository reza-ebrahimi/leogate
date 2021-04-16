use async_graphql::*;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
struct ClientInfo {
  name: String,
  port: u32,
  default: bool
}

#[derive(Clone, Default)]
pub struct ClientQuery;

#[Object]
impl ClientQuery {
  async fn clients(&self) -> Vec<ClientInfo> {
    vec![
        ClientInfo {
          name: "ROS".to_string(),
          port: 8001,
          default: false,
        },
        ClientInfo {
            name: "ROS2".to_string(),
            port: 8002,
            default: true,
        }
    ]
  }
}
