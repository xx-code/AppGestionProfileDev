#[cfg(test)]
mod test {
    use domains::{
        repositories::profile_transaction_repository::ProfileTransactionRepository,
        profile::transaction_delete_profile::TransactionDeleteProfile,
        transaction::Transaction,
    };
    use persistence::{
        data_persistence::DataPersistence,
        profile_transaction_persistence::ProfileTransactionPersistence,
    };
    use crate::profile::test_transaction_update_profile::test::setup_profile;
    
    #[test]
    fn test_delete_profile() {
        let mut db = DataPersistence::new();
        setup_profile(&mut db);
        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let mut ts = TransactionDeleteProfile::new(&profile_id);
        let _ = ts.execute(profile_data);

        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id);

        assert!(profile.is_none());
    }
    #[test]
    fn no_delete_test_profile_not_exist() {
        let mut db = DataPersistence::new();
        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let mut ts = TransactionDeleteProfile::new(&profile_id);
        let res = ts.execute(profile_data);
        drop(ts);

        assert_eq!(res.is_ok(), false)
    }
}