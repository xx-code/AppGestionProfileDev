use crate::errors::admin::ErrorAdmin;
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

    pub fn execute(&self, repo: &mut impl AdminTransactionRepository) -> Result<(), ErrorAdmin> {
        let admin = repo.get_admin(self.admin_id);

        if !admin.is_none() {
            repo.delete_admin(self.admin_id);
            return Ok(())
        } 
        return Err(ErrorAdmin::AdminNoExist)
    }
}