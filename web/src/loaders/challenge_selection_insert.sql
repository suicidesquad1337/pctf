-- uses the temporary table in the 
INSERT INTO challenge_selections (
    "id",
    "name",
    "type",
    "short_description",
    "long_description",
    "hints",
    "created_at",
    "active"
) VALUES (
    -- unnest hackery
    unnest($1::UUID[]),
    unnest($2::BOOL[]),
    unnest($3::BOOL[]),
    unnest($4::BOOL[]),
    unnest($5::BOOL[]),
    unnest($6::BOOL[]),
    unnest($7::BOOL[]),
    unnest($8::BOOL[])
)
