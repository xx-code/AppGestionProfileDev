#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        resume_transaction_persistence::ResumeTransactionPersistence
    };
    use time::Date;
    use entities::resume::ResumeType;
    use domains::resume::{
        transaction_add_resume::TransactionAddResumeCurrent,
        transaction_get_resume::{
            TransactionGetResume,
            TransactionGetAllResume
        },
    };

    #[test]
    fn test_get_resume() {
        let mut  db = DataPersistence::new();

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let ts = TransactionAddResumeCurrent::new(
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        let _ = ts.execute(&mut resume_data);
        drop(ts);

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let ts = TransactionGetResume::new(
            &resume_id,
        );
        let res = ts.execute(&mut resume_data);
        let resume = res.ok().unwrap();

        assert_eq!(&title, resume.get_title());
        assert_eq!(&description, resume.get_description());
        assert_eq!(&type_resume, resume.get_type_resume());
        assert_eq!(&date_start, resume.get_date_start());
    }
    #[test]
    fn test_get_no_resume() {
        let mut db = DataPersistence::new();
        
        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume_id = String::from("resume1");

        let ts = TransactionGetResume::new(
            &resume_id,
        );
        let res = ts.execute(&resume_data);

        assert_eq!(res.is_ok(), false);
    }
    #[test]
    fn test_get_all_resune() {
        let mut  db = DataPersistence::new();

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let ts = TransactionAddResumeCurrent::new(
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        let _ = ts.execute(&mut resume_data);
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let ts = TransactionGetAllResume::new(
        );
        let res = ts.execute(&resume_data);
        let resumes = res.ok().unwrap();

        assert_eq!(resumes.len(), 1);
    }
}