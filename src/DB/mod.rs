use std::collections::HashMap;
use crate::admin::Admin;

pub struct DB {
    admins: Vec<Admin>
}

impl DB {
    pub fn new() -> DB {
        DB {
            admins: Vec::new()
        }
    }

    pub unsafe fn add_new_admin(&mut self, admin: Admin) {
        self.admins.push(admin);
    }

    pub fn get_admin(&self, admin_id: &String) -> Option<&Admin> {
        let mut admin_get = None;
        for admin in &self.admins {
            if admin.get_id() == admin_id {
                admin_get = Some(admin);
            }
        }
        admin_get
    }
}

pub static mut GLOBAL_DB: DB = DB { admins: Vec::new() };