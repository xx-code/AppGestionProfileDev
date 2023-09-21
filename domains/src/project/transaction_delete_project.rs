use entities::project::Project;

use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, project::ErrorProject}
};

pub struct TransactionDeleteProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String
}

impl TransactionDeleteProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String) -> TransactionDeleteProject<'a> {
        TransactionDeleteProject { 
            db, 
            project_id
        }
    }
}

impl Transaction<()> for TransactionDeleteProject<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);
        
        if !project.is_none() {
            self.db.delete_project(self.project_id);
            Ok(())
        } else {
            Err(Box::new(ErrorProject::ProjectNotExist))
        }
        
    }
}