#[cfg(test)]
mod tests {
    use domains::{
        transaction::Transaction,
        admin::transaction_create_admin::TransactionCreateAdmin,
        admin::transaction_delete_admin::TransactionDeleteAdmin
    };
    use repositories::admin_transaction_repository::AdminTransactionRepository;
    use persistence::{
        data_persistence::DataPersistence,
        admin_transaction_persistence::AdminTransactionPersistence, 
    };


    #[test]
    fn test_delete_admin() {
        let admin_id = String::from("admin1");
        let username = String::from("username");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));

        let mut ts = TransactionCreateAdmin::new(
            admin_data,
            &admin_id, 
            &username, 
            &password
        );
        ts.execute();
        drop(ts);

        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));
        let mut ts = TransactionDeleteAdmin::new(admin_data, &admin_id);
        ts.execute();
        drop(ts);
        
        let admin_data = AdminTransactionPersistence::build(&mut db);
        let admin = admin_data.get_admin(&admin_id);
        assert!(admin.is_none());
    }

}