use entities::{
    skill::Skill,
    entity::Entity
};
use domains::repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::data_persistence::{DataPersistence, Indexing};

pub struct SkillTransactionPersistence<'a> {
    db: &'a mut DataPersistence
}

impl Indexing for SkillTransactionPersistence<'_> {}

impl SkillTransactionPersistence<'_> {
    pub fn build<'a>(db: &'a mut DataPersistence, ) -> SkillTransactionPersistence{
        SkillTransactionPersistence { 
            db 
        }
    }
}

impl SkillTransactionRepository for SkillTransactionPersistence<'_> {
    fn add_skill(&mut self, skill: Skill) {
        self.db.skills.push(skill);
    }
    fn get_skill(&self, skill_id: &String) -> Option<&Skill>{
        let index = self.get_index(&self.db.skills, skill_id);
        
        if !index.is_none() {
            return Some(&self.db.skills[index.unwrap()])
        }
        None
    }
    fn get_skills(&self,) -> Vec<Skill> {
        return self.db.skills.clone()
    }
    fn delete_skill(&mut self, skill_id: &String) {
        let index = self.get_index( &self.db.skills, skill_id);

        if !index.is_none() { 
            self.db.skills.remove(index.unwrap());
        }
        
    }
    fn update_skill(&mut self, skill: Skill) {
        let index = self.get_index(&self.db.skills, skill.get_id()).unwrap();
        self.db.skills[index] = skill;
    }
}