#[cfg(test)]
mod test {
    use persistence::{
       data_persistence::DataPersistence,
       project_transaction_persistence::ProjectTransactionPersistence,
    };
    use domains::{
        transaction::Transaction,
        project::transaction_create_project::TransactionCreateCurrentProject
    };
    use time::Date;

    fn setup(db: &mut DataPersistence) {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();

        let mut db = DataPersistence::new();
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut transaction = TransactionCreateCurrentProject::new(
            project_data,
            &project_id,
            &title,
            &description,
            &date_start
        );
        transaction.execute();
    }
    #[test]
    fn test_update_title_project() {
        let mut db = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let title = String::from("title");
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectTitle::new(
            project_data,
            &project_id,
            &title
        );
        ts.execute();
        drop(ts);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_title(), &title);
    }
}