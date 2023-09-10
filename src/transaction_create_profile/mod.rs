use crate::profile_transaction_persistence::ProfileTransactionPersistence;
use crate::profile_transaction_repository::ProfileTransactionRepository;
use crate::transaction::Transaction;
use crate::profile::Profile;
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

#[cfg(test)]
pub mod tests {
    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction,
        entity::Entity,
        data_persistence::DataPersistence,
        profile_transaction_persistence::ProfileTransactionPersistence, 
        admin_transaction_persistence::AdminTransactionPersistence, 
        profile_transaction_repository::ProfileTransactionRepository,
    };
    use super::TransactionCreateProfile;

    #[test]
    pub fn test_transaction_create_profile() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut db = DataPersistence::new();
        let mut admin_data = Box::new(AdminTransactionPersistence::build(&mut db) );

        let mut ts = TransactionCreateAdmin::new(
            admin_data,
            &admin_id,
            &username,
            &password,
        );
        ts.execute();
        drop(ts);

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionCreateProfile::new(
            profile_data,
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        ts.execute();
        drop(ts);
        

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));
        let profile = profile_data.get_profile(&profile_id).unwrap();

        assert_eq!(profile.get_id(), &profile_id);
        assert_eq!(profile.get_firstname(), &firstname);
        assert_eq!(profile.get_lastname(), &lastname);
        assert_eq!(profile.get_email_address(), &email_address);
        assert_eq!(profile.get_phone_number(), &phone_number);
    }
    #[test]
    fn test_if_admin_exist_before_create_admin() {
        let admin_id = String::from("admin_3");

        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut db = DataPersistence::new();

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionCreateProfile::new(
            profile_data,
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        ts.execute();
        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id);

        assert!(profile.is_none());
    }
}