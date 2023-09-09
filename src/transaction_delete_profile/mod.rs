use crate::{
    transaction::Transaction, 
    profile_transaction_persistence::ProfileTransactionPersistence, 
    profile_transaction_repository::ProfileTransactionRepository
};

struct TransactionDeleteProfile<'a> {
    db: &'a mut ProfileTransactionPersistence<'a>,
    profile_id: &'a String
}

impl TransactionDeleteProfile<'_> {
    fn new<'a>(db: &'a mut ProfileTransactionPersistence<'a>, profile_id: &'a String) -> TransactionDeleteProfile<'a> {
        TransactionDeleteProfile { 
            db, 
            profile_id
        }
    }
}

impl Transaction for TransactionDeleteProfile<'_> {
    fn execute(&mut self) -> () {
        self.db.delete_profile(self.profile_id)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        transaction::Transaction, 
        transaction_delete_profile::TransactionDeleteProfile,
        transaction_update_profile::test::setup_profile, 
        data_persistence::DataPersistence, 
        profile_transaction_persistence::ProfileTransactionPersistence,
        profile_transaction_repository::ProfileTransactionRepository,
    };

    #[test]
    fn test_delete_profile() {
        let mut db = DataPersistence::new();
        setup_profile(&mut db);
        let mut profile_data = ProfileTransactionPersistence::build(&mut db);

        let profile_id = String::from("profile1");
        let mut ts = TransactionDeleteProfile::new(&mut profile_data, &profile_id);
        ts.execute();

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id);

        assert!(profile.is_none());
    }
}