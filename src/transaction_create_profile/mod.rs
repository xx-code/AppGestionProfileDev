use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;
use crate::profile::Profile;
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

impl Transaction for TransactionCreateProfile<'_> {
    fn execute(&self) -> () {
        unsafe {
            let profile = Profile::new(
                self.admin_id,
                self.profile_id,
                self.firstname,
                self.lastname,
                self.email_address,
                self.phone_number,
            );

            GLOBAL_DB.add_new_profile(profile);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction,
        DB::GLOBAL_DB,
    };
    use super::TransactionCreateProfile;

    #[test]
    fn transaction_create_profile() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        ts.execute();

        let ts = TransactionCreateProfile::new(
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();

            assert_eq!(profile.get_id(), &profile_id);
            assert_eq!(profile.get_firstname(), &firstname);
            assert_eq!(profile.get_lastname(), &lastname);
            assert_eq!(profile.get_email_address(), &email_address);
            assert_eq!(profile.get_phone_number(), &phone_number);
        }
    }

    #[test]
    fn down() {
        unsafe {
            GLOBAL_DB.clean_admin();
            GLOBAL_DB.clean_profile();
        }
    }
}