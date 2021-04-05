use super::query::*;
use async_graphql::*;

#[derive(Default, MergedObject)]
pub struct RootQuery(TopicQuery, RosMasterQuery, RosNodeQuery);
