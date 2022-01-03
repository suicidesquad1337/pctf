use async_graphql::{dataloader::DataLoader as DL, Context, Object, Result, ID};
use futures::stream::TryStreamExt;
use sqlx::PgPool;

use super::Challenge;
use crate::loaders::ChallengeLoaderByName;

#[non_exhaustive]
#[derive(Debug, Default)]
/// GraphQL queries for a [`super::Challenge`]
pub struct ChallengeQueries;

#[Object]
impl ChallengeQueries {
    /// return a challenge by it's name. Usually you should not use this
    /// function, because minimal changes in the name like a newline would
    /// mean, that this query returns none.
    /// Instead, use the `Node` implementation of `Challenge` with the id.
    async fn challenge(&self, ctx: &Context<'_>, name: String) -> Result<Option<Challenge>> {
        Ok(ctx
            .data_unchecked::<DL<ChallengeLoaderByName>>()
            .load_one(name)
            .await?)
    }

    // cache it so its not that bad
    #[graphql(cache_control(max_age = 300))]
    /// return all challenges (no pagination yet)
    async fn challenges(&self, ctx: &Context<'_>) -> Result<Vec<Challenge>> {
        let pool = ctx.data_unchecked::<PgPool>();
        let challenges: Vec<_> = sqlx::query!(r#"SELECT "id" FROM challenges"#)
            .fetch(pool)
            .map_ok(|c| Challenge::from(c.id))
            .try_collect()
            .await?;
        Ok(challenges)
    }
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct ChallengeMutations;

#[Object]
impl ChallengeMutations {
    async fn submit_flag(&self, _flag: String, _challenge: ID) -> Option<String> {
        None
    }
}
