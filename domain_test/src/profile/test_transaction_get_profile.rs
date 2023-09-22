#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        profile_transaction_persistence::ProfileTransactionPersistence,
        admin_transaction_persistence::AdminTransactionPersistence
    };
    use domains::{
        transaction::Transaction,
        admin::transaction_create_admin::TransactionCreateAdmin,
        profile::transaction_create_profile::TransactionCreateProfile,
        profile::transaction_get_profile::TransactionGetProfile
    };
    pub fn setup_admin(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let admin_data = AdminTransactionPersistence::build(db);
        
        let mut ts = TransactionCreateAdmin::new(
            Box::new(admin_data),
            &admin_id,
            &username,
            &password,
        );
        let _ = ts.execute();
    }

    pub fn setup_profile(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let profile_data = Box::new(ProfileTransactionPersistence::build(db));
        let mut ts = TransactionCreateProfile::new(
            profile_data,
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        let _ = ts.execute();
    }
    #[test]
    fn test_get_profile() {
        let profile_id = String::from("profile1");

        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetProfile::new(
            profile_data,
            &profile_id
        );
        let res = ts.execute();
        let profile = res.ok().unwrap();

        assert_eq!(profile.get_firstname(), &String::from("first"));
        assert_eq!(profile.get_lastname(), &String::from("last"));
        assert_eq!(profile.get_email_address(), &String::from("address"));
        assert_eq!(profile.get_phone_number(), &String::from("07056389"));
    }
    #[test]
    fn test_get_profile_not_exist() {
        let profile_id = String::from("profile1");

        let mut db = DataPersistence::new();
        let profile_data =  Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetProfile::new(
            profile_data,
            &profile_id
        );
        let res = ts.execute();

        assert_eq!(res.is_ok(), false);
    }
}