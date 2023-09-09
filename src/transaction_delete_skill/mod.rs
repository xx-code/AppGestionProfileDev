use crate::skill_transaction_persistence::SkillTransactionPersistence;

pub struct TransactionDeleteSkill<'a> {
    db: &'a mut SkillTransactionPersistence<'a>,
    skill: &'a String,
}

#[cfg(test)]
mod tests {
    use crate::{
        transaction_add_skill::TransactionAddSkill,
        skill_transaction_persistence::SkillTransactionPersistence,
        data_persistence::DataPersistence,
        transaction::Transaction,
    };

    #[test]
    fn test_delete_skill() {
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

        let mut skill_data = SkillTransactionPersistence::build(&mut db);
        let mut ts = TransactionDeleteSkill::new(
            &mut skill_data,
            &skill_id,
        );
        ts.execute();

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id);
        assert!(skill.is_none())
    }
}