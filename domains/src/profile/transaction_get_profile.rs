use entities::profile::Profile;

use crate::{
    repositories::profile_transaction_repository::ProfileTransactionRepository, 
    transaction::Transaction,
    errors::profile::ErrorProfile
};

pub struct TransactionGetProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String
}

impl TransactionGetProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String) -> TransactionGetProfile<'a> {
        TransactionGetProfile { db, profile_id }
    }
}

impl Transaction<Profile> for TransactionGetProfile<'_> {
    fn execute(&mut self) -> Result<Profile, Box<dyn crate::errors::ErrorDomain>> {
        let profile = self.db.get_profile(self.profile_id);

        if profile.is_none() {
            return Err(Box::new(ErrorProfile::ProfileNotExist))
        }

        Ok(profile.unwrap().clone())
    }
}