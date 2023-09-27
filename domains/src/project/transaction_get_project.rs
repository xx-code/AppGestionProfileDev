use crate::errors::project::ErrorProject;
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use entities::project::Project;

pub struct TransactionGetProject<'a> {
    project_id: &'a String
}

impl TransactionGetProject<'_> {
    pub fn new<'a>(project_id: &'a String) -> TransactionGetProject<'a> {
        TransactionGetProject { project_id }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<Project, ErrorProject> {
        let project = repo.get_project(self.project_id);

        if project.is_none() {
            return Err(ErrorProject::ProjectNotExist)
        }

        return Ok(project.unwrap().clone())
    }
}

pub struct TransactionGetAllProject {
}

impl TransactionGetAllProject {
    pub fn new() -> TransactionGetAllProject {
        TransactionGetAllProject { }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<Vec<Project>, ErrorProject> {
        let projects = repo.get_projects();

        return Ok(projects);
    }
}

pub struct TransactionGetProjectByPage {
    page: usize,
    content_size: usize
}

impl TransactionGetProjectByPage {
    pub fn new(page: usize, content_size: usize) -> TransactionGetProjectByPage {
        TransactionGetProjectByPage { 
            page, 
            content_size
        }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<Vec<Project>, ErrorProject> {
        if self.page > repo.get_pages_number(self.content_size) {
            return Err(ErrorProject::PagingNotAllowPageIndexMustBeLessThanPageNumber)
        }
        let projects = repo.get_project_by_pages(self.page, self.content_size);

        return Ok(projects);
    }
}