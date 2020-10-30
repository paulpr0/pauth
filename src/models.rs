use super::db;
use super::schema::pauth::pw_reset;
use super::schema::pauth::user_login_tokens;
use super::schema::pauth::users;
use crate::pauth_error::ApplicationError;
use chrono::{NaiveDateTime, Utc};
use diesel::expression::exists::exists;
use diesel::sql_types::Text;
use diesel::{prelude::*, select, RunQueryDsl};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;
use uuid::Uuid;

#[derive(Queryable, QueryableByName, Identifiable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub chosen_name: String,
    pub email: String,
    pass_hash: String,
    pub last_login: NaiveDateTime,
}

#[derive(AsChangeset, Default)]
#[table_name = "users"]
pub struct UserUpdate {
    chosen_name: Option<String>,
    email: Option<String>,
    pass_hash: Option<String>,
}
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "user_login_tokens"]
pub struct UserLoginToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created: NaiveDateTime,
    pub last_used: NaiveDateTime,
}

#[derive(Queryable, QueryableByName, Identifiable, PartialEq, Debug)]
#[table_name = "users"]
pub struct UserID {
    pub id: i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "pw_reset"]
pub struct PasswordReset {
    pub id: i32,
    pub user_id: i32,
    pub user_token_hash: String,
    pub expires: NaiveDateTime,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "user_login_tokens"]
pub struct NewCookie {
    pub user_id: i32,
    pub token: String,
}

/*
Login methods. If a cookie is passed with user email,
we try to log in with that cookie using try_get, returning a user.
If there is no cookie (or it fails), login with password and get a
cookie to set.
*/

pub type AuthenticatedID = NewCookie;
#[derive(Debug, PartialEq)]
pub enum LoginResult {
    LoggedIn(AuthenticatedID),
    AuthenticationFailure,
}

pub type Reason = String;

#[derive(Debug)]
pub enum UserActionFailure {
    UsernameExists,
    EmailExists,
    UsernameInvalid(Reason),
    EmailInvalid(Reason),
    PasswordInvalid(Reason),
}

#[derive(Debug)]
pub enum AddUserResult {
    Added(AuthenticatedID),
    NotAdded(Vec<UserActionFailure>),
}

#[derive(Debug)]
pub enum DeleteUserResult {
    Deleted,
    AuthFailure,
    NotFound,
}

#[derive(Debug)]
pub enum ChangeDetailsResult {
    Changed,
    NotChanged(Vec<UserActionFailure>),
    AuthenticationFailure,
}
sql_function! {
    #[sql_name="pauth.crypt"]
    fn crypt(pw: Text, salt: Text) -> Text;
}

sql_function! {
    #[sql_name="pauth.gen_salt"]
    fn gen_salt(type_: Text) -> Text;
}

fn encrypt_password(password: &str) -> Result<String, ApplicationError> {
    let conn = db::connection()?;
    Ok(select(crypt(password, gen_salt("bf"))).first::<String>(&conn)?)
}

impl UserUpdate {
    pub fn with_password(password: &str) -> Result<UserUpdate, ApplicationError> {
        Ok(UserUpdate {
            chosen_name: None,
            email: None,
            pass_hash: Some(encrypt_password(password)?),
        })
    }
    pub fn new() -> UserUpdate {
        UserUpdate {
            chosen_name: None,
            email: None,
            pass_hash: None,
        }
    }
    pub fn with_chosen_name(&mut self, name: &str) -> &mut UserUpdate {
        self.chosen_name = Some(name.clone().to_owned());
        self
    }
    pub fn with_email(&mut self, e: &str) -> &mut UserUpdate {
        self.email = Some(e.clone().to_owned());
        self
    }
}

