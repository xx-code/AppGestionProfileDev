use crate::{
    repositories::skill_transaction_repository::SkillTransactionRepository, 
    errors::skill::ErrorSkill};
use entities::skill::Skill;

pub struct TransactionGetSkill<'a> {
    skill_id: &'a String
}

impl TransactionGetSkill<'_> {
    pub fn new<'a>(skill_id: &'a String) -> TransactionGetSkill<'a> {
        TransactionGetSkill { skill_id }
    }

    pub fn execute(&self, repo: &impl SkillTransactionRepository) -> Result<Skill, ErrorSkill> {
        let skill = repo.get_skill(self.skill_id);

        if skill.is_none() {
            return Err(ErrorSkill::SkillNotExist)
        }

        return Ok(skill.unwrap().clone())
    }
}

pub struct TransactionGetAllSkill {

}

impl TransactionGetAllSkill {
    pub fn new() -> TransactionGetAllSkill {
        TransactionGetAllSkill { }
    }

    pub fn execute(&self, repo: &impl SkillTransactionRepository) -> Result<Vec<Skill>, ErrorSkill> {
        let skills = repo.get_skills();

        return Ok(skills)
    } 
}