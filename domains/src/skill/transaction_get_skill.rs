use crate::{
    repositories::skill_transaction_repository::SkillTransactionRepository, 
    transaction::Transaction, errors::{skill::ErrorSkill, ErrorDomain}};
use entities::skill::Skill;

pub struct TransactionGetSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String
}

impl TransactionGetSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String) -> TransactionGetSkill<'a> {
        TransactionGetSkill { db, skill_id }
    }
}

impl Transaction<Skill, ErrorSkill> for TransactionGetSkill<'_> {
    fn execute(&mut self) -> Result<Skill, ErrorSkill> {
        let skill = self.db.get_skill(self.skill_id);

        if skill.is_none() {
            return Err(ErrorSkill::SkillNotExist)
        }

        return Ok(skill.unwrap().clone())
    }
}

pub struct TransactionGetAllSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>
}

impl TransactionGetAllSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>) -> TransactionGetAllSkill {
        TransactionGetAllSkill { db }
    }
}

impl Transaction<Vec<Skill>, ErrorSkill> for TransactionGetAllSkill<'_> {
    fn execute(&mut self) -> Result<Vec<Skill>, ErrorSkill> {
        let skills = self.db.get_skills();

        return Ok(skills)
    }
}