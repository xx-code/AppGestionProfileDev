use crate::entity::Entity;
#[derive(Debug, Clone)]
pub struct Link {
    link_id: String,
    title: String,
    address: String    
}

impl Entity for Link {
    fn get_id(&self,) -> &String {
        &self.link_id
    }
}

impl Link {
    pub fn new(link_id: &String, title: &String, address: &String) -> Link {
        Link {
            link_id: link_id.clone(),
            title: title.clone(),
            address: address.clone()
        }
    }

    pub fn set_title(&mut self, title: &String) {
        self.title = title.clone()
    }

    pub fn set_address(&mut self, address: &String) {
        self.address = address.clone()
    }

    pub fn get_title(&self,) -> &String {
        &self.title
    }

    pub fn get_address(&self,) -> &String {
        &self.address
    }
}