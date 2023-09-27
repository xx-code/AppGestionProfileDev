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
            TransactionAddLinkProfile,
            TransactionDeleteLinkProfile
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

        let admin_data = Box::new(AdminTransactionPersistence::build(db));
        
        let mut ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        let _ = ts.execute(admin_data);
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
            &admin_id,
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        let _ = ts.execute(profile_data);
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
            &profile_id, 
            &new_firstname
        );
        let _ = ts.execute(profile_data);
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
            &profile_id, 
            &new_lastname
        );
        let _ = ts.execute(profile_data);
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
            &profile_id, 
            &new_email
        );
        let _ = ts.execute(profile_data);
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
            &profile_id, 
            &new_phone
        );
        let _ = ts.execute(profile_data);
        drop(ts);
       
        let profile_data = ProfileTransactionPersistence::build(&mut db);
        let profile = profile_data.get_profile(&profile_id).unwrap();
        
        assert_eq!(profile.get_phone_number(), &new_phone);
    }
    #[test]
    fn test_add_link_to_profile() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let link_id = String::from("LINK1");
        let link_title = String::from("Title link");
        let link_address = String::from("linknks@gmail.com");

        let profile_id = String::from("profile1");
        let mut ts = TransactionAddLinkProfile::new(
            &profile_id,
            &link_id,
            &link_title,
            &link_address
        );
        let _ = ts.execute(profile_data);
        drop(ts);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));
        let profile = profile_data.get_profile(&profile_id).unwrap();
        let link = profile.get_link(&link_id).unwrap();
        let links = profile.get_links();

        assert_eq!(link.get_title(), &link_title);
        assert_eq!(link.get_address(), &link_address);
        assert_eq!(links.len(), 1);
    }
    #[test]
    fn test_to_delete_profile_link() {
        let mut db = DataPersistence::new();
        setup_admin(&mut db);
        setup_profile(&mut db);

        let profile_id = String::from("profile1");

        let link_id = String::from("LINK1");
        let link_title = String::from("Title link");
        let link_address = String::from("linknks@gmail.com");

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionAddLinkProfile::new(
            &profile_id,
            &link_id,
            &link_title,
            &link_address
        );

        let _ = ts.execute(profile_data);
        drop(ts);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionDeleteLinkProfile::new(
            &profile_id,
            &link_id
        );

        let res = ts.execute(profile_data);
        drop(ts);

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));
        let profile = profile_data.get_profile(&profile_id).unwrap();
        let links = profile.get_links();

        assert_eq!(res.is_ok(), true);
        assert_eq!(links.len(), 0);
    }
    #[test]
    fn test_do_not_delete() {
        let mut db = DataPersistence::new();

        let profile_id = String::from("profile1");
        let link_id = String::from("LINK1");

        let profile_data = Box::new(ProfileTransactionPersistence::build(&mut db));

        let mut ts = TransactionDeleteLinkProfile::new(
            &profile_id,
            &link_id
        );

        let res = ts.execute(profile_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
}