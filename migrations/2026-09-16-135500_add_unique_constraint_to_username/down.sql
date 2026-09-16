-- Remove unique constraint from the username column in the users table
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_username_unique;
