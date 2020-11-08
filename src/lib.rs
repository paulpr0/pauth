//! Pauth provides simple, standardised authentication using postgres.
//! Add, update, and delete users. Authenticate them, store and verify authentication tokens,
//! do password resets.
//!
//! Users must have at least either an unique username or an unique email address to identify them.
//! It is fine to have both, and to use either as the means of identifying the user. Users must additionally
//! have a password. No stipulation of password strength is made by pauth (if you want to ensure
//! passwords meet a particular standard, you should verify this before allowing them to be set in pauth)
//!
//! Authenticate users with pauth using either their username or email, and their password. When
//! a user is successfully authenticated, an AuthenticatedID is returned. The AuthenticatedID
//! can be used to authenticate the user (like a cookie) without having to re-provide credentials
//!
//! Password resets can be requested by supplying either a username or email as an identifier.
//! If the identifier exists, a token is generated which can then be used to reset the password.
//! Implementing systems should send the token via an alternative route (such as to the registered
//! email of the user). The token can then be used to authenticate and allow changing the user
//! password (or other credentials). Password reset tokens are stored separately from ordinary
//! authentication tokens so can have different rules (such as expiry).
//!
//! ### Getting Started
//!
//! Define the database you want to use by setting the environment vairable DATABASE_URL.
//! When you first call run_db_migrations(), a schema "pauth" will be created which contains
//! all of the tables needed for pauth.
//!
//! All passwords and tokens are stored in encrypted form using [pgcrypto](https://www.postgresql.org/docs/current/pgcrypto.html).
//!
//! ### Still to do
//! pauth is still at an early stage but is under [active development](https://github.com/paulpr0/pauth)
//! + expiry for cookies. We store created date and last used date.
//! + Archiving and deleting. Some systems need a record of who logged in and when. We also need to archive
//! or delete old authentication ids.
//! + count and act on multiple authentication failures.
//! + All config stored as a table in pauth. This should be per user, possibly email domain,
//! and global - whichever is more specific applies. This allows for different settings perhaps for
//! VIP users or sensitive accounts.
//!
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod models;
mod pauth_error;
mod schema;

pub use models::{
    login,
    check_id,
    check_id_and_password,
    add_user,
    get_user,
    change_details,
    delete_user,
    validate_pw_reset,
    AuthenticatedID,
    LoginResult,
    UserActionFailure,
    AddUserResult,
    DeleteUserResult,
    ChangeDetailsResult,
    UserActionFailureReason,
    UserUpdate
};

/// Run any database migrations (schema updates). From 1.0 onwards, any schema changes
/// will constitute at least a minor version upgrade, and any breaking changes trigger
/// a major version upgrade.
///
/// It is suggested that this is run either as part of a build.rs in your project, or
/// called on startup, as it will do nothing if there are no upgrades to perform. If you
/// need more fine grained control of updating schemas, see the migrations source folder
/// for details of all upgrades.
///
/// If running on a production database, I suggest you check the schema updates before running
/// as whilst they should always be reasonable, I don't know your setup and our definitions
/// of 'reasonable' could differ.
pub fn run_db_migrations() {
    db::init()
}
