use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;

pub struct TransactionDeleteAdmin<'a> {
    admin_id: &'a String
}

impl TransactionDeleteAdmin<'_>{
    pub fn new(admin_id: &String) -> TransactionDeleteAdmin {
        TransactionDeleteAdmin { admin_id }
    } 
}

impl Transaction for TransactionDeleteAdmin<'_> {
    fn execute(&self) -> () {
        unsafe {
            GLOBAL_DB.delete_admin(self.admin_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction, 
        DB::GLOBAL_DB,
        transaction_delete_admin::TransactionDeleteAdmin,
    };

    #[test]
    fn test_delete_admin() {
        let admin_id = String::from("admin1");
        let username = String::from("username");
        let password = String::from("password");

        let ts = TransactionCreateAdmin::new(&admin_id, &username, &password);
        ts.execute();

        let ts = TransactionDeleteAdmin::new(&admin_id);
        ts.execute();
        
        unsafe {
            let admin = GLOBAL_DB.get_admin(&admin_id);
            assert!(admin.is_none());
        }
    }

    #[test]
    fn down() {
        unsafe {
            GLOBAL_DB.clean_admin()
        }
    }
}