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

#[cfg(test)]
mod test {
    use crate::transaction::Transaction;
    use crate::transaction_create_profile::TransactionCreateProfile;
    use crate::transaction_create_admin::TransactionCreateAdmin;
    use crate::DB::GLOBAL_DB;
    use super::TransactionUpdateFirstnameProfile;
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
}