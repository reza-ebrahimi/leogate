use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicListQuery;

#[Object]
impl TopicListQuery {
    async fn dummy(&self) -> Option<String> {
        None
    }
}
