use async_graphql::*;

#[derive(Clone, Default)]
pub struct TopicFindQuery;

#[Object]
impl TopicFindQuery {
    async fn dummy(&self) -> Option<String> {
        None
    }
}
