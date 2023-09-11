use crate::errors::ErrorDomain;

pub enum ErrorSkill {
    SkillNotExist
}

impl ErrorDomain for ErrorSkill {}