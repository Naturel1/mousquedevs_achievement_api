-- Add unique constraint to the username column in the users table
ALTER TABLE users ADD CONSTRAINT users_username_unique UNIQUE (username);
