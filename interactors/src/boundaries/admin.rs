use domains::{
    repositories::admin_transaction_repository::AdminTransactionRepository,
    errors::admin::ErrorAdmin
};

pub struct RequestCreateAdmin<'a> {
    pub username: &'a String,
    pub password: &'a String
}
pub struct ResponseGetAdmin {
    pub username: String,
    pub password: String
}

pub type AdminId = String;

pub trait InteractorAdmin {
    fn create_admin_user(&self, repo: &mut impl AdminTransactionRepository,  request: &RequestCreateAdmin) -> Result<AdminId, ErrorAdmin>;
    fn delete_admin_user(&self, repo: &mut impl AdminTransactionRepository, admin_id: &String) -> Result<bool, ErrorAdmin>;
    fn get_admin_user(&self, repo: &impl AdminTransactionRepository, admin_id: &String) -> Result<ResponseGetAdmin, ErrorAdmin>;
    fn get_admin_by_username(&self, repo: &impl AdminTransactionRepository, username: &String) -> Result<ResponseGetAdmin, ErrorAdmin>;
} 