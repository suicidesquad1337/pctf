CREATE TABLE ctf_challenges (
    -- unique id of this challenge
    "id" UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4() PRIMARY KEY,
    -- the name of the challenge displayed to the user
    "name" VARCHAR(32) NOT NULL UNIQUE CHECK (char_length(name) > 0),
    -- a short description (max length 120 chars)
    "short_description" VARCHAR(120) CHECK (char_length(short_description) > 0),
    -- the long description of the challenge
    "long_description" TEXT CHECK (char_length(long_description) > 0),
    -- hints for the challenge
    "hints" TEXT CHECK (char_length(hints) > 0),
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    "active" BOOLEAN NOT NULL DEFAULT false
)