use std::borrow::BorrowMut;

use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, skill::ErrorSkill}
};


pub struct TransactionDeleteSkill<'a> {
    skill_id: &'a String,
}

impl TransactionDeleteSkill<'_> {
    pub fn new<'a>(skill_id: &'a String) -> TransactionDeleteSkill<'a> {
        TransactionDeleteSkill {
            skill_id
        }
    }
}
impl Transaction<(), ErrorSkill, Box<dyn SkillTransactionRepository>> for TransactionDeleteSkill<'_> {
    fn execute(&mut self, repo: Box<dyn SkillTransactionRepository>) -> Result<(), ErrorSkill> {
        let repo = repo.borrow_mut();
        let skill = repo.get_skill(self.skill_id);

        if !skill.is_none() {
            repo.delete_skill(self.skill_id);
            Ok(())
        } else {
            Err(ErrorSkill::SkillNotExist)
        }
    }
}