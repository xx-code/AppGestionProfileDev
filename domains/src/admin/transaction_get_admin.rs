use entities::admin::Admin;
use crate::repositories::admin_transaction_repository::AdminTransactionRepository;
use crate::errors::admin::ErrorAdmin;
use crate::transaction::Transaction;
use crate::errors::ErrorDomain;

pub struct TransactionGetAdmin<'a> {
    admin_id: &'a String,
}

impl TransactionGetAdmin<'_> {
    pub fn new<'a>(admin_id: &'a String) -> TransactionGetAdmin<'a> {
        TransactionGetAdmin {
            admin_id,
        }
    }
}


impl Transaction<Admin, ErrorAdmin, Box<dyn AdminTransactionRepository> > for TransactionGetAdmin<'_> {
    fn execute(&mut self, repo: Box<dyn AdminTransactionRepository> ) -> Result<Admin, ErrorAdmin> {
        let admin = repo.get_admin(self.admin_id);

        if admin.is_none() {
            return Err(ErrorAdmin::AdminNoExist)
        }

        return Ok(admin.unwrap().clone())
    }
}