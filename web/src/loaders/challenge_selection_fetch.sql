SELECT
    -- id / primary key is always needed
    challenges.id,
    -- now some CASE hackery to select each column only if the bool for the id
    -- in the `challenge_selections` table is true
    CASE
        WHEN challenge_selections.name
            THEN challenges.name
        END "name",
    CASE
        WHEN challenge_selections.type
            THEN challenges.type
        END "type",
    CASE
        WHEN challenge_selections.short_description
            THEN challenges.short_description
        END "short_description",
    CASE
        WHEN challenge_selections.long_description
            THEN challenges.long_description
        END "long_description",
    CASE
        WHEN challenge_selections.hints
            THEN challenges.hints
        END "hints",
    CASE
        WHEN challenge_selections.created_at
            THEN challenges.created_at
        END "created_at",
    CASE
        WHEN challenge_selections.active
            THEN challenges.active
        END "active"
FROM challenges
    INNER JOIN challenge_selections ON (challenge_selections.id = challenges.id)
WHERE challenges.id = ANY(SELECT id FROM challenge_selections)
