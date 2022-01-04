-- a enum to define the type of challenge
CREATE TYPE challenge_type AS ENUM (
    'pwn',
    'web',
    'crypto',
    'reversing'
)