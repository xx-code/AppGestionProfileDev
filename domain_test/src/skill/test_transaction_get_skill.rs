#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use domains::{
        transaction::Transaction,
        skill::{
            transaction_add_skill::TransactionAddSkill,
            transaction_get_skill::{
                TransactionGetSkill,
                TransactionGetAllSkill
            },
        }
    };

    #[test]
    fn test_get_skill() {
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
        let _ = ts.execute();
        drop(ts);

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetSkill::new(
            skill_data,
            &skill_id
        );
        let res = ts.execute();
        let skill = res.ok().unwrap();

        assert_eq!(skill.get_title(), &title);
        assert_eq!(skill.get_is_current(), is_current);
        assert_eq!(skill.get_logo(), &logo);
    }
    #[test]
    fn test_get_no_skill() {
        let mut db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetSkill::new(
            skill_data,
            &skill_id
        );
        let res = ts.execute();

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_get_all_skill() {
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
        let _ = ts.execute();
        drop(ts);

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut res = TransactionGetAllSkill::new(
            skill_data,
        );
        let res = res.execute();
        let resumes = res.ok().unwrap();

        assert_eq!(resumes.len(), 1);
    }
}