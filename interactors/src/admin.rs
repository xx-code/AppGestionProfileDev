use domains::admin::transaction_delete_admin::TransactionDeleteAdmin;
use domains::admin::transaction_get_admin::TransactionGetAdmin;
use domains::admin::{transaction_create_admin::TransactionCreateAdmin, transaction_get_admin::TransactionGetAdminByUsername};
use domains::repositories::admin_transaction_repository::AdminTransactionRepository;
use domains::errors::admin::ErrorAdmin;
use uuid::Uuid;
use crate::boundaries::admin::{InteractorAdmin, AdminId, RequestCreateAdmin,  ResponseGetAdmin};
pub struct Admin;

impl InteractorAdmin for Admin {
    fn create_admin_user(&self, repo: &mut impl AdminTransactionRepository,  request: &RequestCreateAdmin) -> Result<AdminId, ErrorAdmin> {
        let admin_id = Uuid::new_v4().to_string();
        // cryp password
        let ts = TransactionCreateAdmin::new(&admin_id, request.username, request.password);
        let res = ts.execute(repo);

        match res {
            Ok(_) => Ok(admin_id),
            Err(e) => Err(e) 
        }
    }

    fn get_admin_by_username(&self, repo: &impl AdminTransactionRepository, username: &String) -> Result<ResponseGetAdmin, ErrorAdmin> {
        let ts = TransactionGetAdminByUsername::new(username);
        let res = ts.execute(repo);

        match res {
            Ok(admin) => Ok(ResponseGetAdmin { username: admin.get_username().clone(), password: admin.get_password().clone() }), 
            Err(e) => Err(e)
        }
    }

    fn get_admin_user(&self, repo: &impl AdminTransactionRepository, admin_id: &String) -> Result<ResponseGetAdmin, ErrorAdmin> {
        let ts = TransactionGetAdmin::new(admin_id);
        let res = ts.execute(repo);

        match res {
            Ok(admin ) => Ok(ResponseGetAdmin { username: admin.get_username().clone(), password: admin.get_password().clone() }),
            Err(e) => Err(e)
        }
    }

    fn delete_admin_user(&self, repo: &mut impl AdminTransactionRepository, admin_id: &String) -> Result<bool, ErrorAdmin> {
        let ts = TransactionDeleteAdmin::new(admin_id);
        let res = ts.execute(repo);

        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }
}