pub fn login(name_or_email: &str, pass: &str) -> Result<LoginResult, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    let conn = db::connection()?;
    match users
        .select(id)
        .filter(email.eq(name_or_email).or(chosen_name.eq(name_or_email)))
        .filter(pass_hash.eq(crypt(pass, pass_hash)))
        .first::<i32>(&conn)
    {
        Ok(i) => Ok(LoginResult::LoggedIn(create_cookie(i)?)),
        Err(diesel::NotFound) => Ok(LoginResult::AuthenticationFailure),
        Err(e) => Err(ApplicationError::Database(e)),
    }
}
fn create_cookie(a_user_id: i32) -> Result<NewCookie, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    let conn = db::connection()?;
    let _ = diesel::update(users.find(a_user_id))
        .set(last_login.eq(Utc::now().naive_utc()))
        .execute(&conn);
    //insert cookie (one per device to allow safe explicit log out)
    let uu = Uuid::new_v4();
    let uu = uu.to_hyphenated().to_string();
    let mut cookie = NewCookie {
        user_id: a_user_id,
        token: encrypt_password(&uu)?,
    };
    let result = diesel::insert_into(user_login_tokens::table)
        .values(&cookie)
        .execute(&conn);
    match result {
        Ok(_) => {
            cookie.token = uu;
            Ok(cookie)
        }
        Err(e) => Err(ApplicationError::Database(e)),
    }
}

pub fn add_user(
    user_name: &str,
    user_email: &str,
    pass: &str,
) -> Result<AddUserResult, ApplicationError> {
    #[allow(unused_imports)]
    use super::schema::pauth::users;
    use super::schema::pauth::users::dsl::*;
    let conn = db::connection()?;

    let existing = users
        .filter(chosen_name.eq(user_name).or(email.eq(user_email)))
        .load::<User>(&conn);
    if existing.is_ok() {
        let mut failures = vec![];
        let u = existing.unwrap();
        if u.len() > 0 {
            if u.get(0).unwrap().email == user_email {
                failures.push(UserActionFailure::EmailExists);
            } else {
                failures.push(UserActionFailure::UsernameExists);
            }
            return Ok(AddUserResult::NotAdded(failures));
        }
    }
    diesel::insert_into(users::table)
        .values((
            chosen_name.eq(user_name),
            email.eq(user_email),
            pass_hash.eq(crypt(pass, gen_salt("bf"))),
        ))
        .execute(&conn)?;
    let login_result = login(user_email, pass)?;
    match login_result {
        LoginResult::LoggedIn(uid) => Ok(AddUserResult::Added(uid)),
        LoginResult::AuthenticationFailure => Err(ApplicationError::ApplicationDataLogic(
            "Unable to login after creating user".to_owned(),
        )),
    }
}

pub fn delete_user(
    auth_token: &AuthenticatedID,
    pass: &str,
) -> Result<DeleteUserResult, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    if !check_id(auth_token)? {
        return Ok(DeleteUserResult::AuthFailure);
    }

    let conn = db::connection()?;
    //check password and delete
    let result = diesel::delete(
        users.filter(
            id.eq(auth_token.user_id)
                .and(pass_hash.eq(crypt(pass, pass_hash))),
        ),
    )
    .execute(&conn);
    match result {
        Ok(size) => {
            if size > 0 {
                Ok(DeleteUserResult::Deleted)
            } else {
                Ok(DeleteUserResult::NotFound)
            }
        }
        Err(e) => Err(ApplicationError::Database(e)),
    }
}

pub fn change_details(
    auth_token: &AuthenticatedID,
    changes: &UserUpdate,
) -> Result<ChangeDetailsResult, ApplicationError> {
    if !check_id(auth_token)? {
        return Ok(ChangeDetailsResult::AuthenticationFailure);
    }
    update_details(auth_token.user_id, changes)
}

fn update_details(uid: i32, changes: &UserUpdate) -> Result<ChangeDetailsResult, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    let conn = db::connection()?;
    let result = diesel::update(users.find(uid)).set(changes).execute(&conn);
    match result {
        Ok(size) => {
            if size > 0 {
                Ok(ChangeDetailsResult::Changed)
            } else {
                Err(ApplicationError::ApplicationDataLogic(
                    "Tried to update, but updated no rows.".to_owned(),
                ))
            }
        }
        Err(e) => Err(ApplicationError::Database(e)),
    }
}

