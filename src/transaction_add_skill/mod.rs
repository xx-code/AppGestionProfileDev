use crate::{
    skill_transaction_persistence::SkillTransactionPersistence,
    transaction::Transaction, skill_transaction_repository::SkillTransactionRepository,
    skill::Skill
};

pub struct TransactionAddSkill<'a> {
    db: &'a mut SkillTransactionPersistence<'a>,
    skill_id: &'a String,
    title: &'a String,
    is_current: bool,
    logo: &'a String,
}
impl TransactionAddSkill<'_> {
    pub fn new<'a>(db: &'a mut SkillTransactionPersistence<'a>, skill_id: &'a String, title: &'a String, is_current: bool, logo: &'a String) -> TransactionAddSkill<'a> {
        TransactionAddSkill { 
            db, 
            skill_id, 
            title, 
            is_current, 
            logo 
        }
    }
}
impl Transaction for TransactionAddSkill<'_> {
    fn execute(&mut self) -> () {
        let skill = Skill::new(
            self.skill_id, 
            self.title, 
            self.is_current, 
            self.logo
        );
        self.db.add_skill(skill);
    }
}
#[cfg(test)]
mod tests {
    use crate::data_persistence::DataPersistence;
    use crate::skill_transaction_persistence::SkillTransactionPersistence;
    use crate::skill_transaction_repository::SkillTransactionRepository;
    use crate::transaction::Transaction;
    use super::TransactionAddSkill;

    #[test]
    fn test_add_skill() {
        let mut db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let mut skill_data = SkillTransactionPersistence::build(&mut db);

        let mut ts = TransactionAddSkill::new(
            &mut skill_data,
            &skill_id,
            &title,
            is_current,
            &logo
        );
        ts.execute();

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();

        assert_eq!(skill.get_title(), &title);
        assert_eq!(skill.get_is_current(), is_current);
        assert_eq!(skill.get_logo(), &logo);
    }
}