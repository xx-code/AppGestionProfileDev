use crate::{skill_transaction_persistence::{
    SkillTransactionPersistence,
}, 
transaction::Transaction, 
skill_transaction_repository::SkillTransactionRepository};

pub struct TransactionDeleteSkill<'a> {
    db: Box<dyn SkillTransactionRepository + 'a>,
    skill_id: &'a String,
}

impl TransactionDeleteSkill<'_> {
    pub fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String) -> TransactionDeleteSkill<'a> {
        TransactionDeleteSkill { 
            db, 
            skill_id
        }
    }
}
impl Transaction for TransactionDeleteSkill<'_> {
    fn execute(&mut self) -> () {
        self.db.delete_skill(self.skill_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transaction_add_skill::TransactionAddSkill,
        skill_transaction_persistence::SkillTransactionPersistence,
        data_persistence::DataPersistence,
        skill_transaction_repository::SkillTransactionRepository,
        transaction::Transaction,
    };

    use super::TransactionDeleteSkill;

    #[test]
    fn test_delete_skill() {
        let mut db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionAddSkill::new(
            skill_data,
            &skill_id,
            &title,
            is_current,
            &logo
        );
        ts.execute();
        drop(ts);

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionDeleteSkill::new(
            skill_data,
            &skill_id,
        );
        ts.execute();
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id);
        assert!(skill.is_none())
    }
}