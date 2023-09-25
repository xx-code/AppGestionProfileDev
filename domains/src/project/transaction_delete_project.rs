use std::borrow::BorrowMut;

use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, project::ErrorProject}
};

pub struct TransactionDeleteProject<'a> {
    project_id: &'a String
}

impl TransactionDeleteProject<'_> {
    pub fn new<'a>(project_id: &'a String) -> TransactionDeleteProject<'a> {
        TransactionDeleteProject { 
            project_id
        }
    }
}

impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>> for TransactionDeleteProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            repo.delete_project(self.project_id);
            Ok(())
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
        
    }
}