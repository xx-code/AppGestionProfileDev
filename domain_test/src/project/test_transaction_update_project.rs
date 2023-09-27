#[cfg(test)]
mod test {
    use persistence::{
       data_persistence::DataPersistence,
       project_transaction_persistence::ProjectTransactionPersistence,
    };
    use domains::{
        repositories::project_transaction_repository::ProjectTransactionRepository,
        project::{
           transaction_create_project::TransactionCreateCompletProject,
           transaction_update_project::{
            TransactionUpdateProjectTitle,
            TransactionUpdateProjectDescription,
            TransactionUpdateProjectDateStart,
            TransactionUpdateProjectDateEnd,
            TransactionAddLinkProject,
            TransactionDeleteLinkProject
           }
        }
    };
    use time::Date;

    fn setup(db: &mut DataPersistence) {
        let title = String::from("project");
        let project_id = String::from("project1");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::January, 4).unwrap();

        let mut project_data = ProjectTransactionPersistence::build(db);

        let transaction = TransactionCreateCompletProject::new(
            &project_id,
            &title,
            &description,
            &date_start,
            &date_end
        );
        let _ = transaction.execute(&mut project_data);
        drop(transaction);
    }
    #[test]
    fn test_update_title_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let title = String::from("title project");
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectTitle::new(
            &project_id,
            &title
        );
        let _ = ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_title(), &title);
    }

    #[test]
    fn test_update_description_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let descripiton = String::from("new _description");
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectDescription::new(
            &project_id,
            &descripiton
        );
        let _ = ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_description(), &descripiton);
    }

    #[test]
    fn test_update_date_start_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let date_start = Date::from_calendar_date(2021, time::Month::April, 3).unwrap();
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectDateStart::new(
            &project_id,
            &date_start
        );
        let _ =  ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_date_start(), &date_start);
    }

    #[test]
    fn test_update_date_end_project() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let date_end = Date::from_calendar_date(2022, time::Month::February, 3).unwrap();
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectDateEnd::new(
            &project_id,
            &date_end
        );
        let _ = ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();

        assert_eq!(project.get_date_end().unwrap(), date_end);
    }
    #[test]
    fn test_not_accept_date_start_more_than_date_end() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let new_date_start = Date::from_calendar_date(2023, time::Month::January, 3).unwrap();
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectDateStart::new(
            &project_id,
            &new_date_start
        );
        let res = ts.execute(&mut project_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_not_accept_date_end_less_than_date_start() {
        let mut db: DataPersistence = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");
        let new_date_end = Date::from_calendar_date(2019, time::Month::January, 3).unwrap();
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionUpdateProjectDateEnd::new(
            &project_id,
            &new_date_end
        );
        let res = ts.execute(&mut project_data);
        drop(ts);
        
        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_to_add_link_project() {
        let mut db = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");

        let link_id = String::from("LINK1");
        let link_title = String::from("Title link");
        let link_address = String::from("linknks@gmail.com");

        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionAddLinkProject::new(
            &project_id,
            &link_id,
            &link_title,
            &link_address
        );

        let _ = ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();
        let link = project.get_link(&link_id).unwrap();
        let links = project.get_links();

        assert_eq!(link.get_title(), &link_title);
        assert_eq!(link.get_address(), &link_address);
        assert_eq!(links.len(), 1);
    }
    #[test]
    fn test_to_delete_project_link() {
        let mut db = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");

        let link_id = String::from("LINK1");
        let link_title = String::from("Title link");
        let link_address = String::from("linknks@gmail.com");

        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionAddLinkProject::new(
            &project_id,
            &link_id,
            &link_title,
            &link_address
        );

        let _ = ts.execute(&mut project_data);
        drop(ts);

        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionDeleteLinkProject::new(
            &project_id,
            &link_id
        );

        let res = ts.execute(&mut project_data);
        drop(ts);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let project = project_data.get_project(&project_id).unwrap();
        let links = project.get_links();

        assert_eq!(res.is_ok(), true);
        assert_eq!(links.len(), 0);
    }
    #[test]
    fn test_no_delete_link_project_not_exist() {
        let mut db = DataPersistence::new();
        setup(&mut db);

        let project_id = String::from("project1");

        let link_id = String::from("LINK1");

        let mut project_data = ProjectTransactionPersistence::build(&mut db);

        let ts = TransactionDeleteLinkProject::new(
            &project_id,
            &link_id
        );

        let res = ts.execute(&mut project_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
}