//! Loaders

mod challenge_loaders;

#[doc(inline)]
pub use challenge_loaders::*;

/// loader_struct macro
///
/// Takes the name of the struct to create. Generates a struct with a field ``pool`` and a ``new`` implementation.
#[macro_export]
macro_rules! loader_struct {
    ($name:ident) => {
        pub struct $name {
            pool: sqlx::Pool<sqlx::Postgres>,
        }

        impl $name {
            #[allow(unused)]
            pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
                Self { pool }
            }
        }
    };
}

/// basic_loader macro
///
/// Takes the name of the loader, the key type, the value type and the sql query.
/// **Note**: The sql query has to return the key as ``ka`` and the value as ``val``.
///
/// # Example
///
/// ```rust
/// crate::basic_loader!(
///     ChallengeLoaderByName,
///     String,
///     Challenge,
///     r#"SELECT "name" AS ka, "id" AS val FROM ctf_challenges WHERE "name" = ANY($1)"#
/// );
/// ```

#[macro_export]
macro_rules! basic_loader {
    ($name:ident, $key:ty, $val:ty, $query:literal) => {
        $crate::loader_struct!($name);

        #[async_trait::async_trait]
        impl async_graphql::dataloader::Loader<$key> for $name {
            type Value = $val;
            type Error = std::sync::Arc<sqlx::Error>;

            async fn load(
                &self,
                keys: &[$key],
            ) -> Result<std::collections::HashMap<$key, Self::Value>, Self::Error> {
                use futures::stream::TryStreamExt;
                Ok(sqlx::query!($query, keys)
                    .fetch(&self.pool)
                    .map_ok(|record| (<$key>::from(record.ka), <$val>::from(record.val)))
                    .try_collect()
                    .await?)
            }
        }
    };
}
