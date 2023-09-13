use entities::skill::Skill;
pub trait SkillTransactionRepository {
    fn add_skill(&mut self, skill: Skill);
    fn get_skill(&self, skill_id: &String) -> Option<&Skill>;
    fn delete_skill(&mut self, skill_id: &String);
    fn update_skill(&mut self, skill: Skill);
}