use crate::boundaries::skill::{
    InteractorSkill,
    SkillID,
    RequestAddSkill,
    RequestUpdateSkill,
    ResponseGetSkill
};
use domains::{repositories::skill_transaction_repository::SkillTransactionRepository, skill::{transaction_add_skill::TransactionAddSkill, transaction_get_skill::TransactionGetSkill, transaction_delete_skill::TransactionDeleteSkill, transaction_update_skill::{TransactionUpdateTitleSkill, TransactionUpdateLogoSkill, TransactionUpdateIsCurrentSkill}}, errors::skill::ErrorSkill};

use uuid::Uuid;

pub struct Skill;

impl InteractorSkill for Skill {
    fn add_skill(&self, repo: &mut impl SkillTransactionRepository, request: RequestAddSkill) -> Result<bool, ErrorSkill> {
        let skill_id = Uuid::new_v4().to_string();
        let ts = TransactionAddSkill::new(&skill_id, request.title, request.is_current, request.logo);

        let res = ts.execute(repo);

        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn get_skill(&self, repo: &impl SkillTransactionRepository, skill_id: &SkillID) -> Result<ResponseGetSkill, ErrorSkill> {
        let ts = TransactionGetSkill::new(skill_id);
        let res = ts.execute(repo);

        match res {
            Ok(skill) => Ok(ResponseGetSkill { title: skill.get_title().clone(), is_current: skill.get_is_current().clone(), logo: skill.get_logo().clone() }),
            Err(e) => Err(e)
        }
    }

    fn update_skill(&self, repo: &mut impl SkillTransactionRepository, request: &RequestUpdateSkill) -> Result<bool, ErrorSkill> {
        let mut error_response = None;
        
        if !request.title.is_none() {
            let ts = TransactionUpdateTitleSkill::new(request.skill_id, request.title.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.logo.is_none() {
            let ts = TransactionUpdateLogoSkill::new(request.skill_id, request.logo.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.is_current.is_none() {
            let ts = TransactionUpdateIsCurrentSkill::new(request.skill_id, request.is_current.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if error_response.is_none() {
            Ok(true)
        } else {
            Err(error_response.unwrap())
        }
    }

    fn delete_skill(&self, repo: &mut impl SkillTransactionRepository, skill_id: &SkillID) -> Result<bool, ErrorSkill> {
        let mut ts = TransactionDeleteSkill::new(skill_id);
        let res = ts.execute(repo);

        match res {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }
}