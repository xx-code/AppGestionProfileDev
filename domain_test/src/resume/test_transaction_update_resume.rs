#[cfg(test)]
pub mod tests {
    use entities::resume::ResumeType;
    use domains::{
        transaction::Transaction,
        resume::transaction_add_resume::TransactionAddResumeComplet,
        resume::transaction_update_resume::{
            TransactionUpdateResumeDateEnd,
            TransactionUpdateResumeDateStart,
            TransactionUpdateResumeDescription,
            TransactionUpdateResumeTitle,
            TransactionUpdateResumeTypeResume
        }
    };
    use persistence::{
        data_persistence::DataPersistence,
        resume_transaction_persistence::ResumeTransactionPersistence,
    };
    use repositories::resume_transaction_repository::ResumeTransactionRepository;
    use time::Date;

    fn setup(db: &mut DataPersistence) {
        let resume_id = String::from("resume1");
        let title: String = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::April, 3).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(db));
        let mut ts = TransactionAddResumeComplet::new(
            resume_data,
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start,
            &date_end
        );
        ts.execute();
    }
    #[test]
    fn test_update_resume_title() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_title = String::from("title - new");

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeTitle::new(
            resume_data,
            &resume_id,
            &new_title, 
        );
        ts.execute();
        drop(ts);
        
        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_title(), &new_title);
    }

    #[test]
    fn test_update_resume_description() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_description = String::from("description - new");

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeDescription::new(
            resume_data,
            &resume_id,
            &new_description, 
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_description(), &new_description);
    }
    #[test]
    fn test_update_resume_type_resume() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_type_resume = ResumeType::Job;

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeTypeResume::new(
            resume_data,
            &resume_id,
            &new_type_resume,
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_type_resume(), &new_type_resume);
    }
    #[test]
    fn test_udpdate_resume_date_start() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_date_start = Date::from_calendar_date(2021, time::Month::April, 1).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeDateStart::new(
            resume_data,
            &resume_id,
            &new_date_start, 
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_date_start(), &new_date_start);
    }
    #[test]
    fn test_update_resume_date_end() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_date_end = Date::from_calendar_date(2022, time::Month::June, 1).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeDateEnd::new(
            resume_data,
            &resume_id,
            &new_date_end, 
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(&resume.get_date_end().unwrap(), &new_date_end);
    }

    #[test]
    fn test_not_accept_date_start_more_than_date_end() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_date_start = Date::from_calendar_date(2023, time::Month::April, 1).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeDateStart::new(
            resume_data,
            &resume_id,
            &new_date_start, 
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_ne!(resume.get_date_start(), &new_date_start);
    }
    #[test]
    fn test_not_accept_date_end_less_than_date_start() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_date_end = Date::from_calendar_date(2019, time::Month::April, 1).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionUpdateResumeDateEnd::new(
            resume_data,
            &resume_id,
            &new_date_end, 
        );
        ts.execute();
        drop(ts);

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_ne!(&resume.get_date_end().unwrap(), &new_date_end);
    }
}