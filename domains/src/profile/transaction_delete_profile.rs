use crate::{transaction::Transaction, errors::{ErrorDomain, profile::ErrorProfile}};
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
    fn execute(&mut self) ->  Result<(), Box<dyn ErrorDomain>> {
        let profile = self.db.get_profile(self.profile_id);
        
        if !profile.is_none() {
            self.db.delete_profile(self.profile_id);
            Ok(())
        } else {
            Err(Box::new(ErrorProfile::ProfileNotExist))
        }
    }
}

