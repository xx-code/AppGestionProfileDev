use crate::admin::Admin;
use crate::profile::Profile;

pub struct DB {
    admins: Vec<Admin>,
    profiles: Vec<Profile>
}

impl DB {
    pub fn new() -> DB {
        DB {
            admins: Vec::new(),
            profiles: Vec::new()
        }
    }

    pub unsafe fn add_new_admin(&mut self, admin: Admin) {
        self.admins.push(admin);
    }

    pub unsafe fn add_new_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
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

    fn get_index_profile(&self, profile_id: &String) -> Option<usize> {
        let mut index_got = None;
        for (index, profile) in self.profiles.iter().enumerate() {
            if profile.get_id() == profile_id {
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

    pub fn get_profile(&self, profile_id: &String) -> Option<&Profile> {
        let profile_index= self.get_index_profile(profile_id);
        if profile_index.is_none() {
            return None
        }
        return self.profiles.get(profile_index.unwrap())
    }

    pub fn clean_admin(&mut self) {
        for index in 0..self.admins.len() {
            self.admins.remove(index);
        }
    }
    pub fn clean_profile(&mut self) {
        for index in 0..self.profiles.len() {
            self.profiles.remove(index);
        }
    }
}

pub static mut GLOBAL_DB: DB = DB { admins: Vec::new(), profiles: Vec::new() };