use crate::transaction::Transaction;
use repositories::profile_transaction_repository::ProfileTransactionRepository;

pub struct TransactionDeleteProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String
}

impl TransactionDeleteProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String) -> TransactionDeleteProfile<'a> {
        TransactionDeleteProfile { 
            db, 
            profile_id
        }
    }
}

impl Transaction for TransactionDeleteProfile<'_> {
    fn execute(&mut self) -> () {
        self.db.delete_profile(self.profile_id)
    }
}

