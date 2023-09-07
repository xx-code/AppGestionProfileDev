#[cfg(test)]
mod test {
    use crate::transaction_create_profile::tests::test_transaction_create_profile;
    use crate::DB::GLOBAL_DB;
    #[test]
    fn test_update_firstname_profile() {
        test_transaction_create_profile();
        let profile_id = String::from("profile1");
        let new_firstname = String::from("new_firstname");
        let ts = TransactionUpdateFirstnameProfile::new(&profile_id);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id).unwrap();
            assert_eq!(profile.get_firstname(), &new_firstname);
        }
    } 
}