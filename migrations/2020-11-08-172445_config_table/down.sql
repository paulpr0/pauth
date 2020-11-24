-- This file should undo anything in `up.sql`
drop table if exists pauth.default_config;
drop table if exists pauth.user_config;
drop table if exists pauth.config;
drop table if exists pauth.login_history;
drop table if exists pauth.auth_history;
drop table if exists pauth.user_history;
drop table if exists pauth.source;