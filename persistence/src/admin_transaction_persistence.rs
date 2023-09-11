use crate::data_persistence::{DataPersistence, Indexing};
use repositories::admin_transaction_repository::AdminTransactionRepository;
use entities::admin::Admin;
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

    fn is_already_exist(&self, username: &String) -> bool {
        for admin in &self.db.admins {
            if admin.get_username() == username {
                return true
            }
        }
        false
    }
}