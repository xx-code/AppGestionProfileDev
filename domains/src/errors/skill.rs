use crate::errors::ErrorDomain;

#[derive(Clone, Copy)]
pub enum ErrorSkill {
    SkillNotExist
}

impl ErrorDomain for ErrorSkill {}