#[cfg(test)]
mod test {
    use persistence::{
       data_persistence::DataPersistence,
       project_transaction_persistence::ProjectTransactionPersistence,
    };
    use domains::{
        transaction::Transaction,
        project::{
           transaction_create_project::TransactionCreateCompletProject,
           transaction_update_project::{
            TransactionUpdateProjectTitle,
            TransactionUpdateProjectDescription,
            TransactionUpdateProjectDateStart,
            TransactionUpdateProjectDateEnd
           }
        }
    };
    use repositories::project_transaction_repository::ProjectTransactionRepository;
    use time::Date;

    fn setup(db: &mut DataPersistence) {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::January, 4).unwrap();

        let project_data = Box::new(ProjectTransactionPersistence::build(db));

        let mut transaction = TransactionCreateCompletProject::new(
            project_data,
            &project_id,
            &title,
            &description,
            &date_start,
            &date_end
        );
        let _ = transaction.execute();
        drop(transaction);
    }
    #[test]
    fn test_update_title_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let title = String::from("title project");
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectTitle::new(
            project_data,
            &project_id,
            &title
        );
        let _ = ts.execute();
        drop(ts);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_title(), &title);
    }

    #[test]
    fn test_update_description_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let descripiton = String::from("new _description");
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectDescription::new(
            project_data,
            &project_id,
            &descripiton
        );
        let _ = ts.execute();
        drop(ts);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_description(), &descripiton);
    }

    #[test]
    fn test_update_date_start_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let date_start = Date::from_calendar_date(2021, time::Month::April, 3).unwrap();
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectDateStart::new(
            project_data,
            &project_id,
            &date_start
        );
        let _ =  ts.execute();
        drop(ts);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_date_start(), &date_start);
    }

    #[test]
    fn test_update_date_end_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let date_end = Date::from_calendar_date(2022, time::Month::February, 3).unwrap();
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectDateEnd::new(
            project_data,
            &project_id,
            &date_end
        );
        let _ = ts.execute();
        drop(ts);

        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_date_end().unwrap(), date_end);
    }
    #[test]
    fn test_not_accept_date_start_more_than_date_end() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let new_date_start = Date::from_calendar_date(2023, time::Month::January, 3).unwrap();
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectDateStart::new(
            project_data,
            &project_id,
            &new_date_start
        );
        let res = ts.execute();
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_not_accept_date_end_less_than_date_start() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let new_date_end = Date::from_calendar_date(2019, time::Month::January, 3).unwrap();
        
        let project_data = Box::new(ProjectTransactionPersistence::build(&mut db));

        let mut ts = TransactionUpdateProjectDateEnd::new(
            project_data,
            &project_id,
            &new_date_end
        );
        let res = ts.execute();
        drop(ts);
        
        assert_eq!(res.is_ok(), false);
    }    
}