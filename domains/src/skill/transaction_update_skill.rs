use crate::repositories::skill_transaction_repository::SkillTransactionRepository;
use crate::{transaction::Transaction, errors::{ErrorDomain, skill::ErrorSkill}};
pub struct TransactionUpdateTitleSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
    title: &'a String
}
impl TransactionUpdateTitleSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, title: &'a String) -> TransactionUpdateTitleSkill<'a> {
        TransactionUpdateTitleSkill { 
            db, 
            skill_id, 
            title
        }
    }
}
impl Transaction for TransactionUpdateTitleSkill<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let skill =  self.db.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_title(self.title);
            self.db.update_skill(skill);
            Ok(())
        } else {
            Err(Box::new(ErrorSkill::SkillNotExist))
        }
    }
}

pub struct TransactionUpdateIsCurrentSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
    is_current: bool
}
impl TransactionUpdateIsCurrentSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, is_current: bool) -> TransactionUpdateIsCurrentSkill<'a> {
        TransactionUpdateIsCurrentSkill { 
            db, 
            skill_id, 
            is_current
        }
    }
}
impl Transaction for TransactionUpdateIsCurrentSkill<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let skill =  self.db.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_is_current(self.is_current);
            self.db.update_skill(skill);
            Ok(())
        } else {
            Err(Box::new(ErrorSkill::SkillNotExist))
        }
    }
}

pub struct TransactionUpdateLogoSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
    logo: &'a String
}
impl TransactionUpdateLogoSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, logo: &'a String) -> TransactionUpdateLogoSkill<'a> {
        TransactionUpdateLogoSkill { 
            db, 
            skill_id, 
            logo
        }
    }
}
impl Transaction for TransactionUpdateLogoSkill<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let skill =  self.db.get_skill(self.skill_id);
        
        if !skill.is_none() {
            let mut skill = skill.unwrap().clone();
            skill.set_logo(self.logo);
            self.db.update_skill(skill);
            Ok(())
        } else {
            Err(Box::new(ErrorSkill::SkillNotExist))
        }
    }
}