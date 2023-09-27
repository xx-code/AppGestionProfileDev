#[cfg(test)]
mod tests {
    use domains::{
        repositories::admin_transaction_repository::AdminTransactionRepository,
        admin::transaction_create_admin::TransactionCreateAdmin
    };
    use persistence::{
        data_persistence::DataPersistence,
        admin_transaction_persistence::AdminTransactionPersistence, 
    };

    #[test]
    fn test_creation_admin(){
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let mut admin_data = AdminTransactionPersistence::build(&mut db);

        let new_admin = TransactionCreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );

        let _ = new_admin.execute(&mut admin_data);
        drop(new_admin);
        
        let admin_data = AdminTransactionPersistence::build(&mut db);
        let admin = admin_data.get_admin(&admin_id).unwrap();

        assert_eq!(admin.get_username(), &username);
        assert_eq!(admin.get_password(), &password);
    }

    #[test]
    fn test_avoid_to_create_admin_that_already_exist() {
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let mut admin_data = AdminTransactionPersistence::build(&mut db);

        let new_admin = TransactionCreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );

        let _ = new_admin.execute(&mut admin_data);
        drop(new_admin);

        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password valid");

        let mut admin_data = AdminTransactionPersistence::build(&mut db);
        let new_admin = TransactionCreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );
        
        let val = new_admin.execute(&mut admin_data).is_ok();
        assert_eq!(val, false);
    }
}