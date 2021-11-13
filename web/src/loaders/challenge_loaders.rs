use crate::{
    basic_loader,
    challenge::{Challenge, ChallengeType},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

basic_loader!(
    ChallengeLoaderByName,
    String,
    Challenge,
    r#"SELECT "name" AS ka, "id" AS val FROM ctf_challenges WHERE "name" = ANY($1)"#
);

basic_loader!(
    ChallengeNameLoaderByID,
    Uuid,
    String,
    r#"SELECT "id" AS ka, "name" AS val FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    ShortDescriptionLoaderByID,
    Uuid,
    Option<String>,
    r#"SELECT "id" AS ka, "short_description" AS val FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    LongDescriptionLoaderByID,
    Uuid,
    Option<String>,
    r#"SELECT "id" AS ka, "long_description" AS val FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    IsActiveLoaderByID,
    Uuid,
    bool,
    r#"SELECT "id" AS ka, "active" AS val FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    CreatedAtLoaderByID,
    Uuid,
    DateTime<Utc>,
    r#"SELECT "id" AS ka, "created_at" AS val FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    ChallengeTypeLoaderByID,
    Uuid,
    ChallengeType,
    r#"SELECT "id" AS ka, "challenge_type" AS "val!: ChallengeType" FROM ctf_challenges WHERE "id" = ANY($1)"#
);

basic_loader!(
    ChallengeHintsLoaderByID,
    Uuid,
    Option<String>,
    r#"SELECT "id" AS ka, "hints" AS "val" FROM ctf_challenges WHERE "id" = ANY($1)"#
);
