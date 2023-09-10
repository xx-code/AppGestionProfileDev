use repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::transaction::Transaction;

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

impl Transaction for TransactionDeleteProject<'_> {
    fn execute(&mut self) -> () {
        self.db.delete_project(self.project_id);
    }
}