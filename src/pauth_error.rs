use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;

pub type InternalErrorMessage = String;

#[derive(Debug)]
pub enum ApplicationError {
    Database(diesel::result::Error),
    Connection(r2d2::Error),
    ApplicationDataLogic(InternalErrorMessage),
}

impl From<DieselError> for ApplicationError {
    fn from(error: DieselError) -> ApplicationError {
        ApplicationError::Database(error)
    }
}

impl From<R2D2Error> for ApplicationError {
    fn from(error: R2D2Error) -> ApplicationError {
        ApplicationError::Connection(error)
    }
}
