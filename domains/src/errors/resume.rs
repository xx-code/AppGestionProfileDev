use crate::errors::ErrorDomain;

pub enum ErrorResume {
    DateEndMustBeSuperiorDateStart,
    ResumeNotExist
}

impl ErrorDomain for ErrorResume {}