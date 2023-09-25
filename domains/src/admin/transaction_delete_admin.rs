use std::borrow::BorrowMut;

use crate::{transaction::Transaction, errors::{ErrorDomain, admin::ErrorAdmin}};
use crate::repositories::admin_transaction_repository::AdminTransactionRepository;

pub struct TransactionDeleteAdmin<'a> {
    admin_id: &'a String
}

impl TransactionDeleteAdmin<'_>{
    pub fn new<'a> (admin_id: &'a String) -> TransactionDeleteAdmin<'a> {
        TransactionDeleteAdmin {
            admin_id 
        }
    } 
}

impl Transaction<(), ErrorAdmin, Box<dyn AdminTransactionRepository>> for TransactionDeleteAdmin<'_> {
    fn execute(&mut self, repo: Box<dyn AdminTransactionRepository> ) -> Result<(), ErrorAdmin>{
        let repo = repo.borrow_mut();
        
        let admin = repo.get_admin(self.admin_id);

        if !admin.is_none() {
            repo.delete_admin(self.admin_id);
            return Ok(())
        } 
        return Err(ErrorAdmin::AdminNoExist)
    }
}