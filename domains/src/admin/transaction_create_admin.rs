use entities::admin::Admin;
use crate::repositories::admin_transaction_repository::AdminTransactionRepository;
use crate::errors::admin::ErrorAdmin;

pub struct TransactionCreateAdmin<'a> {
    admin_id: &'a String,
    username: &'a String,
    password: &'a String,
}

impl TransactionCreateAdmin<'_> {
    pub fn new<'a>(admin_id: &'a String, username: &'a String, password: &'a String) -> TransactionCreateAdmin<'a> {
        TransactionCreateAdmin {
            admin_id, 
            username, 
            password 
        }
    }

    pub fn execute(&self, repo: &mut impl AdminTransactionRepository) -> Result<(), ErrorAdmin> {
        let new_admin = Admin::new(
            self.admin_id,
            self.username,
            self.password,
        );

        if repo.is_already_exist(self.username) == false {
            repo.create_admin(new_admin);
            Ok(())
        } else {
            Err(ErrorAdmin::AdminAlreadyExist)
        }
    }
}