use crate::errors::project::ErrorProject;
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::transaction::Transaction;
use entities::project::Project;

pub struct TransactionGetProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String
}

impl TransactionGetProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String) -> TransactionGetProject<'a> {
        TransactionGetProject { db, project_id }
    }
}

impl Transaction<Project> for TransactionGetProject<'_> {
    fn execute(&mut self) -> Result<Project, Box<dyn crate::errors::ErrorDomain>> {
        let project = self.db.get_project(self.project_id);

        if project.is_none() {
            return Err(Box::new(ErrorProject::ProjectNotExist))
        }

        return Ok(project.unwrap().clone())
    }
}

pub struct TransactionGetAllProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>
}

impl TransactionGetAllProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>) -> TransactionGetAllProject<'a> {
        TransactionGetAllProject { db }
    }
}

impl Transaction<Vec<Project>> for TransactionGetAllProject<'_> {
    fn execute(&mut self) -> Result<Vec<Project>, Box<dyn crate::errors::ErrorDomain>> {
        let projects = self.db.get_projects();

        return Ok(projects);
    }
}

pub struct TransactionGetProjectByPage<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    page: usize,
    content_size: usize
}

impl TransactionGetProjectByPage<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, page: usize, content_size: usize) -> TransactionGetProjectByPage {
        TransactionGetProjectByPage { 
            db, 
            page, 
            content_size
        }
    }
}

impl Transaction<Vec<Project>> for TransactionGetProjectByPage<'_> {
    fn execute(&mut self) -> Result<Vec<Project>, Box<dyn crate::errors::ErrorDomain>> {
        let projects = self.db.get_project_by_pages(self.page, self.content_size);

        return Ok(projects);
    }
}