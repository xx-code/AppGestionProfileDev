 
 #[cfg(test)]
 mod tests {
    use persistence::{
        data_persistence::DataPersistence,
        skill_transaction_persistence::SkillTransactionPersistence
    };
    use domains::{
        repositories::skill_transaction_repository::SkillTransactionRepository,
        transaction::Transaction,
        skill::{
            transaction_add_skill::TransactionAddSkill,
            transaction_update_skill::{
                TransactionUpdateTitleSkill,
                TransactionUpdateLogoSkill,
                TransactionUpdateIsCurrentSkill
            }
        }
    };

    fn setup(db: &mut DataPersistence) {
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let skill_data = Box::new(SkillTransactionPersistence::build(db));
        let mut ts = TransactionAddSkill::new(
            &skill_id,
            &title,
            is_current,
            &logo
        );
        let _ = ts.execute(skill_data);
    }

    #[test]
    fn test_update_title_skill() {
        let mut db = DataPersistence::new();
        
        setup(&mut db);

        let skill_id = String::from("Skill1");

        let new_title = String::from("new title");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateTitleSkill::new(
            &skill_id, 
            &new_title,
        );
        let _ = ts.execute(skill_data);
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();
        assert_eq!(skill.get_title(), &new_title);
    }

    #[test]
    fn test_update_logo_skill() {
        let mut db = DataPersistence::new();
        
        setup(&mut db);

        let skill_id = String::from("Skill1");

        let new_logo = String::from("new logo");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateLogoSkill::new(
            &skill_id, 
            &new_logo,
        );
        let _ = ts.execute(skill_data);
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();
        assert_eq!(skill.get_logo(), &new_logo);
    }


    #[test]
    fn test_update_is_current_skill() {
        let mut db = DataPersistence::new();
        
        setup(&mut db);

        let skill_id = String::from("Skill1");

        let is_current = true;

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateIsCurrentSkill::new(
            &skill_id, 
            is_current,
        );
        let _ = ts.execute(skill_data);
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();
        assert_eq!(skill.get_is_current(), is_current);
    }
}