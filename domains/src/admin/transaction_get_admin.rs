use entities::admin::Admin;
use crate::repositories::admin_transaction_repository::AdminTransactionRepository;
use crate::errors::admin::ErrorAdmin;
use crate::transaction::Transaction;
use crate::errors::ErrorDomain;

pub struct TransactionGetAdmin<'a> {
    db: Box<dyn AdminTransactionRepository + 'a>,
    admin_id: &'a String,
}

impl TransactionGetAdmin<'_> {
    pub fn new<'a>(db: Box<dyn AdminTransactionRepository + 'a>, admin_id: &'a String) -> TransactionGetAdmin<'a> {
        TransactionGetAdmin {
            db,
            admin_id,
        }
    }
}


impl Transaction<Admin> for TransactionGetAdmin<'_> {
    fn execute(&mut self) -> Result<Admin, Box<dyn ErrorDomain>> {
        let admin = self.db.get_admin(self.admin_id);

        if admin.is_none() {
            return Err(Box::new(ErrorAdmin::AdminNoExist))
        }

        return Ok(admin.unwrap().clone())
    }
}