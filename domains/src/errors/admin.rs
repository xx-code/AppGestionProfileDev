use crate::errors::ErrorDomain;
pub enum ErrorAdmin {
    AdminAlreadyExist,
    AdminNoExist
}

impl ErrorDomain for ErrorAdmin {}