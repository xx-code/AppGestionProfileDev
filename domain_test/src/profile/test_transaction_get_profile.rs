#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        profile_transaction_persistence::ProfileTransactionPersistence,
        admin_transaction_persistence::AdminTransactionPersistence
    };
    use domains::{
        admin::transaction_create_admin::TransactionCreateAdmin,
        profile::transaction_create_profile::TransactionCreateProfile,
        profile::transaction_get_profile::{TransactionGetProfile, ProfileDto}
    };
    pub fn setup_admin(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let mut admin_data = AdminTransactionPersistence::build(db);
        
        let ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        let _ = ts.execute(&mut admin_data);
    }

    pub fn setup_profile(db: &mut DataPersistence) {
        let admin_id = String::from("admin_1");
        let profile_id = String::from("profile1");
        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let mut profile_data = ProfileTransactionPersistence::build(db);
        let ts = TransactionCreateProfile::new(
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        let _ = ts.execute(&mut profile_data);
    }
    #[test]
    fn test_get_profile() {
        let profile_id = String::from("profile1");

        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile_dto = ProfileDto {
            firstname: String::from("first"),
            lastname: String::from("last"),
            email_address: String::from("address"),
            phone_number: String::from("07056389")
        };

        let ts = TransactionGetProfile::new(
            &profile_id
        );
        let res = ts.execute(&profile_data);
        let profile = res.ok().unwrap();

        assert_eq!(profile_dto, profile);
    }
    #[test]
    fn test_get_profile_not_exist() {
        let profile_id = String::from("profile1");

        let mut db = DataPersistence::new();
        let profile_data =  ProfileTransactionPersistence::build(&mut db);

        let ts = TransactionGetProfile::new(
            &profile_id
        );
        let res = ts.execute(&profile_data);

        assert_eq!(res.is_ok(), false);
    }
}