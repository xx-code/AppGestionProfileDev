use crate::errors::ErrorDomain;

pub enum ErrorProfile {
    ProfileNotExist,
    LinkProfileNotExit
}

impl ErrorDomain for ErrorProfile {}