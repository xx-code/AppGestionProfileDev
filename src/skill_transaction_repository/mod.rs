use crate::skill::Skill;
pub trait SkillTransactionRepository {
    fn add_skill(&mut self, skill: Skill);
    fn get_skill(&self, skill_id: &String) -> Option<&Skill>;
}