pub fn get_user(auth_token: &AuthenticatedID) -> Result<Option<User>, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    if !check_id(auth_token)? {
        return Ok(None);
    }

    let conn = db::connection()?;

    let result = users.filter(id.eq(auth_token.user_id)).first(&conn);
    match result {
        Ok(user) => Ok(Some(user)),
        Err(e) => Err(ApplicationError::Database(e)),
    }
}

pub fn check_id(auth_token: &AuthenticatedID) -> Result<bool, ApplicationError> {
    use super::schema::pauth::user_login_tokens::dsl::*;
    let conn = db::connection()?;

    let result = user_login_tokens
        .filter(
            user_id
                .eq(auth_token.user_id)
                .and(token.eq(crypt(auth_token.token.clone(), token))),
        )
        .load::<UserLoginToken>(&conn);
    match result {
        Ok(tokens) => Ok(tokens.len() > 0),
        Err(e) => Err(ApplicationError::Database(e)),
    }
}

pub fn check_id_and_password(
    auth_token: &AuthenticatedID,
    password: &str,
) -> Result<bool, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    if !check_id(auth_token)? {
        return Ok(false);
    }
    let conn = db::connection()?;
    Ok(select(exists(
        users
            .filter(id.eq(auth_token.user_id))
            .filter(pass_hash.eq(crypt(password, pass_hash))),
    ))
    .get_result(&conn)?)
}

pub fn generate_pw_reset(
    name_or_email: &str,
    expires: Option<NaiveDateTime>,
) -> Result<Option<String>, ApplicationError> {
    use super::schema::pauth::users::dsl::*;
    let conn = db::connection()?;
    if let Ok(user) = users
        .filter(chosen_name.eq(name_or_email).or(email.eq(name_or_email)))
        .first::<User>(&conn)
    {
        //generate a pw_reset and return the string
        let tok = generate_random_string(20);
        diesel::insert_into(pw_reset::table)
            .values((
                pw_reset::dsl::user_id.eq(user.id),
                pw_reset::dsl::user_token_hash.eq(crypt(tok.clone(), gen_salt("bf"))),
                pw_reset::dsl::expires.eq(expires),
            ))
            .execute(&conn)?;
        Ok(Some(tok))
    } else {
        Ok(None)
    }
}

fn check_pw_reset(
    name_or_email: &str,
    reset_token: String,
) -> Result<Option<i32>, ApplicationError> {
    let conn = db::connection()?;

    Ok(users::table
        .left_join(pw_reset::table)
        .select(users::dsl::id)
        .filter(
            users::dsl::chosen_name
                .eq(name_or_email)
                .or(users::dsl::email.eq(name_or_email)),
        )
        .filter(
            pw_reset::user_token_hash
                .eq(crypt(reset_token.clone(), pw_reset::dsl::user_token_hash)),
        )
        .filter(
            pw_reset::dsl::expires
                .is_null()
                .or(pw_reset::dsl::expires.gt(diesel::dsl::now)),
        )
        .load(&conn)?
        .first()
        .cloned())
}

pub fn validate_pw_reset(
    name_or_email: &str,
    reset_token: String,
) -> Result<LoginResult, ApplicationError> {
    let uid = check_pw_reset(name_or_email, reset_token)?;
    if uid.is_some() {
        Ok(LoginResult::LoggedIn(create_cookie(uid.unwrap())?))
    } else {
        Ok(LoginResult::AuthenticationFailure)
    }
}

