#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use domains::skill::{
        transaction_add_skill::TransactionAddSkill,
        transaction_get_skill::{
            TransactionGetSkill,
            TransactionGetAllSkill
        },
    };

    #[test]
    fn test_get_skill() {
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

        let ts = TransactionGetSkill::new(
            &skill_id
        );
        let res = ts.execute(&mut skill_data);
        let skill = res.ok().unwrap();

        assert_eq!(skill.get_title(), &title);
        assert_eq!(skill.get_is_current(), is_current);
        assert_eq!(skill.get_logo(), &logo);
    }
    #[test]
    fn test_get_no_skill() {
        let mut db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let mut skill_data = SkillTransactionPersistence::build(&mut db);

        let ts = TransactionGetSkill::new(
            &skill_id
        );
        let res = ts.execute(&mut skill_data);

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_get_all_skill() {
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
        let res = TransactionGetAllSkill::new(
        );
        let res = res.execute(&mut skill_data);
        let resumes = res.ok().unwrap();

        assert_eq!(resumes.len(), 1);
    }
}