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

    fn get_index_admin(&self, admin_id: &String) -> Option<usize> {
        let mut index_got = None;
        for (index, admin) in self.admins.iter().enumerate() {
            if admin.get_id() == admin_id {
                index_got = Some(index);
            }
        }
        index_got
    }

    pub fn get_admin(&self, admin_id: &String) -> Option<&Admin> {
        let admin_index= self.get_index_admin(admin_id);
        if admin_index.is_none() {
            return None
        }
        return self.admins.get(admin_index.unwrap())
    }

    pub fn delete_admin(&mut self, admin_id: &String) {
        let index = self.get_index_admin(admin_id).unwrap();
        self.admins.remove(index);
    }
}

pub static mut GLOBAL_DB: DB = DB { admins: Vec::new() };