#[cfg(test)]
pub mod test {
    use domains::{
        repositories::profile_transaction_repository::ProfileTransactionRepository,
        admin::transaction_create_admin::TransactionCreateAdmin,
        profile::transaction_create_profile::TransactionCreateProfile,
        profile::transaction_update_profile::{
            TransactionUpdateEmailAddressProfile,
            TransactionUpdateFirstnameProfile,
            TransactionUpdateLastnameProfile,
            TransactionUpdatePhoneNumberProfile,
        },
        transaction::Transaction,
    };
    use persistence::{
        data_persistence::DataPersistence,
        profile_transaction_persistence::ProfileTransactionPersistence,
        admin_transaction_persistence::AdminTransactionPersistence
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
    fn test_update_firstname_profile() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_firstname = String::from("new_firstname");
        let mut ts = TransactionUpdateFirstnameProfile::new(
            profile_data,
            &profile_id, 
            &new_firstname
        );
        let _ = ts.execute();
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

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_lastname = String::from("new_lastname");
        let mut ts = TransactionUpdateLastnameProfile::new(
            profile_data,
            &profile_id, 
            &new_lastname
        );
        let _ = ts.execute();
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

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_email = String::from("New email");
        let mut ts = TransactionUpdateEmailAddressProfile::new(
            profile_data, 
            &profile_id, 
            &new_email
        );
        let _ = ts.execute();
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

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let profile_id = String::from("profile1");
        let new_phone: String = String::from("482982569");
        let mut ts = TransactionUpdatePhoneNumberProfile::new(
            profile_data, 
            &profile_id, 
            &new_phone
        );
        let _ = ts.execute();
        drop(ts);
       
        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();
        
        assert_eq!(profile.get_phone_number(), &new_phone);
    }

}