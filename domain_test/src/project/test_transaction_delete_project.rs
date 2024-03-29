#[cfg(test)]
mod test {
    use persistence::{
        data_persistence::DataPersistence,
        project_transaction_persistence::ProjectTransactionPersistence,
     };
     use domains::{
        repositories::project_transaction_repository::ProjectTransactionRepository,
        project::{
            transaction_create_project::TransactionCreateCurrentProject,
            transaction_delete_project::TransactionDeleteProject,
         }
     };
     use time::Date;
    #[test]
    fn test_transaction_delete_project() {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();

        let mut db = DataPersistence::new();
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let transaction = TransactionCreateCurrentProject::new(
            &project_id,
            &title,
            &description,
            &date_start
        );
        let _ =transaction.execute(&mut project_data);
        drop(transaction);

        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let transaction = TransactionDeleteProject::new(
            &project_id,
        );
        let _ = transaction.execute(&mut project_data);
        drop(transaction);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id);

        assert!(project.is_none())
    }
    
    #[test]
    fn test_not_delete_project_no_exist() {
        let project_id = String::from("project1");

        let mut db = DataPersistence::new();
        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let transaction = TransactionDeleteProject::new(
            &project_id,
        );
        let res = transaction.execute(&mut project_data);
        drop(transaction);

        assert_eq!(res.is_ok(), false);
    }
}