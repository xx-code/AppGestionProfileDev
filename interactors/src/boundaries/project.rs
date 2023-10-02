use domains::{repositories::project_transaction_repository::ProjectTransactionRepository, errors::project::ErrorProject};
use time::Date;

pub struct RequestCreateProject<'a> {
    pub title: &'a String,
    pub description: &'a String,
    pub date_start: &'a Date,
    pub date_end: Option<&'a Date>
}

pub struct RequestUpdateProject<'a> {
    pub project_id: &'a String,
    pub title: Option<&'a String>,
    pub description: Option<&'a String>,
    pub date_start: Option<&'a Date>,
    pub date_end: Option<&'a Date>
}

pub struct RequestAddLink<'a> {
    pub project_id: &'a String,
    pub title: &'a String,
    pub address: &'a String
}

pub struct RequestGetProjectsByPaging {
    pub page: usize, 
    pub content_size: usize
}

pub struct ResponseGetLink {
    pub title: String,
    pub address: String
}

pub struct ResponseGetProject {
    pub title: String,
    pub description: String,
    pub date_start: Date,
    pub date_end: Option<Date>,
    pub links: Vec<ResponseGetLink> 
}

pub struct ResponseGetProjects {
    pub projects: Vec<ResponseGetProject>
}

pub type ProjectID = String;

pub trait InteractorProject {
    fn create_profile(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestCreateProject) -> Result<ProjectID, ErrorProject>;
    fn add_link(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestAddLink) -> Result<ProjectID, ErrorProject>;
    fn delete_link(&self, repo: &mut impl ProjectTransactionRepository, project_id: &ProjectID, link_id: &String) -> Result<bool, ErrorProject>;
    fn get_project(&self, repo: &impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<ResponseGetProject, ErrorProject>;
    fn get_all_project(&self, repo: &impl ProjectTransactionRepository) -> Result<ResponseGetProjects, ErrorProject>;
    fn get_project_by_pages(&self,  repo: &impl ProjectTransactionRepository, request: &RequestGetProjectsByPaging) -> Result<ResponseGetProjects, ErrorProject>;
    fn delete_project(&self, repo: &mut impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<bool, ErrorProject>;
    fn update_project(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestUpdateProject) -> Result<bool, ErrorProject>;
}