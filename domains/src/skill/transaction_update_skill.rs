use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::errors::skill::ErrorSkill;
pub struct TransactionUpdateTitleSkill<'a> {
    skill_id: &'a String,
    title: &'a String
}
impl TransactionUpdateTitleSkill<'_> {
    pub fn new<'a>(skill_id: &'a String, title: &'a String) -> TransactionUpdateTitleSkill<'a> {
        TransactionUpdateTitleSkill {
            skill_id, 
            title
        }
    }

    pub fn execute(&self, repo: &mut impl SkillTransactionRepository) -> Result<(), ErrorSkill> {
        let skill =  repo.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_title(self.title);
            repo.update_skill(skill);
            Ok(())
        } else {
            Err(ErrorSkill::SkillNotExist)
        }
    }
}

pub struct TransactionUpdateIsCurrentSkill<'a> {
    skill_id: &'a String,
    is_current: bool
}
impl TransactionUpdateIsCurrentSkill<'_> {
    pub fn new<'a>(skill_id: &'a String, is_current: bool) -> TransactionUpdateIsCurrentSkill<'a> {
        TransactionUpdateIsCurrentSkill { 
            skill_id, 
            is_current
        }
    }

    pub fn execute(&self, repo: &mut impl SkillTransactionRepository) -> Result<(), ErrorSkill> {
        let skill = repo.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_is_current(self.is_current);
            repo.update_skill(skill);
            Ok(())
        } else {
            Err(ErrorSkill::SkillNotExist)
        }
    }
}


pub struct TransactionUpdateLogoSkill<'a> {
    skill_id: &'a String,
    logo: &'a String
}
impl TransactionUpdateLogoSkill<'_> {
    pub fn new<'a>(skill_id: &'a String, logo: &'a String) -> TransactionUpdateLogoSkill<'a> {
        TransactionUpdateLogoSkill { 
            skill_id, 
            logo
        }
    }

    pub fn execute(&self, repo: &mut impl SkillTransactionRepository) -> Result<(), ErrorSkill> {
        let skill =  repo.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_logo(self.logo);
            repo.update_skill(skill);
            Ok(())
        } else {
            Err(ErrorSkill::SkillNotExist)
        }
    }
}