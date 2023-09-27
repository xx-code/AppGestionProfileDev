use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::errors::skill::ErrorSkill;


pub struct TransactionDeleteSkill<'a> {
    skill_id: &'a String,
}

impl TransactionDeleteSkill<'_> {
    pub fn new<'a>(skill_id: &'a String) -> TransactionDeleteSkill<'a> {
        TransactionDeleteSkill {
            skill_id
        }
    }

    pub fn execute(&mut self, repo: &mut impl SkillTransactionRepository) -> Result<(), ErrorSkill> {
        let skill = repo.get_skill(self.skill_id);

        if !skill.is_none() {
            repo.delete_skill(self.skill_id);
            Ok(())
        } else {
            Err(ErrorSkill::SkillNotExist)
        }
    }
}