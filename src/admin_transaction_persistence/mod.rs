use crate::admin_transaction_repository::AdminTransactionRepository;
use crate::data_persistence::{DataPersistence, Indexing};
use crate::admin::Admin;
pub struct AdminTransactionPersistence<'a> {
    db: &'a mut DataPersistence
}

impl AdminTransactionPersistence<'_> {
    pub fn build<'a>(db: &'a mut DataPersistence) -> AdminTransactionPersistence<'a>{
        AdminTransactionPersistence { db }
    }
}
impl Indexing for AdminTransactionPersistence<'_>{}
impl AdminTransactionRepository for AdminTransactionPersistence<'_> {
    fn create_admin(&mut self, admin: Admin) {
        self.db.admins.push(admin)
    }
    fn get_admin(&self, admin_id: &String) -> Option<&Admin> {
        let index = self.get_index(&self.db.admins, admin_id);
        
        if !index.is_none() {
           return Some(&self.db.admins[index.unwrap()])
        }
        None
    }
    fn delete_admin(&mut self, admin_id: &String) {
        let index = self.get_index(&self.db.admins, admin_id);
        
        if !index.is_none() {
            self.db.admins.remove(index.unwrap());
        }
    }
}