Pauth is intended to provide an authentication API where the user credentials are stored in a postgres database.

It has features for password reset and authentication tokens (e.g. a session cookie in web world).

The current status is very much alpha - it compiles and the tests pass, but the API is not stable, and there are features not present that would likely preclude its use for real work.

Passwords and password reset tokens are stored using postgres's crypt function to provide well salted, secure passwords. At present, login tokens are not, which would provide anyone with access to the database with a way of impersonating a user, so this will be fixed soon.

At present, there is no failed login count or timed delay, but these features are planned.

## Getting started

Create a database if you don't already have one:
```
psql template1
```
then inside the shell:
```
create database pauth;
create user someuser with password 'somepass';
```
set the environment variable DATABASE_URL. If you used the example setup above, you would se it to:
```
postgres://someuser:somepass@localhost:5432/pauth
```
The first time you use the library (for example running the create_user_log_in_pw_reset_delete() test), we will run diesel migrations so you should see a new schema in your database named pauth at that point
the pauth schema will be owned by a new user pauth_admin which has the password 'pauth_admin_password' by default. To change this, either 
make the chanege in postgres and change the database URL, or modify the relevant lines in up.sql.

See the test reate_user_log_in_pw_reset_delete in models.rs for a walkthrough of the different API calls.

## Functional Issues to fix (in rough priority order):
    - Count failed logins per 'source'
    - Clean up the API and write sensible example code
    - improve the general documentation
    - write doc tests for each public API call
   
## Non-functional issues to fix:
    - split the queries from the higher level API so that users can have a layer where they provide their own db connection
    - write better and more tests 
    
## Why?
This project exists becuase I frequently need to create something to handle users and authenticate them for toy systems I create.
In the past these have always been ad-hoc, and the user authentication has been tied to other attributes relevant to the user of that particular system.
Handling the different interactions a user has with authentication is fairly standard, and boring to re-implement, so the aim
is to implement it once here in a sensible way so that all my future pet rust projects *(and maybe yours too)* can get on with solving more interesting problems 
    
## Contributing
Any and everything is welcome. Questions are welcome. Constructive criticism is very welcome. If you want to make bigger changes (e.g. allowing use of a different rdbms, adding oauth or similar) then please talk to me before sending a PR. 

