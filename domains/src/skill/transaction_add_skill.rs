use std::borrow::BorrowMut;

use entities::skill::Skill;
use crate::errors::skill::ErrorSkill;
use crate::{transaction::Transaction, errors::ErrorDomain};
use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
pub struct TransactionAddSkill<'a> {
    skill_id: &'a String,
    title: &'a String,
    is_current: bool,
    logo: &'a String,
}
impl TransactionAddSkill<'_> {
    pub fn new<'a>(skill_id: &'a String, title: &'a String, is_current: bool, logo: &'a String) -> TransactionAddSkill<'a> {
        TransactionAddSkill { 
            skill_id, 
            title, 
            is_current, 
            logo 
        }
    }
}
impl Transaction<(), ErrorSkill, Box<dyn SkillTransactionRepository>> for TransactionAddSkill<'_> {
    fn execute(&mut self, repo: Box<dyn SkillTransactionRepository>) -> Result<(), ErrorSkill> {
        let repo = repo.borrow_mut();
        let skill = Skill::new(
            self.skill_id, 
            self.title, 
            self.is_current, 
            self.logo
        );
        repo.add_skill(skill);
        Ok(())
    }
}