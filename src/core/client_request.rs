use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ClientRequest {
  pub request: RequestResource,
  pub body: Body,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body {
  pub kind: RequestKind,
  pub topic: String,
  pub msg_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum RequestResource {
  StreamRead(String),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum RequestKind {
  PointCloud2(String),
  Int8(String),
  Int16(String),
  Int32(String),
  Int64(String),
}
