#[cfg(test)]
mod tests {
    use persistence::{
        data_persistence::DataPersistence, 
        resume_transaction_persistence::ResumeTransactionPersistence
    };
    use time::Date;
    use entities::resume::ResumeType;
    use domains::{
        transaction::Transaction,
        resume::{
            transaction_add_resume::TransactionAddResumeCurrent,
            transaction_get_resume::{
                TransactionGetResume,
                TransactionGetAllResume
            },
        }
    };

    #[test]
    fn test_get_resume() {
        let mut  db = DataPersistence::new();

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionAddResumeCurrent::new(
            resume_data,
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        let _ = ts.execute();
        drop(ts);

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionGetResume::new(
            resume_data,
            &resume_id,
        );
        let res = ts.execute();
        let resume = res.ok().unwrap();

        assert_eq!(&title, resume.get_title());
        assert_eq!(&description, resume.get_description());
        assert_eq!(&type_resume, resume.get_type_resume());
        assert_eq!(&date_start, resume.get_date_start());
    }
    #[test]
    fn test_get_no_resume() {
        let mut db = DataPersistence::new();
        
        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let resume_id = String::from("resume1");

        let mut ts = TransactionGetResume::new(
            resume_data,
            &resume_id,
        );
        let res = ts.execute();

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

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));

        let mut ts = TransactionAddResumeCurrent::new(
            resume_data,
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        let _ = ts.execute();
        drop(ts);

        let resume_data = Box::new(ResumeTransactionPersistence::build(&mut db));
        let mut ts = TransactionGetAllResume::new(
            resume_data,
        );
        let res = ts.execute();
        let resumes = res.ok().unwrap();

        assert_eq!(resumes.len(), 1);
    }
}