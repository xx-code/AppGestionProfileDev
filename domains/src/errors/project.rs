use crate::errors::ErrorDomain;

pub enum ErrorProject {
    ProjectNotExist,
    DateEndMustBeSuperiorDateStart,
    LinkNotExist
}

impl ErrorDomain for ErrorProject {}