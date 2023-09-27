#[cfg(test)]
mod tests {
    use entities::resume::ResumeType;
    use domains::{
        repositories::resume_transaction_repository::ResumeTransactionRepository,
        resume::transaction_add_resume::{
            TransactionAddResumeComplet,
            TransactionAddResumeCurrent,
        }
    };
    use persistence::{
        data_persistence::DataPersistence,
        resume_transaction_persistence::ResumeTransactionPersistence,
    };
    use time::Date;
    
    #[test]
    fn test_add_resume_current() {
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
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_title(), &title);
        assert_eq!(resume.get_description(), &description);
        assert_eq!(resume.get_type_resume(), &type_resume);
        assert_eq!(resume.get_date_start(), &date_start);
    }
    #[test]
    fn test_add_resume_complet() {
        let mut db = DataPersistence::new(); 

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::April, 3).unwrap();

        let mut resume_data =ResumeTransactionPersistence::build(&mut db);

        let ts = TransactionAddResumeComplet::new(
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start,
            &date_end
        );
        let _ = ts.execute(&mut resume_data);
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_title(), &title);
        assert_eq!(resume.get_description(), &description);
        assert_eq!(resume.get_type_resume(), &type_resume);
        assert_eq!(resume.get_date_start(), &date_start);
        assert_eq!(&resume.get_date_end().unwrap(), &date_end);
    }
    #[test]
    fn test_validation_date_resume() {
        let mut db = DataPersistence::new(); 

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2020, time::Month::April, 3).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let ts = TransactionAddResumeComplet::new(
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start,
            &date_end
        );
        let res = ts.execute(&mut resume_data);
        drop(ts);

        assert_eq!(res.is_ok(), false);
    }
}