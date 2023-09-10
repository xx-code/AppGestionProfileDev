use crate::transaction::Transaction;
use repositories::admin_transaction_repository::AdminTransactionRepository;

pub struct TransactionDeleteAdmin<'a> {
    db: Box<dyn AdminTransactionRepository + 'a>, 
    admin_id: &'a String
}

impl TransactionDeleteAdmin<'_>{
    pub fn new<'a> (db: Box<dyn AdminTransactionRepository + 'a>, admin_id: &'a String) -> TransactionDeleteAdmin<'a> {
        TransactionDeleteAdmin { 
            db, 
            admin_id 
        }
    } 
}

impl Transaction for TransactionDeleteAdmin<'_> {
    fn execute(&mut self) -> () {
        self.db.delete_admin(self.admin_id);
    }
}