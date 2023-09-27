use entities::profile::Profile;
use crate::errors::admin::ErrorAdmin;
use crate::repositories::profile_transaction_repository::ProfileTransactionRepository;
pub struct TransactionCreateProfile<'a> {
    admin_id: &'a String,
    profile_id: &'a String,
    firstname: &'a String,
    lastname: &'a String,
    email_address: &'a String,
    phone_number: &'a String
}

impl TransactionCreateProfile<'_> {
    pub fn new<'a>(admin_id: &'a String, profile_id: &'a String, firstname: &'a String, lastname: &'a String, email_address: &'a String, phone_number: &'a String) -> TransactionCreateProfile<'a> {
        TransactionCreateProfile {
            admin_id,
            profile_id, 
            firstname, 
            lastname, 
            email_address, 
            phone_number, 
        }
    }

    pub fn execute(&self, repo: &mut impl ProfileTransactionRepository) -> Result<(), ErrorAdmin> {
        if repo.is_admin_exist(self.admin_id) {
            let profile = Profile::new(
                self.admin_id,
                self.profile_id,
                self.firstname,
                self.lastname,
                self.email_address,
                self.phone_number,
            );
    
            repo.create_profile(profile);
            return Ok(())
        } else {
            return Err(ErrorAdmin::AdminNoExist)
        }
    }
}