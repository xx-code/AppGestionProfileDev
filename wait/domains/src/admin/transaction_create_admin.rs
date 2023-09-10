use entities::admin::Admin;
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