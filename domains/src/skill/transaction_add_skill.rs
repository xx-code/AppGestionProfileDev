use entities::skill::Skill;
use crate::{transaction::Transaction, errors::ErrorDomain};
use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
pub struct TransactionAddSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
    title: &'a String,
    is_current: bool,
    logo: &'a String,
}
impl TransactionAddSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, title: &'a String, is_current: bool, logo: &'a String) -> TransactionAddSkill<'a> {
        TransactionAddSkill { 
            db, 
            skill_id, 
            title, 
            is_current, 
            logo 
        }
    }
}
impl Transaction for TransactionAddSkill<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let skill = Skill::new(
            self.skill_id, 
            self.title, 
            self.is_current, 
            self.logo
        );
        self.db.add_skill(skill);
        Ok(())
    }
}