use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Body {
    pub kind: String,
    pub topic: String,
    pub msg_type: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ClientMessage {
    pub request: String,
    pub body: Body,
}
