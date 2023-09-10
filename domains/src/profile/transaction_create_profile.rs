use repositories::profile_transaction_repository::ProfileTransactionRepository;
use entities::profile::Profile;
use crate::transaction::Transaction;

pub struct TransactionCreateProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    admin_id: &'a String,
    profile_id: &'a String,
    firstname: &'a String,
    lastname: &'a String,
    email_address: &'a String,
    phone_number: &'a String
}

impl TransactionCreateProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, admin_id: &'a String, profile_id: &'a String, firstname: &'a String, lastname: &'a String, email_address: &'a String, phone_number: &'a String) -> TransactionCreateProfile<'a> {
        TransactionCreateProfile {
            db,
            admin_id,
            profile_id, 
            firstname, 
            lastname, 
            email_address, 
            phone_number, 
        }
    }
}

impl Transaction for TransactionCreateProfile<'_> {
    fn execute(&mut self) -> () {
        
        if self.db.is_admin_exist(self.admin_id) {
            let profile = Profile::new(
                self.admin_id,
                self.profile_id,
                self.firstname,
                self.lastname,
                self.email_address,
                self.phone_number,
            );
    
            self.db.create_profile(profile);
        }
    }
}