use super::query::*;
use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct RootQuery(
  TopicFindQuery,
  TopicInfoQuery,
  TopicListQuery,
  TopicTypeQuery,
);
