#[cfg(test)]
pub mod tests {
    use crate::{
        data_persistence::DataPersistence,
        resume::ResumeType,
        resume_transaction_persistence::ResumeTransactionPersistence,
        transaction_add_resume::TransactionAddResumeComplet,
        transaction::Transaction,
        resume_transaction_repository::ResumeTransactionRepository,
    };
    use time::Date;

    fn setup(db: &mut DataPersistence) {
        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2020, time::Month::April, 3).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(db);
        let mut ts = TransactionAddResumeComplet::new(
            &mut resume_data,
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start,
            &date_end
        );
        ts.execute();
    }
    fn test_update_resume_title() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_title = String::from("title - new");

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);
        let mut ts = TransactionUpdateResumeTitle::new(
            &mut resume_data,
            &resume_id,
            &new_title, 
        );
        ts.execute();

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id);
        assert_eq!(resume.get_title(), &new_title);
    }
}