use crate::errors::profile::ErrorProfile;
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

    pub fn execute(&self, repo: &mut impl ProfileTransactionRepository) -> Result<(), ErrorProfile> {
        let profile = repo.get_profile(self.profile_id);
        
        if !profile.is_none() {
            repo.delete_profile(self.profile_id);
            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }
}

