#[cfg(test)]
mod test {
    use domains::{
        profile::transaction_delete_profile::TransactionDeleteProfile,
        transaction::Transaction,
    };
    use persistence::{
        data_persistence::DataPersistence,
        profile_transaction_persistence::ProfileTransactionPersistence,
    };
    use repositories::profile_transaction_repository::ProfileTransactionRepository;
    use crate::profile::test_transaction_update_profile::test::setup_profile;
    
    #[test]
    fn test_delete_profile() {
        let mut db = DataPersistence::new();
        setup_profile(&mut db);
        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let mut ts = TransactionDeleteProfile::new(profile_data, &profile_id);
        ts.execute();

        drop(ts);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id);

        assert!(profile.is_none());
    }
}