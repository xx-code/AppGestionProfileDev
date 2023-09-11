use crate::errors::ErrorDomain;

pub enum ErrorProject {
    ProjectNotExist,
    DateEndMustBeSuperiorDateStart
}

impl ErrorDomain for ErrorProject {}