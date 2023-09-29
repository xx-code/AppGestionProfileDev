use domains::{repositories::skill_transaction_repository::SkillTransactionRepository, errors::skill::ErrorSkill};

pub struct RequestAddSkill<'a> {
    title: &'a String,
    is_current: bool,
    logo: &'a String,
}

pub struct RequestUpdateSkill<'a> {
    title: Option<&'a String>,
    is_current: Option<bool>,
    logo: Option<&'a String>
}

pub struct ResponseGetSkill {
    title: String,
    is_current: bool,
    logo: String
}

pub type SkillID = String;

pub trait InteractorSkill {
    fn add_skill(&self, repo: &mut impl SkillTransactionRepository, request: RequestAddSkill) -> Result<bool, ErrorSkill>;
    fn get_skill(&self, repo: &impl SkillTransactionRepository, skill_id: &SkillID) -> Result<ResponseGetSkill, ErrorSkill>;
    fn update_skill(&self, repo: &mut impl SkillTransactionRepository, request: &RequestUpdateSkill) -> Result<bool, ErrorSkill>;
    fn delete_skill(&self, repo: &mut impl SkillTransactionRepository, skill_id: &SkillID) -> Result<bool, ErrorSkill>;
}