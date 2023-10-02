use domains::{repositories::skill_transaction_repository::SkillTransactionRepository, errors::skill::ErrorSkill};

pub struct RequestAddSkill<'a> {
    pub title: &'a String,
    pub is_current: bool,
    pub logo: &'a String,
}

pub struct RequestUpdateSkill<'a> {
    pub skill_id: &'a String,
    pub title: Option<&'a String>,
    pub is_current: Option<bool>,
    pub logo: Option<&'a String>
}

pub struct ResponseGetSkill {
    pub title: String,
    pub is_current: bool,
    pub logo: String
}

pub type SkillID = String;

pub trait InteractorSkill {
    fn add_skill(&self, repo: &mut impl SkillTransactionRepository, request: RequestAddSkill) -> Result<bool, ErrorSkill>;
    fn get_skill(&self, repo: &impl SkillTransactionRepository, skill_id: &SkillID) -> Result<ResponseGetSkill, ErrorSkill>;
    fn update_skill(&self, repo: &mut impl SkillTransactionRepository, request: &RequestUpdateSkill) -> Result<bool, ErrorSkill>;
    fn delete_skill(&self, repo: &mut impl SkillTransactionRepository, skill_id: &SkillID) -> Result<bool, ErrorSkill>;
}