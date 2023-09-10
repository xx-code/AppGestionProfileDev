use entities::{
    admin::Admin,
    profile::Profile,
    resume::Resume,
    skill::Skill,
    entity::Entity,
};
/*use crate::profile::Profile;
use crate::resume::Resume;
use crate::entity::Entity;
use crate::skill::Skill;*/

#[derive(Debug, Clone)]
pub struct DataPersistence {
    pub admins: Vec<Admin>,
    pub profiles: Vec<Profile>,
    pub resumes: Vec<Resume>,
    pub skills: Vec<Skill>,
}

impl DataPersistence {
    pub fn new() -> DataPersistence {
        DataPersistence { admins: Vec::new(), profiles: Vec::new(), resumes: Vec::new(), skills: Vec::new() }
    }
}

pub trait Indexing {
    fn get_index<T: Entity>(&self, array: &Vec<T>, id: &String) -> Option<usize> {
        let mut index_got = None;
        for (index, element) in array.iter().enumerate() {
            if element.get_id() == id {
                index_got = Some(index);
            }
        }
        index_got
    }
}