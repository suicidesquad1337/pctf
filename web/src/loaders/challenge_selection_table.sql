-- this table stores a bool for each row 
-- each column in this table corresponds to the column in the `challenges` table
-- if true, the column is requested
CREATE TEMPORARY TABLE challenge_selections (
    "id" UUID PRIMARY KEY,
    "name" BOOL NOT NULL,
    "type" BOOL NOT NULL,
    "short_description" BOOL NOT NULL,
    "long_description" BOOL NOT NULL,
    "hints" BOOL NOT NULL,
    "created_at" BOOL NOT NULL,
    "active" BOOL NOT NULL
-- this table is only valid for one transaction
) ON COMMIT DROP