fn generate_random_string(len: usize) -> String {
    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(len)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::models::{
        AddUserResult, ChangeDetailsResult, DeleteUserResult, LoginResult,
        UserUpdate,
    };
    use std::env;

    fn setup() {
        if env::var("DATABASE_URL").is_err() {
            env::set_var(
                "DATABASE_URL",
                "postgres://pauth_admin:pauth_admin_password@localhost:5432/pauth",
            )
        }
    }

    #[test]
    fn add_paul() {
        setup();
        let res = add_user("Paul", "paul@pr0.co.uk", "test");
        if res.is_err() {
            print!("Error: {:?}", res.err().unwrap());
            panic!()
        } else {
            match res.unwrap() {
                AddUserResult::Added(_) => {}
                AddUserResult::NotAdded(_) => panic!(),
            }
        }
        if let LoginResult::LoggedIn(user_login) = login("Paul", "test").unwrap() {
            delete_user(&user_login, "test").unwrap();
        }
    }

    #[test]
    fn get_password_hash() {
        setup();
        let pass = UserUpdate::with_password("test").unwrap();
        println!("Password hash:'{}'", pass.pass_hash.unwrap())
    }

    #[test]
    fn create_user_log_in_pw_reset_delete() {
        setup();
        let user = add_user("user94", "user94@pr0.co.uk", "pass94").unwrap();
        let _a_user_id;
        let cookie;
        match user {
            AddUserResult::Added(auth_id) => {
                _a_user_id = Some(auth_id.user_id);
                cookie = Some(auth_id);
            }
            AddUserResult::NotAdded(_) => {
                panic!("Test failure: User not added when it is expected that they would be")
            }
        }
        //verify the cookie
        assert_eq!(true, check_id(&cookie.clone().unwrap()).unwrap());

        //log the user in and get a new cookie
        let login_result = login("user94", "pass94").unwrap();
        match login_result {
            LoginResult::LoggedIn(cookie2) => assert_eq!(true, check_id(&cookie2).unwrap()),
            _ => panic!("Test failure: Not able to log user in as expected"),
        }

        //request a pw_reset
        let pw_reset_token = generate_pw_reset("user94@pr0.co.uk", None)
            .unwrap()
            .unwrap();

        let auth_token;
        //authenticate a pw_reset
        match validate_pw_reset("user94", pw_reset_token.clone()).unwrap() {
            LoginResult::LoggedIn(id) => {
                assert_eq!(true, check_id(&id).unwrap());
                auth_token = Some(id);
            }
            _ => panic!("Test failure: Password reset validation failed"),
        }

        //change the password
        let new_pass = UserUpdate::with_password("new_pass").unwrap();



        match change_details(&auth_token.unwrap(), &new_pass).unwrap()
        {
            ChangeDetailsResult::Changed => {}
            _ => panic!("Change details with pw reset token failed"),
        }
        //old password no longer works
        assert_eq!(
            LoginResult::AuthenticationFailure,
            login("user94", "pass94").unwrap()
        );

        //log in with new password
        let login_result = login("user94", "new_pass").unwrap();
        let mut cookie;
        match login_result {
            LoginResult::LoggedIn(cookie2) => {
                assert_eq!(true, check_id(&cookie2).unwrap());
                cookie = Some(cookie2);
            }
            _ => panic!("Test failure: Not able to log user in as expected"),
        }
        //change the email
        change_details(
            cookie.as_ref().unwrap(),
            UserUpdate::new().with_email("new_email@pr0.co.uk"),
        )
        .unwrap();

        //log in with new email
        let login_result = login("new_email@pr0.co.uk", "new_pass").unwrap();
        //cookie = None;
        match login_result {
            LoginResult::LoggedIn(c) => {
                assert_eq!(true, check_id(&c).unwrap());
                cookie = Some(c);
            }
            LoginResult::AuthenticationFailure => panic!("Auth failure logging in with new email"), //_=>{panic!("Test failure: Not able to log user in with new email as expected")}
        }

        //delete the user
        let delete_user_result = delete_user(&cookie.unwrap(), "new_pass").unwrap();
        match delete_user_result {
            DeleteUserResult::Deleted => {}
            _ => panic!("Test Failure: User not deleted"),
        }
    }
    #[test]
    fn cannot_add_existing_user() {}
}
