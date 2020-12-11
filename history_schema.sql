/**
  todo - login function should be in sql - and should fail if
  there are multiple accounts with an email and email is used
  (unless there is a username match). Multiple accts with one
  email is not forbidden, but multiple accounts with the same username
  is. Is this right? do we want this to really be pushed out to
  user space if multiple user accounts with same email is a thing?

  table old_usernames which is just a list of deleted/changed usernames
  -- allow a user to change back to one they had by checking their
  history. old usernames should have either a daterange or at least a
  retired date.

  list of workflows:

  -create user
    --insert into users
  verify email (pw reset with initial pw as null, then continue)
  -login
  -authenticate
  reset_request
  reset_attempt
  change_password
  logout
  logout_everywhere
  //don't want a logout_everywhere_else as it could be misused
  //if you gain access to a machine, log out all other then do stuff
  //if you logout others you need to log yourself back in

  modify
  delete

  reinstate_old_username
  show_login_history
  show_auth_history

  admin_change_username
  admin_change_password
  admin_nullify_password
  admin_change_email
  admin_remove_old_username
  admin_delete_user_history_excluding/including_usernames
  admin_force_logout_user
  admin_cancel_reset_tokens


 */
begin;

create type pauth.hist_action as enum (
    'create',
    'change_chosen_name',
    'change_email',
    'change_password',
    'login',
    'authenticate',
    'request_reset',
    'reset',
    'delete'
    );

create table pauth.hist_user (
    date timestamp not null default now(),
    pauth_user integer references pauth.users(id),
    action pauth.hist_action not null,
    old_user_data jsonb,
    new_user_data jsonb
);
comment on table pauth.hist_user is
    'Stores history of all changes to users and actions performed on users. '
    'For a list of actions stored, see the pauth.hist_action enum. '
    'Some actions (such as change_*) will populate either the old_user_data '
    'the new_user_data, or both. This data is stored as a jsonb representation '
    'of the user record.';

--drop type if exists pauth.failure_type;
create type pauth.failure_type as enum (
    'auth',
    'login',
    'reset'
);

create table pauth.old_chosen_names (
    chosen_name varchar primary key not null unique,
    retired_at timestamp not null default now(),
    user_id integer references pauth.users(id)
);

create table pauth.failures (
  date timestamp not null default now(),
  pauth_user integer references pauth.users(id),
  failure pauth.failure_type not null,
  source integer references pauth.source(id)
);

-- add trigger for create user
create or replace function pauth.hist_create_user()
    returns trigger
    language plpgsql
as $$
begin
    if new.email in (select chosen_name from pauth.users) then
        raise exception 'email %s is used as a chosen name', new.email;
    end if;
    if new.chosen_name in (select email from pauth.users) then
        raise exception 'chosen name %s is an email already associated with an account', chosen_name;
    end if;
    insert into pauth.hist_user(pauth_user, action, new_user_data)
    values (new.id, 'create', to_jsonb(new));
    return new;
end;
$$;

create trigger hist_create_user_trigger
    after insert
    on pauth.users
    for each row
    execute procedure pauth.hist_create_user();


-- add trigger for modify user
create or replace function pauth.hist_change_user()
    returns trigger
    language plpgsql
as $$
begin
    if new.chosen_name is distinct from old.chosen_name then
        if new.chosen_name in (select email from pauth.users where id !=new.id) then
            raise exception 'chosen name %s is an email already associated with an account', chosen_name;
        end if;
        insert into pauth.hist_user(pauth_user, action, old_user_data, new_user_data)
        values (new.id, 'change_chosen_name', to_jsonb(old), to_jsonb(new));
        insert into pauth.old_chosen_names(chosen_name,  user_id) values
        (old.chosen_name, new.id);
    end if;
    if new.email is distinct from old.email then
        if new.email in (select chosen_name from pauth.users where id!=new.id) then
            raise exception 'email %s is used as a chosen name', new.email;
        end if;
        insert into pauth.hist_user(pauth_user, action, old_user_data, new_user_data)
        values (new.id, 'change_email', to_jsonb(old), to_jsonb(new));
    end if;
    if new.pass_hash is distinct from old.pass_hash then
        insert into pauth.hist_user(pauth_user, action, old_user_data, new_user_data)
        values (new.id, 'change_password',to_jsonb(old), to_jsonb(new));
    end if;
    return new;
end;
$$;

create trigger hist_change_user_trigger
    after update
    on pauth.users
    for each row
    execute procedure pauth.hist_change_user();

-- add trigger for delete user
create or replace function pauth.hist_delete_user()
    returns trigger
    language plpgsql
as $$
    begin
        update pauth.old_chosen_names
        set user_id =null where user_id = old.id;
        update pauth.hist_user
        set pauth_user = null where pauth_user = old.id;
        insert into pauth.old_chosen_names(chosen_name) values (old.chosen_name);
        update old_chosen_names set user_id=null where user_id=old.id;
        insert into pauth.hist_user(action, old_user_data)
        values ('delete', to_jsonb(old));
    end;
