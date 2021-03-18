use super::subscription::*;
use async_graphql::*;

#[derive(Default, MergedSubscription)]
pub struct RootSubscription(
  //TopicBandwidthSubscription,
  //TopicDelaySubscription,
  TopicEchoSubscription,
  //TopicHzSubscription,
);
