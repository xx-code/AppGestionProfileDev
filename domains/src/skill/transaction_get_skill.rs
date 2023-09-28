use crate::{
    repositories::skill_transaction_repository::SkillTransactionRepository, 
    errors::skill::ErrorSkill
};

#[derive(Debug, PartialEq, Clone)]
pub struct SkillDto {
    pub title: String,
    pub is_current: bool,
    pub logo: String,
}

pub struct TransactionGetSkill<'a> {
    skill_id: &'a String
}

impl TransactionGetSkill<'_> {
    pub fn new<'a>(skill_id: &'a String) -> TransactionGetSkill<'a> {
        TransactionGetSkill { skill_id }
    }

    pub fn execute(&self, repo: &impl SkillTransactionRepository) -> Result<SkillDto, ErrorSkill> {
        let skill = repo.get_skill(self.skill_id);

        if skill.is_none() {
            return Err(ErrorSkill::SkillNotExist)
        }

        let res = skill.unwrap();

        return Ok(SkillDto { 
            title: res.get_title().clone(), 
            is_current: res.get_is_current(), 
            logo: res.get_logo().clone() 
        })
    }
}

pub struct TransactionGetAllSkill {

}

impl TransactionGetAllSkill {
    pub fn new() -> TransactionGetAllSkill {
        TransactionGetAllSkill { }
    }

    pub fn execute(&self, repo: &impl SkillTransactionRepository) -> Result<Vec<SkillDto>, ErrorSkill> {
        let skills = repo.get_skills();

        let mut skill_dto = Vec::new();

        for skill in skills {
            skill_dto.push(SkillDto {
                title: skill.get_title().clone(),
                is_current: skill.get_is_current(),
                logo: skill.get_logo().clone()
            });
        }

        return Ok(skill_dto)
    } 
}