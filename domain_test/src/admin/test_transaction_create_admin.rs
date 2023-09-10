#[cfg(test)]
mod tests {
    use domains::{
        transaction::Transaction,
        admin::transaction_create_admin::TransactionCreateAdmin
    };
    use repositories::admin_transaction_repository::AdminTransactionRepository;
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
        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));

        let mut new_admin = TransactionCreateAdmin::new(
            admin_data,
            &admin_id, 
            &username,
            &password
        );

        new_admin.execute();
        drop(new_admin);
        
        let admin_data = AdminTransactionPersistence::build(&mut db);
        let admin = admin_data.get_admin(&admin_id).unwrap();

        assert_eq!(admin.get_username(), &username);
        assert_eq!(admin.get_password(), &password);
    }
}