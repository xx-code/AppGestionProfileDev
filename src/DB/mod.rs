use crate::admin::Admin;
use crate::entity::Entity;
use crate::profile::Profile;
use crate::resume::Resume;

pub struct DB {
    admins: Vec<Admin>,
    profiles: Vec<Profile>,
    resumes: Vec<Resume>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            admins: Vec::new(),
            profiles: Vec::new(),
            resumes: Vec::new(),
        }
    }

    pub unsafe fn add_new_admin(&mut self, admin: Admin) {
        self.admins.push(admin);
    }

    pub unsafe fn add_new_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    pub unsafe fn add_resume(&mut self, resume: Resume) {
        self.resumes.push(resume);
    }

    fn get_index<T: Entity>(&self, array: &Vec<T>, id: &String) -> Option<usize> {
        let mut index_got = None;
        for (index, element) in array.iter().enumerate() {
            if element.get_id() == id {
                index_got = Some(index);
            }
        }
        index_got
    }

    pub fn get_admin(&self, admin_id: &String) -> Option<&Admin> {
        let admin_index = self.get_index(&self.admins, admin_id);
        if admin_index.is_none() {
            return None
        }
        return self.admins.get(admin_index.unwrap())
    }

    pub fn delete_admin(&mut self, admin_id: &String) {
        let index = self.get_index(&self.admins, admin_id).unwrap();
        self.admins.remove(index);
    }

    pub fn delete_profile(&mut self, profile_id: &String) {
        let index = self.get_index(&self.profiles, profile_id).unwrap();
        self.profiles.remove(index);
    }

    pub fn get_profile(&self, profile_id: &String) -> Option<& Profile> {
        let profile_index= self.get_index(&self.profiles, profile_id);
        if profile_index.is_none() {
            return None
        }
        self.profiles.get(profile_index.unwrap())
    }

    pub fn update_profile(&mut self, profile_id: &String, update_info:(&str, &String)) {
        let idx_profile = self.get_index(&self.profiles, profile_id);
        if !idx_profile.is_none() {
            let old_profile = self.profiles.get(idx_profile.unwrap()).unwrap().clone();
            let mut new_profile = Profile::new(old_profile.get_admin_id(), old_profile.get_id(), old_profile.get_firstname(), old_profile.get_lastname(), old_profile.get_email_address(), old_profile.get_phone_number()); 
            let (field, new_value) = update_info;
            match field {
                "firstname" => new_profile.set_firstname(new_value), 
                "lastname" => new_profile.set_lastname(new_value),
                "email_address" => new_profile.set_email_address(new_value), 
                "phone_number" => new_profile.set_phone_number(new_value),
                _ => todo!("return an error")
            }
            self.profiles.remove(idx_profile.unwrap());
            for a in &self.profiles {
               println!("d {}", a.get_firstname()); 
            }
            
            unsafe {
              self.add_new_profile(new_profile);   
            };
        } else {
            println!("Add error no found profile")
        }
    }

    pub fn get_resume(&self, resume_id: &String) -> Option<&Resume> {
        let resume_index= self.get_index(&self.resumes, resume_id);
        if resume_index.is_none() {
            return None
        }
        self.resumes.get(resume_index.unwrap())
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

    pub fn clean(&mut self) {
        self.clean_admin();
        self.clean_profile();
    }
}

pub static mut GLOBAL_DB: DB = DB { admins: Vec::new(), profiles: Vec::new(), resumes: Vec::new() };