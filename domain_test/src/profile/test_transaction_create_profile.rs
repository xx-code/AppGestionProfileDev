#[cfg(test)]
pub mod tests {
    use domains::{
        repositories::profile_transaction_repository::ProfileTransactionRepository,
        admin::transaction_create_admin::TransactionCreateAdmin,
        profile::transaction_create_profile::TransactionCreateProfile
    };
    use persistence::{
        data_persistence::DataPersistence,
        profile_transaction_persistence::ProfileTransactionPersistence,
        admin_transaction_persistence::AdminTransactionPersistence
    };
    use entities::entity::Entity;

    #[test]
    pub fn test_transaction_create_profile() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut db = DataPersistence::new();
        let mut admin_data = AdminTransactionPersistence::build(&mut db);

        let ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        let _ = ts.execute(&mut admin_data);
        drop(ts);

        let mut profile_data = ProfileTransactionPersistence::build(&mut db);

        let ts = TransactionCreateProfile::new(
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        let _ = ts.execute(&mut profile_data);
        drop(ts);
        

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();

        assert_eq!(profile.get_id(), &profile_id);
        assert_eq!(profile.get_firstname(), &firstname);
        assert_eq!(profile.get_lastname(), &lastname);
        assert_eq!(profile.get_email_address(), &email_address);
        assert_eq!(profile.get_phone_number(), &phone_number);
    }
    #[test]
    fn test_if_admin_exist_before_create_admin() {
        let admin_id = String::from("admin_3");

        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut db = DataPersistence::new();

        let mut profile_data = ProfileTransactionPersistence::build(&mut db);

        let ts = TransactionCreateProfile::new(
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        let res = ts.execute(&mut profile_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
}