-- stores the ctf challenges
CREATE TABLE challenges (
    -- unique id of this challenge
    -- for `gen_random_uuid()` ensure that the `pgcrypto` extension is present
    -- or else enable it with `CREATE EXTENSION EXISTS pgcrypto`
    "id" UUID NOT NULL UNIQUE DEFAULT gen_random_uuid() PRIMARY KEY,
    -- the name of the challenge displayed to the user
    "name" VARCHAR(32) NOT NULL UNIQUE CHECK (char_length(name) > 0),
    -- the type of this challenge
    "type" CHALLENGE_TYPE NOT NULL,
    -- a short description (max length 120 chars)
    "short_description" VARCHAR(120) CHECK (char_length(short_description) > 0),
    -- the long description of the challenge
    "long_description" TEXT CHECK (char_length(long_description) > 0),
    -- hints for the challenge to help players
    "hints" TEXT CHECK (char_length(hints) > 0),
    -- then this challenge was created
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    -- if the challenge is being deployed by the supervisor
    "active" BOOLEAN NOT NULL DEFAULT false
)