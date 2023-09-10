#[cfg(test)]
mod test {
    use persistence::{
        data_persistence::DataPersistence,
        project_transaction_persistence::ProjectTransactionPersistence,
    };
    use repositories::project_transaction_repository::ProjectTransactionRepository;
    use time::Date;
    use domains::{
        transaction::Transaction,
        project::transaction_create_project::{
            TransactionCreateCurrentProject,
            TransactionCreateCompletProject
        }
    };
    #[test]
    fn test_create_project_current() {
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
        drop(transaction);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_title(), &title);
        assert_eq!(project.get_description(), &description);
        assert_eq!(project.get_date_start(), &date_start);
    }

    #[test]
    fn test_create_project_complet() {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::January, 4).unwrap();

        let mut db = DataPersistence::new();
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut transaction = TransactionCreateCompletProject::new(
            project_data,
            &project_id,
            &title,
            &description,
            &date_start,
            &date_end
        );
        transaction.execute();
        drop(transaction);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_title(), &title);
        assert_eq!(project.get_description(), &description);
        assert_eq!(project.get_date_start(), &date_start);
        assert_eq!(project.get_date_end().unwrap(), date_end);
    }
}