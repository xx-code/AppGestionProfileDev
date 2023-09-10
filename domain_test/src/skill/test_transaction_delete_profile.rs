#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence,
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use repositories::skill_transaction_repository::SkillTransactionRepository;
    use domains::{
        transaction::Transaction,
        skill::{
            transaction_add_skill::TransactionAddSkill,
            transaction_delete_skill::TransactionDeleteSkill
        }
    };

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