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
    use crate::transaction_create_profile::tests::test_transaction_create_profile;
    use crate::DB::GLOBAL_DB;
    use super::TransactionUpdateFirstnameProfile;
    #[test]
    fn test_update_firstname_profile() {
        test_transaction_create_profile();
        let profile_id = String::from("profile1");
        let new_firstname = String::from("new_firstname");
        let ts = TransactionUpdateFirstnameProfile::new(&profile_id, &new_firstname);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_firstname(), &new_firstname);
        }

        down();
    }

    fn down() {
        unsafe {
            GLOBAL_DB.clean_admin();
            GLOBAL_DB.clean_profile();
        }
    }
}