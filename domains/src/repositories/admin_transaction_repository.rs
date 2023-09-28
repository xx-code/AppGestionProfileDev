use entities::admin::Admin;

pub trait AdminTransactionRepository {
    fn create_admin(&mut self, admin: Admin);
    fn get_admin(&self, admin_id: &String) -> Option<&Admin>;
    fn get_admin_by_username(&self, username: &String) -> Option<&Admin>;
    fn delete_admin(&mut self, admin_id: &String);
    fn is_already_exist(&self, username: &String) -> bool;
}