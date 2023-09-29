use domains::{repositories::project_transaction_repository::ProjectTransactionRepository, errors::project::ErrorProject};
use time::Date;

pub struct RequestCreateProject<'a> {
    title: &'a String,
    description: &'a String,
    date_start: &'a Date,
    date_end: Option<&'a Date>
}

pub struct RequestUpdateProject<'a> {
    title: Option<&'a String>,
    description: Option<&'a String>,
    date_start: Option<&'a Date>,
    date_end: Option<&'a Date>
}

pub struct RequestAddLink<'a> {
    title: &'a String,
    address: &'a String
}

pub struct RequestUpdateLink<'a> {
    title: Option<&'a String>,
    address: Option<&'a String>
}

pub struct RequestGetProjectsByPaging {
    age: usize, 
    content_size: usize
}

pub struct ResponeGetLink {
    title: String,
    address: String
}

pub struct ResponseGetProject {
    pub title: String,
    pub description: String,
    pub date_start: Date,
    pub date_end: Option<Date>,
    pub links: Vec<ResponeGetLink> 
}

pub struct ResponseGetProjects {
    pub projects: Vec<ResponseGetProject>
}

pub type ProjectID = String;

pub trait InteractorProject {
    fn create_profile(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestCreateProject) -> Result<ProjectID, ErrorProject>;
    fn get_project(&self, repo: &impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<ResponseGetProject, ErrorProject>;
    fn get_all_project(&self, repo: &impl ProjectTransactionRepository) -> Result<ResponseGetProjects, ErrorProject>;
    fn get_project_by_pages(&self,  repo: &impl ProjectTransactionRepository, request: &RequestGetProjectsByPaging) -> Result<ResponseGetProjects, ErrorProject>;
    fn delete_project(&self, repo: &mut impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<bool, ErrorProject>;
    fn update_project(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestUpdateProject) -> Result<bool, ErrorProject>;
}