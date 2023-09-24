use crate::errors::ErrorDomain;

pub enum ErrorProject {
    ProjectNotExist,
    DateEndMustBeSuperiorDateStart,
    LinkNotExist,
    PagingNotAllowPageIndexMustBeLessThanPageNumber
}

impl ErrorDomain for ErrorProject {}