use crate::errors::project::ErrorProject;
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::transaction::Transaction;
use entities::project::Project;

pub struct TransactionGetProject<'a> {
    project_id: &'a String
}

impl TransactionGetProject<'_> {
    pub fn new<'a>(project_id: &'a String) -> TransactionGetProject<'a> {
        TransactionGetProject { project_id }
    }
}

impl Transaction<Project, ErrorProject, Box<dyn ProjectTransactionRepository>> for TransactionGetProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<Project, ErrorProject> {
        let project = repo.get_project(self.project_id);

        if project.is_none() {
            return Err(ErrorProject::ProjectNotExist)
        }

        return Ok(project.unwrap().clone())
    }
}

pub struct TransactionGetAllProject<'a> {
}

impl TransactionGetAllProject<'_> {
    pub fn new<'a>() -> TransactionGetAllProject<'a> {
        TransactionGetAllProject { }
    }
}

impl Transaction<Vec<Project>, ErrorProject, Box<dyn ProjectTransactionRepository>> for TransactionGetAllProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<Vec<Project>, ErrorProject> {
        let projects = repo.get_projects();

        return Ok(projects);
    }
}

pub struct TransactionGetProjectByPage<'a> {
    page: usize,
    content_size: usize
}

impl TransactionGetProjectByPage<'_> {
    pub fn new<'a>(page: usize, content_size: usize) -> TransactionGetProjectByPage<'a> {
        TransactionGetProjectByPage { 
            page, 
            content_size
        }
    }
}

impl Transaction<Vec<Project>, ErrorProject, Box<dyn ProjectTransactionRepository>> for TransactionGetProjectByPage<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<Vec<Project>, ErrorProject> {
        
        if self.page > repo.get_pages_number(self.content_size) {
            return Err(ErrorProject::PagingNotAllowPageIndexMustBeLessThanPageNumber)
        }
        let projects = repo.get_project_by_pages(self.page, self.content_size);

        return Ok(projects);
    }
}