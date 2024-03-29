use crate::entity::Entity;
use crate::link::Link;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Profile {
    admin_id: String,
    profile_id: String,
    firstname: String,
    lastname: String,
    email_address: String,
    phone_number: String,
    links: HashMap<String, Link>
}

impl Entity for Profile {
    fn get_id(&self,) -> &String {
        &self.profile_id
    }
}
impl Profile {
    pub fn new(admin_id: &String, profile_id: &String, firstname: &String, lastname: &String, email_address: &String, phone_number: &String) -> Profile {
        Profile {
            admin_id: admin_id.clone(),
            profile_id: profile_id.clone(),
            firstname: firstname.clone(),
            lastname: lastname.clone(),
            email_address: email_address.clone(),
            phone_number: phone_number.clone(),
            links: HashMap::new(),
        }
    }

    pub fn add_link(&mut self, link: Link) {
        self.links.insert(link.get_id().clone(), link);
    }
    
    pub fn get_link(&self, link_id: &String) -> Option<&Link> {
        self.links.get(link_id)
    }
    
    pub fn remove_link(&mut self, link_id: &String) {
        self.links.remove(link_id);
    }
    
    pub fn get_links(&self,) -> Vec<Link> {
        let links = self.links.values().cloned().collect();

        links
    }

    pub fn set_firstname(&mut self, firstname: &String) {
        self.firstname = firstname.to_owned()
    }

    pub fn set_lastname(&mut self, lastname: &String) {
        self.lastname = lastname.to_owned()
    }

    pub fn set_email_address(&mut self, email_address: &String) {
        self.email_address = email_address.to_owned()
    }

    pub fn set_phone_number(&mut self, phone_number: &String) {
        self.phone_number = phone_number.clone()
    }
    
    pub fn get_admin_id(&self,) -> &String {
        &self.admin_id
    }

    pub fn get_firstname(&self,) -> &String {
        &self.firstname
    }

    pub fn get_lastname(&self,) -> &String {
        &self.lastname
    }

    pub fn get_email_address(&self,) -> &String {
        &self.email_address
    }

    pub fn get_phone_number(&self,) -> &String {
        &self.phone_number
    }
}