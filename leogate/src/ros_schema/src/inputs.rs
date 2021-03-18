use async_graphql::*;

#[derive(InputObject)]
pub struct TopicInput {
  pub topic: String,
  pub msg_type: String,
}
