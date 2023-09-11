use crate::errors::ErrorDomain;

pub enum ErrorProfile {
    ProfileNotExist
}

impl ErrorDomain for ErrorProfile {}