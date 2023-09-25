use entities::profile::Profile;

use crate::{
    repositories::profile_transaction_repository::ProfileTransactionRepository, 
    transaction::Transaction,
    errors::profile::ErrorProfile
};

pub struct TransactionGetProfile<'a> {
    profile_id: &'a String
}

impl TransactionGetProfile<'_> {
    pub fn new<'a>(profile_id: &'a String) -> TransactionGetProfile<'a> {
        TransactionGetProfile { profile_id }
    }
}

impl Transaction<Profile, ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionGetProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<Profile, ErrorProfile> {
        let profile = repo.get_profile(self.profile_id);

        if profile.is_none() {
            return Err(ErrorProfile::ProfileNotExist)
        }

        Ok(profile.unwrap().clone())
    }
}