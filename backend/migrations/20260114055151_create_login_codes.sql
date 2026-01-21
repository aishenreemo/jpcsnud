-- Add migration script here

-- Create table to store short-lived login codes
CREATE TABLE IF NOT EXISTS "LoginCodes" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES "Users"(user_id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    expires_at BIGINT NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS login_codes_user_idx ON "LoginCodes" (user_id);
CREATE INDEX IF NOT EXISTS login_codes_expires_idx ON "LoginCodes" (expires_at);
