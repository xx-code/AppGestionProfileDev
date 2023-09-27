use entities::profile::Profile;
use crate::{
    repositories::profile_transaction_repository::ProfileTransactionRepository, 
    errors::profile::ErrorProfile
};

pub struct TransactionGetProfile<'a> {
    profile_id: &'a String
}

impl TransactionGetProfile<'_> {
    pub fn new<'a>(profile_id: &'a String) -> TransactionGetProfile<'a> {
        TransactionGetProfile { profile_id }
    }

    pub fn execute(&self, repo: &impl ProfileTransactionRepository) -> Result<Profile, ErrorProfile> {
        let profile = repo.get_profile(self.profile_id);

        if profile.is_none() {
            return Err(ErrorProfile::ProfileNotExist)
        }

        Ok(profile.unwrap().clone())
    }
}