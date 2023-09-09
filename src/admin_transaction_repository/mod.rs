use crate::admin::Admin;

pub trait AdminTransactionRepository {
    fn create_admin(&mut self, admin: Admin);
    fn get_admin(&self, admin_id: &String) -> Option<&Admin>;
    fn delete_admin(&mut self, admin_id: &String);
}