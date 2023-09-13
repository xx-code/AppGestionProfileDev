use crate::{transaction::Transaction, errors::{ErrorDomain, admin::ErrorAdmin}};
use crate::repositories::admin_transaction_repository::AdminTransactionRepository;

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
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>>{
        let admin = self.db.get_admin(self.admin_id);

        if !admin.is_none() {
            self.db.delete_admin(self.admin_id);
            return Ok(())
        } 
        return Err(Box::new(ErrorAdmin::AdminNoExist))
    }
}