#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence,
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use domains::{
        repositories::skill_transaction_repository::SkillTransactionRepository,
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

        let mut skill_data = SkillTransactionPersistence::build(&mut db);
        let ts = TransactionAddSkill::new(
            &skill_id,
            &title,
            is_current,
            &logo
        );
        let _ = ts.execute(&mut skill_data);
        drop(ts);

        let mut skill_data = SkillTransactionPersistence::build(&mut db);
        let mut ts = TransactionDeleteSkill::new(
            &skill_id,
        );
        let _ = ts.execute(&mut skill_data);
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id);
        assert!(skill.is_none())
    }

    #[test]
    fn test_no_delete_skill_if_not_exist() {
        let skill_id = String::from("Skill1");
        let mut db = DataPersistence::new();
        let mut skill_data = SkillTransactionPersistence::build(&mut db);
        let mut ts = TransactionDeleteSkill::new(
            &skill_id,
        );
        let res = ts.execute(&mut skill_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
}