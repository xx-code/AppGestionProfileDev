use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;

pub struct DeleteAdmin<'a> {
    admin_id: &'a String
}

impl DeleteAdmin<'_>{
    pub fn new(admin_id: &String) -> DeleteAdmin {
        DeleteAdmin { admin_id }
    } 
}

impl Transaction for DeleteAdmin<'_> {
    fn execute(&self) -> () {
        unsafe {
            GLOBAL_DB.delete_admin(self.admin_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        create_admin::CreateAdmin, 
        transaction::Transaction, 
        DB::GLOBAL_DB,
        delete_admin::DeleteAdmin,
    };

    #[test]
    fn test_delete_admin() {
        let admin_id = String::from("admin1");
        let username = String::from("username");
        let password = String::from("password");

        let ts = CreateAdmin::new(&admin_id, &username, &password);
        ts.execute();

        let ts = DeleteAdmin::new(&admin_id);
        ts.execute();
        
        unsafe {
            let admin = GLOBAL_DB.get_admin(&admin_id);
            assert!(admin.is_none());
        }
    }
}