$$;
create trigger hist_delete_user_trigger
    before delete
    on pauth.users
    for each row
    execute procedure pauth.hist_delete_user();

/**
  if the user details match, put an entry into the auth table and the user
  hist table, and return the uuid, otherwise put an entry in failures and
  return null
 */
create or replace function pauth.login(
    in name_or_email varchar,
    in pass varchar,
    in source_id integer default null,
    out login_token text)
    language plpgsql
as $$
declare
    uuid text := pauth.gen_random_uuid();
    userid integer;
begin
    insert into pauth.user_login_tokens (user_id, token)
    select id, pauth.crypt(uuid, 'bf') from pauth.users
    where (chosen_name = name_or_email or email = name_or_email)
        and pass_hash = pauth.crypt(pass, pass_hash)
    limit 1
    returning user_id into userid;
    if userid is null then
        insert into pauth.failures (pauth_user, failure, source)
        values
        (
            (
                select id
                from pauth.users
                where chosen_name = name_or_email
                or email = name_or_email
            ),
            'login'::pauth.failure_type, source_id
         );
    else
        insert into pauth.hist_user(pauth_user, action)
        values (userid, 'login');
    end if;
    select uuid into login_token;
end
$$;

create or replace function pauth.authenticate(
    in name_or_email varchar,
    in uuid_hash text,
    in source_id integer default null,
    out status boolean
) language plpgsql
as $$
declare userid integer;
begin
    select
           users.id
    from
         pauth.users join pauth.user_login_tokens on users.id = user_login_tokens.user_id
    where
          (chosen_name = name_or_email or email = name_or_email)
            and token = pauth.crypt(uuid_hash, pass_hash)
    limit 1
    into
        userid;
    if userid is null then
        insert into pauth.failures(pauth_user, failure, source)
        values (
                (
                   select
                           id
                   from
                        pauth.users
                   where
                         chosen_name = name_or_email
                            or email = name_or_email
                ),
                'auth',
                source_id

               );
        return false;
    end if;
    insert
        into pauth.hist_user
            (pauth_user, action)
        values (userid, 'authenticate') ;
    return true;
end
$$;


insert into pauth.users (chosen_name, email, pass_hash ) values
    ('p1', 'p1@pr0.uk', pauth.crypt('pass', pauth.gen_salt('bf'))),
    ('p2', 'p2@pr0.uk', pauth.crypt('pass', pauth.gen_salt('bf'))),
    ('p3', 'p3@pr0.uk', pauth.crypt('pass', pauth.gen_salt('bf')));
\echo "should log in and give a token"
select pauth.login('p1','pass');
\echo "should fail to log in, returning null with p1 entry in failures"
select pauth.login('p1', 'not pass');
select * from pauth.failures;
\echo "should fail to login with no idea of user"
select pauth.login('ppp', 'pass');
select * from pauth.failures;

update pauth.users set chosen_name = 'p1a' where chosen_name='p1';
--select pg_sleep(1);
update pauth.users set email = 'p1a@pr0.uk' where chosen_name='p1a';
update pauth.hist_user set date = now()+'1 hour' where new_user_data->> 'chosen_name' = 'p1a';
--select pg_sleep(1);
update pauth.users set chosen_name = 'p1' where chosen_name='p1a';
update pauth.hist_user set date = now()+'2 hour' where old_user_data->> 'chosen_name' = 'p1a';

--select * from pauth.hist_user;
select * from pauth.users;

-- select all usernames for a user
create function pauth.get_chosen_names_for(
    in current_name varchar,
    out name varchar
)
returns setof varchar
language SQL as $$
select (
    select (old_user_data ->> 'chosen_name') as name
    )
from
     pauth.hist_user
where
      pauth_user = (select id from pauth.users where chosen_name =current_name)
      and
      action = 'change_chosen_name'
      and
      old_user_data is not null
union distinct
select
    chosen_name as name
from
     pauth.users
where
    chosen_name = current_name
;
$$;

\echo "Usernames and dates set";
select old_user_data->> 'chosen_name', date from pauth.hist_user;

select
    old_user_data ->> 'chosen_name' as name,
           tsrange(date, coalesce(lead(date) over (partition by pauth_user,action order by date), now()+'3 hour')::timestamp,
               '[]') as validity


from
     pauth.hist_user where old_user_data is not null;

--chosen names and date ranges
create function pauth.get_chosen_names_over_time(
    in current_name varchar,
    out name varchar,
    out from_to tsrange
)
returns setof record
language SQL as $$
select
    old_user_data ->> 'chosen_name' as name,
           tsrange(date, coalesce(lead(date) over (partition by pauth_user order by date), now())::timestamp,
               '[)') as validity
from
     pauth.hist_user
where
      pauth_user = (select id from pauth.users where chosen_name =current_name)
      and
    (action = 'change_chosen_name' or action = 'create')
      and
      old_user_data is not null
$$;


select pauth.get_chosen_names_for('p1');

select pauth.get_chosen_names_for('p2');


rollback;
