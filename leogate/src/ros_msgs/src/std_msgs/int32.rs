use async_graphql::*;

#[derive(Clone, Debug, Default, SimpleObject, serde::Deserialize)]
pub struct Int32 {
  data: i32,
}
