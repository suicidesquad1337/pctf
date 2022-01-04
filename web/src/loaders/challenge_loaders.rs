use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    basic_loader,
    challenge::{Challenge, ChallengeType},
    loader_struct,
};
use async_graphql::dataloader::Loader;
use chrono::{DateTime, Utc};
use futures::prelude::stream::StreamExt;
use uuid::Uuid;

mod loader_front;
pub use loader_front::*;

// used to check if a challenge exists in the database
basic_loader!(
    ChallengeLoaderByID,
    Uuid,
    Challenge,
    r#"SELECT "id" AS ka, "id" AS val FROM challenges WHERE "id" = ANY($1)"#
);

loader_struct!(ChallengeLoader);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
/// Enum used to select a column for a row
enum ChallengeFieldSelection {
    Name,
    Type,
    ShortDescription,
    LongDescription,
    Hints,
    CreatedAt,
    Active,
}

/// Corresponding responses to [`ChallengeFieldSelection`]
#[derive(Debug, Clone)]
enum ChallengeField {
    Name(String),
    Type(ChallengeType),
    ShortDescription(String),
    LongDescription(String),
    Hints(String),
    CreatedAt(DateTime<Utc>),
    Active(bool),
}

#[derive(Debug, sqlx::FromRow)]
/// used to have a typed response with [`sqlx::query_as`] (the function, not the macro)
struct ChallengeResponseRow {
    id: Uuid,
    name: Option<String>,
    #[sqlx(rename = "type")]
    typ: Option<ChallengeType>,
    short_description: Option<String>,
    long_description: Option<String>,
    hints: Option<String>,
    created_at: Option<DateTime<Utc>>,
    active: Option<bool>,
}

