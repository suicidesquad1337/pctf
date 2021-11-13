use async_graphql::{EmptySubscription, MergedObject};

use crate::{
    challenge::{ChallengeMutations, ChallengeQueries},
    node::NodeQuery,
};

/// A type alias that uses our merged query/mutation objects
pub type Schema = async_graphql::Schema<Queries, Mutations, EmptySubscription>;

#[derive(MergedObject, Default)]
/// This struct merges all queries
pub struct Queries(NodeQuery, ChallengeQueries);

#[derive(MergedObject, Default)]
/// This struct mrges all mutations
pub struct Mutations(ChallengeMutations);
