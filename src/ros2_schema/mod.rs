use async_graphql::*;

mod inputs;
mod query;

pub use inputs::*;
use query::*;

#[derive(Default, MergedObject)]
pub struct RootQuery(Ros2NodeQuery, Ros2MasterQuery, Ros2TimeQuery);

pub type Ros2Schema =
  async_graphql::Schema<RootQuery, async_graphql::EmptyMutation, async_graphql::EmptySubscription>;
