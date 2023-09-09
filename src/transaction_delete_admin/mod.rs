use crate::admin_transaction_persistence::AdminTransactionPersistence;
use crate::admin_transaction_repository::AdminTransactionRepository;
use crate::transaction::Transaction;

pub struct TransactionDeleteAdmin<'a> {
    db: &'a mut AdminTransactionPersistence<'a>, 
    admin_id: &'a String
}

impl TransactionDeleteAdmin<'_>{
    pub fn new<'a> (db: &'a mut AdminTransactionPersistence<'a>, admin_id: &'a String) -> TransactionDeleteAdmin<'a> {
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

#[cfg(test)]
mod tests {
    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction, 
        data_persistence::DataPersistence,
        transaction_delete_admin::TransactionDeleteAdmin,
        admin_transaction_persistence::AdminTransactionPersistence, admin_transaction_repository::AdminTransactionRepository,
    };

    #[test]
    fn test_delete_admin() {
        let admin_id = String::from("admin1");
        let username = String::from("username");
        let password = String::from("password");

        let mut db = DataPersistence::new();
        let mut admin_data = AdminTransactionPersistence::build(&mut db);

        let mut ts = TransactionCreateAdmin::new(
            &mut admin_data,
            &admin_id, 
            &username, 
            &password
        );
        ts.execute();

        let mut admin_data = AdminTransactionPersistence::build(&mut db);
        let mut ts = TransactionDeleteAdmin::new(&mut admin_data, &admin_id);
        ts.execute();
        
        let admin_data = AdminTransactionPersistence::build(&mut db);
        let admin = admin_data.get_admin(&admin_id);
        assert!(admin.is_none());
    }

}