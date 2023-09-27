#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence,
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use domains::{
        repositories::skill_transaction_repository::SkillTransactionRepository,
        transaction::Transaction,
        skill::transaction_add_skill::TransactionAddSkill
    };

    #[test]
    fn test_add_skill() {
        let mut db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));

        let mut ts = TransactionAddSkill::new(
            &skill_id,
            &title,
            is_current,
            &logo
        );
        let _ = ts.execute(skill_data);
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();

        assert_eq!(skill.get_title(), &title);
        assert_eq!(skill.get_is_current(), is_current);
        assert_eq!(skill.get_logo(), &logo);
    }
}