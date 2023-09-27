#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        project_transaction_persistence::ProjectTransactionPersistence
    };
    use domains::project::{
        transaction_create_project::TransactionCreateCompletProject,
        transaction_get_project::{
            TransactionGetProject,
            TransactionGetAllProject,
            TransactionGetProjectByPage
        },
    };
    use entities::entity::Entity;
    use time::Date;

    fn setup(db: &mut DataPersistence, project_id: &String) {
        let title = String::from("project");
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
    fn test_get_project() {
        let mut db = DataPersistence::new();
        
        let project_id = String::from("project1");
        let title = String::from("project");
        let description = String::from("description project");
        let date_start = Date::from_calendar_date(2021, time::Month::January, 3).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::January, 4).unwrap();

        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionCreateCompletProject::new(
            &project_id,
            &title,
            &description,
            &date_start,
            &date_end
        );
        let _ = ts.execute(&mut project_data);
        drop(ts);
        
        let  project_data =ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProject::new(
            &project_id
        );
        let res = ts.execute(&project_data);
        let project = res.ok().unwrap();

        assert_eq!(project.get_title(), &title);
        assert_eq!(project.get_description(), &description);
        assert_eq!(project.get_date_start(), &date_start);
        assert_eq!(&project.get_date_end().unwrap(), &date_end);
    }
    #[test]
    fn test_get_no_project() {
        let mut db = DataPersistence::new();
        let project_id = String::from("project1");

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProject::new(
            &project_id
        );
        let res = ts.execute(&project_data);

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_get_all_project() {
        let mut db = DataPersistence::new();

        let project_id1 = String::from("project1");
        setup(&mut db, &project_id1);
        let project_id2 = String::from("project2");
        setup(&mut db, &project_id2);
        let project_id3 = String::from("project3");
        setup(&mut db, &project_id3);

        let project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetAllProject::new(

        );
        let res = ts.execute(&project_data);
        let projects = res.ok().unwrap();

        assert_eq!(projects.len(), 3);
    }

    #[test]
    fn test_get_project_by_paging() { 
        let mut db = DataPersistence::new();
        for i in 0..10 {
            let project_id = format!("project{}", i);
            setup(&mut db, &project_id);
        }

        let page = 1;
        let content_size = 5;

        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProjectByPage::new(
            page,
            content_size
        );
        let res = ts.execute(&mut project_data);
        let projects = res.ok().unwrap();

        assert_eq!(projects.len(), 5);
        let project = &projects[0];
        let real_id = String::from("project0");
        assert_eq!(project.get_id(), &real_id);
        drop(ts);

        let page = 2;
        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProjectByPage::new(
            page,
            content_size
        );
        let res = ts.execute(&mut project_data);
        let projects = res.ok().unwrap();

        let project = &projects[0];
        let real_id = String::from("project5");
        assert_eq!(project.get_id(), &real_id);
        drop(ts);

        let content_size = 7;
        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProjectByPage::new(
            page,
            content_size
        );
        let res = ts.execute(&mut project_data);
        let projects = res.ok().unwrap();

        assert_eq!(projects.len(), 3);
        let project = &projects[0];
        let real_id = String::from("project7");
        assert_eq!(project.get_id(), &real_id);
        drop(ts);

        let page = 3;
        let content_size = 3;
        
        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProjectByPage::new(
            page,
            content_size
        );
        let res = ts.execute(&mut project_data);
        let projects = res.ok().unwrap();

        let project = &projects[0];
        let real_id = String::from("project6");
        assert_eq!(project.get_id(), &real_id);
    }
    #[test]
    fn test_not_allow_bad_paging() {
        let mut db = DataPersistence::new();
        let project_id = format!("project1");
        setup(&mut db, &project_id);
        let page = 13;
        let content_size = 5;

        let mut project_data = ProjectTransactionPersistence::build(&mut db);
        let ts = TransactionGetProjectByPage::new(
            page,
            content_size
        );
        let res = ts.execute(&mut project_data);
        assert_eq!(res.is_ok(), false);
    }
}