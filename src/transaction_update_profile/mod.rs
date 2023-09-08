use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;

pub struct TransactionUpdateFirstnameProfile<'a> {
    profile_id: &'a String,
    firstname: &'a String,
}
impl TransactionUpdateFirstnameProfile<'_> {
    fn new<'a>(profile_id: &'a String, firstname: &'a String) -> TransactionUpdateFirstnameProfile<'a> {
        TransactionUpdateFirstnameProfile { 
            profile_id, 
            firstname 
        }
    }
}
impl Transaction for TransactionUpdateFirstnameProfile<'_> {
    fn execute(&self) -> () {
        unsafe {
            let udpate_info = ("firstname", self.firstname);
            GLOBAL_DB.update_profile(self.profile_id, udpate_info);
        }
    }
}

pub struct TransactionUpdateLastnameProfile<'a> {
    profile_id: &'a String,
    lastname: &'a String,
}
impl TransactionUpdateLastnameProfile<'_> {
    fn new<'a>(profile_id: &'a String, lastname: &'a String) -> TransactionUpdateLastnameProfile<'a> {
        TransactionUpdateLastnameProfile { 
            profile_id, 
            lastname
        }
    }
}
impl Transaction for TransactionUpdateLastnameProfile<'_> {
    fn execute(&self) -> () {
        unsafe {
            let udpate_info = ("lastname", self.lastname);
            GLOBAL_DB.update_profile(self.profile_id, udpate_info);
        }
    }
}

pub struct TransactionUpdateEmailAddressProfile<'a> {
    profile_id: &'a String,
    email_address: &'a String,
}
impl TransactionUpdateEmailAddressProfile<'_> {
    fn new<'a>(profile_id: &'a String, email_address: &'a String) -> TransactionUpdateEmailAddressProfile<'a> {
        TransactionUpdateEmailAddressProfile { 
            profile_id, 
            email_address
        }
    }
}
impl Transaction for TransactionUpdateEmailAddressProfile<'_> {
    fn execute(&self) -> () {
        unsafe {
            let udpate_info = ("email_address", self.email_address);
            GLOBAL_DB.update_profile(self.profile_id, udpate_info);
        }
    }
}

pub struct TransactionUpdatePhoneNumberProfile<'a> {
    profile_id: &'a String,
    phone_number: &'a String,
}
impl TransactionUpdatePhoneNumberProfile<'_> {
    fn new<'a>(profile_id: &'a String, phone_number: &'a String) -> TransactionUpdatePhoneNumberProfile<'a> {
        TransactionUpdatePhoneNumberProfile { 
            profile_id,
            phone_number
        }
    }
}
impl Transaction for TransactionUpdatePhoneNumberProfile<'_>{
    fn execute(&self) -> () {
        unsafe {
            let udpate_info = ("phone_number", self.phone_number);
            GLOBAL_DB.update_profile(self.profile_id, udpate_info);
        }
    }   
}

#[cfg(test)]
mod test {
    use crate::transaction::Transaction;
    use crate::transaction_create_profile::TransactionCreateProfile;
    use crate::transaction_create_admin::TransactionCreateAdmin;
    use crate::DB::GLOBAL_DB;
    use super::{
        TransactionUpdateFirstnameProfile,
        TransactionUpdateLastnameProfile,
        TransactionUpdateEmailAddressProfile,
        TransactionUpdatePhoneNumberProfile
    };
    fn setup() {
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
    }
    #[test]
    fn test_update_firstname_profile() {
        setup();
        let profile_id = String::from("profile1");
        let new_firstname = String::from("new_firstname");
        let ts = TransactionUpdateFirstnameProfile::new(&profile_id, &new_firstname);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_firstname(), &new_firstname);
            GLOBAL_DB.clean();
        }
    }

    #[test]
    fn test_update_lastname_profile() {
        setup();
        let profile_id = String::from("profile1");
        let new_lastname = String::from("new_lastname");
        let ts = TransactionUpdateLastnameProfile::new(&profile_id, &new_lastname);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_lastname(), &new_lastname);
            GLOBAL_DB.clean();
        }
    }

    #[test]
    fn test_update_email_address() {
        setup();
        let profile_id = String::from("profile1");
        let new_email = String::from("New email");
        let ts = TransactionUpdateEmailAddressProfile::new(&profile_id, &new_email);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_email_address(), &new_email);
            GLOBAL_DB.clean();
        }
    }

    #[test]
    fn test_phone_number() {
        setup();
        let profile_id = String::from("profile1");
        let new_phone: String = String::from("482982569");
        let ts = TransactionUpdatePhoneNumberProfile::new(&profile_id, &new_phone);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_phone_number(), &new_phone);
            GLOBAL_DB.clean();
        }
    }

}