-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.

DROP FUNCTION IF EXISTS diesel_manage_updated_at(_tbl regclass);
DROP FUNCTION IF EXISTS diesel_set_updated_at();



ALTER TABLE IF EXISTS ONLY pauth.user_login_tokens DROP CONSTRAINT IF EXISTS user_login_tokens_user_id_fkey;
ALTER TABLE IF EXISTS ONLY pauth.pw_reset DROP CONSTRAINT IF EXISTS pw_reset_user_id_fkey;
DROP INDEX IF EXISTS pauth.users_email_idx;
DROP INDEX IF EXISTS pauth.users_chosen_name_idx;
DROP INDEX IF EXISTS pauth.user_login_tokens_user_id_fkey;
DROP INDEX IF EXISTS pauth.pw_reset_user_id_fkey;
ALTER TABLE IF EXISTS ONLY pauth.users DROP CONSTRAINT IF EXISTS users_pkey;
ALTER TABLE IF EXISTS ONLY pauth.user_login_tokens DROP CONSTRAINT IF EXISTS user_login_tokens_pkey;
ALTER TABLE IF EXISTS ONLY pauth.pw_reset DROP CONSTRAINT IF EXISTS pw_reset_pkey;
ALTER TABLE IF EXISTS pauth.users ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS pauth.user_login_tokens ALTER COLUMN id DROP DEFAULT;
ALTER TABLE IF EXISTS pauth.pw_reset ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE IF EXISTS pauth.users_id_seq;
DROP TABLE IF EXISTS pauth.users;
DROP SEQUENCE IF EXISTS pauth.user_login_tokens_id_seq;
DROP TABLE IF EXISTS pauth.user_login_tokens;
DROP SEQUENCE IF EXISTS pauth.pw_reset_id_seq;
DROP TABLE IF EXISTS pauth.pw_reset;
DROP EXTENSION IF EXISTS pgcrypto;
DROP EXTENSION IF EXISTS plpgsql;
DROP SCHEMA IF EXISTS pauth;

DROP USER IF EXISTS pauth_admin;
