#[cfg(test)]
mod tests {
    use crate::{
        skill_transaction_persistence::SkillTransactionPersistence, 
        data_persistence::DataPersistence, 
        transaction_add_skill::TransactionAddSkill,
        transaction::Transaction,
        skill_transaction_repository::SkillTransactionRepository,
    };

    fn setup(db: &mut DataPersistence) {
        
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let skill_data = Box::new(SkillTransactionPersistence::build(db));
        let mut ts = TransactionAddSkill::new(
            skill_data,
            &skill_id,
            &title,
            is_current,
            &logo
        );
        ts.execute();
    }

    fn test_update_title_skill() {
        let mut db = DataPersistence::new();
        
        setup(&mut db);

        let skill_id = String::from("Skill1");

        let new_title = String::from("new title");

        let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateTitleSkill::new(
            skill_data,
            &skill_id, 
            &new_title,
        );
        ts.execute();
        drop(ts);

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(&skill_id).unwrap();
        assert_eq!(skill.get_title(), &new_title);
    }
}