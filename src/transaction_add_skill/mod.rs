#[cfg(test)]
mod tests {
    use crate::data_persistence::DataPersistence;

    #[test]
    fn test_add_skill() {
        let db = DataPersistence::new();
        let skill_id = String::from("Skill1");

        let title = String::from("title skill");
        let is_current = false;
        let logo = String::from("logo");

        let mut skill_data = SkillTransactionPersistence::build(&mut db);

        let mut ts = TransactionAddSkill::new(
            &skill_data,
            skill_id,
            title,
            is_current,
            logo
        );
        ts.execute();

        let skill_data = SkillTransactionPersistence::build(&mut db);
        let skill = skill_data.get_skill(skill_id);

        assert_eq!(skill.get_title(), title);
        assert_eq!(skill.get_is_current(), is_current);
        assert_eq!(skill.get_logo(), logo);
    }
}