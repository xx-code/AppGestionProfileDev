#[cfg(test)]
mod test {
    use persistence::data_persistence::DataPersistence;
    use time::Date;
    #[test]
    fn test_create_project_current() {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3);

        let mut db = DataPersistence::new();
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let transaction = TransactionCreateCurrentProject::new(
            project_data,
            &project_id,
            &title,
            &description,
            &date_start
        );
        transaction.execute();
        drop(transaction);

        let project = project_data.get_profile(&project_id);
        
        assert_eq!(project.get_title(), &title);
        assert_eq!(project.get_description(), &description);
        assert_eq!(project.get_date_start(), &date_start);
    }
}