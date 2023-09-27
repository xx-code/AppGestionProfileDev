#[cfg(test)]
mod tests {
    use entities::resume::ResumeType;
    use domains::{
        transaction::Transaction,
        repositories::resume_transaction_repository::ResumeTransactionRepository,
        resume::{
            transaction_delete_resume::TransactionDeleteResume,
            transaction_add_resume::TransactionAddResumeComplet,
        }
    };
    use persistence::{
        data_persistence::DataPersistence,
        resume_transaction_persistence::ResumeTransactionPersistence,
    };
    use time::Date;

    #[test]
    fn test_delete_resume() {
        let mut db = DataPersistence::new(); 

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2020, time::Month::April, 3).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionAddResumeComplet::new(
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start,
            &date_end
        );
        let _ = ts.execute(resume_data);
        drop(ts);

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionDeleteResume::new(
            &resume_id,
        );
        let _ = ts.execute(resume_data);
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id);
        assert!(resume.is_none());
    }

    #[test]
    fn test_not_delete_resume_if_no_exist() {
        let mut db = DataPersistence::new(); 

        let resume_id = String::from("resume1");
        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionDeleteResume::new(
            &resume_id,
        );
        let res = ts.execute(resume_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }    
}