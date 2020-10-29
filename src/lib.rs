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
    change_details,
    delete_user,
    validate_pw_reset,
    AuthenticatedID,
    LoginResult,
    UserActionFailure,
    AddUserResult,
    DeleteUserResult,
    ChangeDetailsResult,
    Reason,
    UserUpdate
};

