use repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, skill::ErrorSkill}
};


pub struct TransactionDeleteSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
}

impl TransactionDeleteSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String) -> TransactionDeleteSkill<'a> {
        TransactionDeleteSkill { 
            db, 
            skill_id
        }
    }
}
impl Transaction for TransactionDeleteSkill<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let skill = self.db.get_skill(self.skill_id);

        if !skill.is_none() {
            self.db.delete_skill(self.skill_id);
            Ok(())
        } else {
            Err(Box::new(ErrorSkill::SkillNotExist))
        }
    }
}