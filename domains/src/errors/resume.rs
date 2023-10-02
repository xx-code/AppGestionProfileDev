use crate::errors::ErrorDomain;

pub enum ErrorResume {
    DateEndMustBeSuperiorDateStart,
    ResumeNotExist,
    ResumeTypeDoesntMatch,
}

impl ErrorDomain for ErrorResume {}