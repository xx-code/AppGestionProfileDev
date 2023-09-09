use crate::{
    data_persistence::{DataPersistence, Indexing}, 
    skill_transaction_repository::SkillTransactionRepository,
    skill::Skill,
};

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
}