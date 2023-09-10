use crate::admin::{Admin};
use crate::admin_transaction_repository::AdminTransactionRepository;
use crate::transaction::Transaction;
use crate::admin_transaction_persistence::AdminTransactionPersistence;
pub struct TransactionCreateAdmin<'a> {
    db: Box<dyn AdminTransactionRepository + 'a>,
    admin_id: &'a String,
    username: &'a String,
    password: &'a String,
}

impl TransactionCreateAdmin<'_> {
    pub fn new<'a>(db: Box<dyn AdminTransactionRepository + 'a>, admin_id: &'a String, username: &'a String, password: &'a String) -> TransactionCreateAdmin<'a> {
        TransactionCreateAdmin {
            db,
            admin_id, 
            username, 
            password 
        }
    }
}
impl Transaction for TransactionCreateAdmin<'_> {
    fn execute(&mut self) -> () {
        let new_admin = Admin::new(
            self.admin_id,
            self.username,
            self.password,
        );

        self.db.create_admin(new_admin);
    }
}

#[cfg(test)]
mod tests {
    use super::{TransactionCreateAdmin, };
    use crate::{
        transaction::Transaction,
        data_persistence::DataPersistence,
        admin_transaction_persistence::AdminTransactionPersistence, 
        admin_transaction_repository::AdminTransactionRepository,
    };

    #[test]
    fn test_creation_admin(){
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let mut admin_data = Box::new(AdminTransactionPersistence::build(&mut db));

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