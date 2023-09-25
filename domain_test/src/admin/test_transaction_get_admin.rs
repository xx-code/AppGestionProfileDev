#[cfg(test)]
mod tests {
    use domains::{
        admin::transaction_create_admin::TransactionCreateAdmin,
        admin::transaction_get_admin::TransactionGetAdmin,
        transaction::Transaction
    };
    use persistence::{
        data_persistence::DataPersistence,
        admin_transaction_persistence::AdminTransactionPersistence,
    };
    #[test]
    fn test_get_admin_does_exist() {
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));

        let mut new_admin = TransactionCreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );

        let _ = new_admin.execute(admin_data);
        drop(new_admin);
        
        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetAdmin::new(
            &admin_id
        );

        let res = ts.execute(admin_data);
        let admin = res.ok().unwrap();

        assert_eq!(admin.get_username(), &username);
        assert_eq!(admin.get_password(), &password);
    }
    #[test]
    fn test_admin_not_exist() {
        let admin_id = String::from("admin1");

        let mut db = DataPersistence::new();
        let admin_data = Box::new(AdminTransactionPersistence::build(&mut db));
        
        let mut ts = TransactionGetAdmin::new(
            &admin_id
        );

        let res = ts.execute(admin_data);
        
        assert_eq!(res.is_ok(), false);
    }
}