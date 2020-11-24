-- Add the config table for pauth
-- Can set defaults in sql nicely here
create table pauth.config (
    id serial primary key not null,
--    user_id integer references pauth.users(id),
    config_key varchar,
    config_value varchar
--    constraint user_and_key_unique unique (user_id, config_key)
);
-- entries which form the default config
create table pauth.default_config (
    id serial primary key not null,
    config_id integer references pauth.config(id)
);

--user specific overrides
create table pauth.user_config (
    id serial primary key not null,
    user_id integer references pauth.users(id),
    config_id integer references pauth.config(id),
    constraint user_and_config_unique unique (user_id, config_id)
);

create table pauth.source(
    id serial primary key not null,
    ip inet,
    mac macaddr,
    identifier text
);

-- history table for logins
--  route is an optional field for use when
--  users could log in via different routes (app, website, cmd)
create table pauth.login_history(
    id serial primary key not null,
    user_id integer references pauth.users(id),
    login_time timestamp without time zone default now(),
    source integer references pauth.source(id),
    route text
);

create table pauth.auth_history(
  id serial primary key not null,
  user_id integer references pauth.users(id),
  auth_time timestamp without time zone default now(),
  source integer references pauth.source(id),
  route text
);

-- store the old details with change time where they have changed
-- unchanged columns should be null.
create table pauth.user_history(
    id serial primary key not null,
    user_id integer references pauth.users(id),
    change_time timestamp without time zone default now(),
    old_chosen_name varchar,
    old_email varchar,
    old_pass text
);

-- the with cfg as... consruct inserts a reference to the newly created
-- config into pauth.default_config
with cfg as (insert into pauth.config(config_key, config_value)
    values ('max failed logins per source', '5')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('max failed logins per source reset time minutes', '1440') -- 1 day
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('token validity minutes', '1440000') --1000 days
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('expire token if not used for minutes', '144000') --100 days
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('password reset validity minutes', '720' ) -- 12 hours
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('keep login history', 'true')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('keep reset history', 'true')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('keep user change history', 'true')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('login history retention days', '365')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('reset history retention days', '365')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;

with cfg as (insert into pauth.config(config_key, config_value)
    values ('user change history retention', '365')
    returning id as cfg_id)
insert into pauth.default_config(config_id) select cfg_id from cfg;









