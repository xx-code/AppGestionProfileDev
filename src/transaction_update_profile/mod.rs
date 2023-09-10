use crate::profile_transaction_persistence::ProfileTransactionPersistence;
use crate::profile_transaction_repository::ProfileTransactionRepository;
use crate::transaction::Transaction;

pub struct TransactionUpdateFirstnameProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    firstname: &'a String,
}
impl TransactionUpdateFirstnameProfile<'_> {
    fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, firstname: &'a String) -> TransactionUpdateFirstnameProfile<'a> {
        TransactionUpdateFirstnameProfile {
            db,
            profile_id, 
            firstname 
        }
    }
}
impl Transaction for TransactionUpdateFirstnameProfile<'_> {
    fn execute(&mut self) -> () {
        let profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_firstname(self.firstname);
            self.db.update_profile(profile);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateLastnameProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    lastname: &'a String,
}
impl TransactionUpdateLastnameProfile<'_> {
    fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, lastname: &'a String) -> TransactionUpdateLastnameProfile<'a> {
        TransactionUpdateLastnameProfile {
            db,
            profile_id, 
            lastname
        }
    }
}
impl Transaction for TransactionUpdateLastnameProfile<'_> {
    fn execute(&mut self) -> () {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_lastname(self.lastname);
            self.db.update_profile(profile);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateEmailAddressProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    email_address: &'a String,
}
impl TransactionUpdateEmailAddressProfile<'_> {
    fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, email_address: &'a String) -> TransactionUpdateEmailAddressProfile<'a> {
        TransactionUpdateEmailAddressProfile { 
            db,
            profile_id, 
            email_address
        }
    }
}
impl Transaction for TransactionUpdateEmailAddressProfile<'_> {
    fn execute(&mut self) -> () {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_email_address(self.email_address);
            self.db.update_profile(profile);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdatePhoneNumberProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    phone_number: &'a String,
}
impl TransactionUpdatePhoneNumberProfile<'_> {
    fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, phone_number: &'a String) -> TransactionUpdatePhoneNumberProfile<'a> {
        TransactionUpdatePhoneNumberProfile {
            db,
            profile_id,
            phone_number
        }
    }
}
impl Transaction for TransactionUpdatePhoneNumberProfile<'_>{
    fn execute(&mut self) -> () {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_phone_number(self.phone_number);
            self.db.update_profile(profile);
        } else {
            println!("ADD test gestion error no profile")
        }
    }   
}

#[cfg(test)]
pub mod test {
    use crate::data_persistence::DataPersistence;
    use crate::profile_transaction_persistence::ProfileTransactionPersistence;
    use crate::profile_transaction_repository::ProfileTransactionRepository;
    use crate::{transaction::Transaction, admin_transaction_persistence::AdminTransactionPersistence};
    use crate::transaction_create_profile::TransactionCreateProfile;
    use crate::transaction_create_admin::TransactionCreateAdmin;
    use super::{
        TransactionUpdateFirstnameProfile,
        TransactionUpdateLastnameProfile,
        TransactionUpdateEmailAddressProfile,
        TransactionUpdatePhoneNumberProfile
    };
    pub fn setup_admin(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let mut admin_data = AdminTransactionPersistence::build(db);
        
        let mut ts = TransactionCreateAdmin::new(
            Box::new(admin_data),
            &admin_id,
            &username,
            &password,
        );
        ts.execute();
    }

    pub fn setup_profile(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(db));
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
    }
    #[test]
    fn test_update_firstname_profile() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_firstname = String::from("new_firstname");
        let mut ts = TransactionUpdateFirstnameProfile::new(
            profile_data,
            &profile_id, 
            &new_firstname
        );
        ts.execute();
        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();

        assert_eq!(profile.get_firstname(), &new_firstname);
    }

    #[test]
    fn test_update_lastname_profile() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_lastname = String::from("new_lastname");
        let mut ts = TransactionUpdateLastnameProfile::new(
            profile_data,
            &profile_id, 
            &new_lastname
        );
        ts.execute();
        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();

        assert_eq!(profile.get_lastname(), &new_lastname);
    }

    #[test]
    fn test_update_email_address() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_email = String::from("New email");
        let mut ts = TransactionUpdateEmailAddressProfile::new(
            profile_data, 
            &profile_id, 
            &new_email
        );
        ts.execute();
        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();

        assert_eq!(profile.get_email_address(), &new_email);
        
    }

    #[test]
    fn test_phone_number() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let mut profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_phone: String = String::from("482982569");
        let mut ts = TransactionUpdatePhoneNumberProfile::new(
            profile_data, 
            &profile_id, 
            &new_phone
        );
        ts.execute();
        drop(ts);
       
        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();
        
        assert_eq!(profile.get_phone_number(), &new_phone);
    }

}