#[async_trait]
impl Loader<(Challenge, ChallengeFieldSelection)> for ChallengeLoader {
    type Value = ChallengeField;
    type Error = Arc<sqlx::Error>;
    async fn load(
        &self,
        keys: &[(Challenge, ChallengeFieldSelection)],
    ) -> Result<HashMap<(Challenge, ChallengeFieldSelection), Self::Value>, Self::Error> {
        let mut collected: HashMap<Challenge, HashSet<ChallengeFieldSelection>> = HashMap::new();
        for (challenge, selection) in keys {
            // when https://github.com/rust-lang/rust/issues/65225 is stabilized,
            // this could be replaced with Entry::insert_entry (currently this
            // method is called `insert` and is behind the `entry_insert` feature flag).
            // get the selections for the `challenge` or insert an empty one and modify that
            collected.entry(*challenge).or_default().insert(*selection);
        }

        // the size of the rows
        let to_allocate = collected.len();

        // for each column + primary key, we need one Vec
        let mut id = Vec::with_capacity(to_allocate);
        let mut name = Vec::with_capacity(to_allocate);
        let mut typ = Vec::with_capacity(to_allocate);
        let mut s_desc = Vec::with_capacity(to_allocate);
        let mut l_desc = Vec::with_capacity(to_allocate);
        let mut hints = Vec::with_capacity(to_allocate);
        let mut created_at = Vec::with_capacity(to_allocate);
        let mut active = Vec::with_capacity(to_allocate);

        for (challenge, selections) in collected {
            id.push(*challenge);
            name.push(selections.contains(&ChallengeFieldSelection::Name));
            typ.push(selections.contains(&ChallengeFieldSelection::Type));
            s_desc.push(selections.contains(&ChallengeFieldSelection::ShortDescription));
            l_desc.push(selections.contains(&ChallengeFieldSelection::LongDescription));
            hints.push(selections.contains(&ChallengeFieldSelection::Hints));
            created_at.push(selections.contains(&ChallengeFieldSelection::CreatedAt));
            active.push(selections.contains(&ChallengeFieldSelection::Active));
        }

        let mut transaction = self.pool.begin().await?;

        sqlx::query_file!("src/loaders/challenge_selection_table.sql")
            .execute(&mut transaction)
            .await?;

        // the following two queries can't be checked at compile time because
        // they use a temporary table which is only available during the transaction
        sqlx::query(include_str!("challenge_selection_insert.sql"))
            .bind(&id)
            .bind(&name)
            .bind(&typ)
            .bind(&s_desc)
            .bind(&l_desc)
            .bind(&hints)
            .bind(&created_at)
            .bind(&active)
            .execute(&mut transaction)
            .await?;

        let mut rows = sqlx::query_as::<_, ChallengeResponseRow>(include_str!(
            "challenge_selection_fetch.sql"
        ))
        .fetch(&mut transaction);

        let mut response = HashMap::new();
        while let Some(row) = rows.next().await {
            let row = row?;
            let id: Challenge = row.id.into();
            if let Some(name) = row.name {
                response.insert(
                    (id, ChallengeFieldSelection::Name),
                    ChallengeField::Name(name),
                );
            }
            if let Some(typ) = row.typ {
                response.insert(
                    (id, ChallengeFieldSelection::Type),
                    ChallengeField::Type(typ),
                );
            }
            if let Some(s_desc) = row.short_description {
                response.insert(
                    (id, ChallengeFieldSelection::ShortDescription),
                    ChallengeField::ShortDescription(s_desc),
                );
            }
            if let Some(l_desc) = row.long_description {
                response.insert(
                    (id, ChallengeFieldSelection::LongDescription),
                    ChallengeField::LongDescription(l_desc),
                );
            }
            if let Some(hints) = row.hints {
                response.insert(
                    (id, ChallengeFieldSelection::Hints),
                    ChallengeField::Hints(hints),
                );
            }
            if let Some(created_at) = row.created_at {
                response.insert(
                    (id, ChallengeFieldSelection::CreatedAt),
                    ChallengeField::CreatedAt(created_at),
                );
            }
            if let Some(active) = row.active {
                response.insert(
                    (id, ChallengeFieldSelection::Active),
                    ChallengeField::Active(active),
                );
            }
        }
        drop(rows);

        // finish the transaction so the temporary table is deleted
        transaction.commit().await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use async_graphql::dataloader::Loader;
    use uuid::Uuid;

    use crate::challenge::{Challenge, ChallengeType};

    use super::{ChallengeField, ChallengeFieldSelection, ChallengeLoader};

    #[tokio::test]
    async fn loader_load() {
        let pool = sqlx::PgPool::connect(
            &std::env::var("PCTF_DB_URI").expect("test needs database connection"),
        )
        .await
        .expect("cannot connect to database");

        // now insert some test data
        let a = Uuid::from_str("dcbebb24-c149-400c-b09f-3d6839d10900").unwrap();
        let b = Uuid::from_str("f0eb77dd-3766-4569-ab44-c385059a3ed3").unwrap();
        let c = Uuid::from_str("cfe1ae10-e1e0-4149-8627-3e08770a782e").unwrap();

        let a_name = "a ctf challenge";
        let b_name = "pickle_rick";
        let c_name = "log4j";

        let a_type = ChallengeType::Crypto;
        let b_type = ChallengeType::Web;
        let c_type = ChallengeType::Pwn;

        let a_short_desc = "Bayern raus aus Deutschland (Spass)";
        let b_short_desc = "pickle rick from rick and morty and a bit of python";
        // no short description for c

        let a_long_desc = concat!(
            "Hass Frau, du nichts, ich Mann\n",
            "Blase, bis du kotzt, aber kotz auf mein'n Schwanz\n",
            "Hass Frau, du nichts, ich Mann\n",
            "Ich fick in dein'n Arsch und danach leckst du ab\n",
            "Hass Frau, du nichts, ich Mann\n",
            "Fick mich und halt dein Maul\n",
        );
        let b_long_desc = concat!(
            "Pickerick was a challenge in a previous ctf. ",
            "The player had to use a template injection. ",
            "With that template injection, the player was able to steal the ",
            "secret used for token signing. ",
            "Since the token was signed and therefore trusted, the server used ",
            "pythons pickle module to load a user from that token, this enabled ",
            "RCE. Add a misconfigured sudo and there you go!"
        );

        let a_hints = "SXTN";

        sqlx::query!(
            r#"
        INSERT INTO challenges (
            "id",
            "name",
            "type",
            "short_description",
            "long_description",
            "hints"
        ) VALUES ($1, $2, $3, $4, $5, $6), 
        ($7, $8, $9, $10, $11, null),
        ($12, $13, $14, null, null, null)
        ON CONFLICT DO NOTHING
        "#,
            a,
            a_name,
            // just needed for sqlx, dont ask me
            // see https://github.com/launchbadge/sqlx/issues/1004#issuecomment-764964043
            a_type as _,
            a_short_desc,
            a_long_desc,
            a_hints,
            b,
            b_name,
            b_type as _,
            b_short_desc,
            b_long_desc,
            c,
            c_name,
            c_type as _
        )
        .execute(&pool)
        .await
        .expect("failed to insert testdata");

        let loader = ChallengeLoader::new(pool.clone());

        let a_challenge: Challenge = a.into();
        let b_challenge: Challenge = b.into();
        // we dont request c to proof that only requested data is returned
        let c_challenge: Challenge = c.into();

        let keys = &[
            (a_challenge, ChallengeFieldSelection::Name),
            (a_challenge, ChallengeFieldSelection::LongDescription),
            (a_challenge, ChallengeFieldSelection::CreatedAt),
            (a_challenge, ChallengeFieldSelection::Type),
            (b_challenge, ChallengeFieldSelection::ShortDescription),
            // should be not in the result set since it is null
            (b_challenge, ChallengeFieldSelection::Hints),
        ][..];
        let result = loader.load(keys).await.unwrap();

        assert!(result
            .get(&(b_challenge, ChallengeFieldSelection::Hints))
            .is_none());

        let a_type_resp = if let ChallengeField::Type(typ) = result
            .get(&(a_challenge, ChallengeFieldSelection::Type))
            .unwrap()
        {
            typ
        } else {
            panic!()
        };
        assert_eq!(a_type_resp, &a_type);

        // ensure nothing about c is in the response
        for ((i, _), _) in result {
            if i == c_challenge {
                panic!("c should not be in the response")
            }
        }

        sqlx::query!(
            r#"
        DELETE FROM challenges WHERE "id" = ANY($1)
        "#,
            &[a, b, c][..]
        )
        .execute(&pool)
        .await
        .expect("cannot delete test data");
    }
}
