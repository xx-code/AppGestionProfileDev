use crate::admin::{Admin, self};
use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;
pub struct CreateAdmin<'a> {
    admin_id: &'a String,
    username: &'a String,
    password: &'a String,
}

impl CreateAdmin<'_> {
    pub fn new<'a>(admin_id: &'a String, username: &'a String, password: &'a String) -> CreateAdmin<'a> {
        CreateAdmin { 
            admin_id: admin_id, 
            username: username, 
            password: password 
        }
    }
}
impl Transaction for CreateAdmin<'_> {
    fn execute<'a>(&'a self) -> () {
        let new_admin = Admin::new(
            self.admin_id,
            self.username,
            self.password,
        );

        unsafe {
           GLOBAL_DB.add_new_admin(new_admin);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateAdmin, };
    use crate::{DB::GLOBAL_DB, transaction::Transaction};
    
    #[test]
    fn test_creation_admin(){
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let new_admin = CreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );

        new_admin.execute();
        
        unsafe {
            let admin = GLOBAL_DB.get_admin(&admin_id).unwrap();

            assert_eq!(admin.get_username(), &username);
            assert_eq!(admin.get_password(), &password);
        }
    }

    #[test]
    fn down() {
        unsafe {
            GLOBAL_DB.clean_admin()
        }
    }
}