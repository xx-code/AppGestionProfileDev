use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::errors::project::ErrorProject;

pub struct TransactionDeleteProject<'a> {
    project_id: &'a String
}

impl TransactionDeleteProject<'_> {
    pub fn new<'a>(project_id: &'a String) -> TransactionDeleteProject<'a> {
        TransactionDeleteProject { 
            project_id
        }
    }

    pub fn execute(&self, repo: &mut impl ProjectTransactionRepository) -> Result<(), ErrorProject> {
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            repo.delete_project(self.project_id);
            Ok(())
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
    }
}
