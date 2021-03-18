use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicTypeQuery;

#[Object]
impl TopicTypeQuery {
  async fn dummy(&self) -> Option<String> {
    None
  }
}
