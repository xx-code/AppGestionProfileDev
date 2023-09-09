use time::Date;
use crate::admin_transaction_persistence::AdminTransactionPersistence;
use crate::admin_transaction_repository::AdminTransactionRepository;
use crate::resume_transaction_persistence::ResumeTransactionPersistence;
use crate::resume_transaction_repository::ResumeTransactionRepository;
use crate::transaction::Transaction;
use crate::resume::{
    Resume,
    ResumeType
};
pub struct TransactionAddResumeCurrent<'a> {
    db_resume: &'a mut  ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
}
impl TransactionAddResumeCurrent<'_> {
    fn new<'a>(
        db_resume: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date) -> TransactionAddResumeCurrent<'a> {
        TransactionAddResumeCurrent { 
            db_resume,
            resume_id, 
            title, 
            description, 
            type_resume, 
            date_start 
        }
    }
}
impl Transaction for TransactionAddResumeCurrent<'_> {
    fn execute(&mut self) -> () {
        let resume = Resume::new(
            self.resume_id,
            self.title,
            self.description,
            self.type_resume,
            self.date_start,
            None
        );

        self.db_resume.add_resume(resume);
    }
}

pub struct TransactionAddResumeComplet<'a> {
    db_resume: &'a mut  ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
    date_end: &'a Date,
}
impl TransactionAddResumeComplet<'_> {
    fn new<'a>(
        db_resume: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date, date_end: &'a Date) -> TransactionAddResumeComplet<'a> {
        TransactionAddResumeComplet { 
            db_resume,
            resume_id, 
            title, 
            description, 
            type_resume, 
            date_start, 
            date_end
        }
    }
}
impl Transaction for TransactionAddResumeComplet<'_> {
    fn execute(&mut self) -> () {

        if self.date_start < self.date_end {
            let resume = Resume::new(
                self.resume_id,
                self.title,
                self.description,
                self.type_resume,
                self.date_start,
                Some(self.date_end),
            );
    
            self.db_resume.add_resume(resume);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
      transaction::Transaction,
      resume::ResumeType,
      transaction_add_resume::TransactionAddResumeCurrent, 
      resume_transaction_persistence::ResumeTransactionPersistence, 
      data_persistence::DataPersistence, 
      resume_transaction_repository::ResumeTransactionRepository,
      transaction_add_resume::TransactionAddResumeComplet
    };
    use time::Date;
    
    /*fn setup() -> {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let db = DataPersistence::new();
        let resume_data = ProfileTransactionPersistence::build(&db);

        let ts = TransactionCreateAdmin::new(
            resume_data
            &admin_id,
            &username,
            &password,
        );
        ts.execute();
    }*/
    #[test]
    fn test_add_resume_current() {
        let mut  db = DataPersistence::new();

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let mut ts = TransactionAddResumeCurrent::new(
            &mut resume_data,
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        ts.execute();
        
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

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

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

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id);
        assert!(resume.is_none());
    }
}