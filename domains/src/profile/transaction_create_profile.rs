use std::borrow::BorrowMut;

use entities::profile::Profile;
use crate::{transaction::Transaction, errors::{ErrorDomain, admin::ErrorAdmin, profile::ErrorProfile}};
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
}

impl Transaction<(), ErrorAdmin, Box<dyn ProfileTransactionRepository>> for TransactionCreateProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository> ) -> Result<(), ErrorAdmin> {
        
        let repo = repo.borrow_mut();

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