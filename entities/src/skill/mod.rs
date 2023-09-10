use crate::entity::Entity;

#[derive(Debug, Clone)]
pub struct Skill {
    skill_id: String,
    title: String,
    is_current: bool,
    logo: String,
}

impl Entity for Skill {
    fn get_id(&self,) -> &String {
        &self.skill_id
    }
}

impl Skill {
    pub fn new(skill_id: &String, title: &String, is_current: bool, logo: &String) -> Skill {
        Skill { 
            skill_id: skill_id.clone(), 
            title: title.clone(), 
            is_current, 
            logo: logo.clone()
        }
    }

    pub fn set_title(&mut self, title: &String) {
        self.title = title.to_owned();
    }

    pub fn set_is_current(&mut self, is_current: bool) {
        self.is_current = is_current;
    }

    pub fn set_logo(&mut self, logo: &String) {
        self.logo = logo.to_owned();
    }

    pub fn get_title(&self,) -> &String {
        &self.title
    }

    pub fn get_is_current(&self,) -> bool {
        self.is_current
    }

    pub fn get_logo(&self,) -> &String {
        &self.logo
    }
}
