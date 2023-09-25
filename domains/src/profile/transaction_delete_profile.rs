use std::borrow::BorrowMut;

use crate::{transaction::Transaction, errors::{ErrorDomain, profile::ErrorProfile}};
use crate::repositories::profile_transaction_repository::ProfileTransactionRepository;

pub struct TransactionDeleteProfile<'a> {
    profile_id: &'a String
}

impl TransactionDeleteProfile<'_> {
    pub fn new<'a>(profile_id: &'a String) -> TransactionDeleteProfile<'a> {
        TransactionDeleteProfile { 
            profile_id
        }
    }
}

impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionDeleteProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) ->  Result<(), ErrorProfile> {
        let mut repo = repo.borrow_mut();

        let profile = repo.get_profile(self.profile_id);
        
        if !profile.is_none() {
            repo.delete_profile(self.profile_id);
            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }
}

