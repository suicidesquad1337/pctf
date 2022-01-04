use async_graphql::{Context, Object, Result, ID};
use futures::stream::TryStreamExt;
use sqlx::PgPool;

use super::Challenge;

#[non_exhaustive]
#[derive(Debug, Default)]
/// GraphQL queries for a [`super::Challenge`]
pub struct ChallengeQueries;

#[Object]
impl ChallengeQueries {
    // cache it so its not that bad
    #[graphql(cache_control(max_age = 300))]
    /// return all challenges (no pagination yet)
    async fn challenges(&self, ctx: &Context<'_>) -> Result<Vec<Challenge>> {
        let pool = ctx.data_unchecked::<PgPool>();
        let challenges: Vec<_> = sqlx::query!(r#"SELECT "id" FROM challenges LIMIT 1000"#)
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
