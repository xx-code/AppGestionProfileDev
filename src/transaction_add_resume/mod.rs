use time::Date;
use crate::resume_transaction_persistence::ResumeTransactionPersistence;
use crate::resume_transaction_repository::ResumeTransactionRepository;
use crate::transaction::Transaction;
use crate::resume::{
    Resume,
    ResumeType
};
pub struct TransactionAddResumeCurrent<'a> {
    db: &'a mut  ResumeTransactionPersistence<'a>,
    admin_id: &'a String,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
}
impl TransactionAddResumeCurrent<'_> {
    fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, admin_id: &'a String, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date) -> TransactionAddResumeCurrent<'a> {
        TransactionAddResumeCurrent { 
            db,
            admin_id, 
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
        let mut do_let_add = false;
        
        let resume = Resume::new(
            self.resume_id,
            self.title,
            self.description,
            self.type_resume,
            self.date_start,
            None
        );

        self.db.add_resume(resume);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
      transaction_create_admin::TransactionCreateAdmin,
      transaction::Transaction,
      resume::ResumeType,
      transaction_add_resume::TransactionAddResumeCurrent, 
      resume_transaction_persistence::ResumeTransactionPersistence, 
      data_persistence::DataPersistence, resume_transaction_repository::ResumeTransactionRepository,
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

        let admin_id = String::from("admin1");
        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let mut ts = TransactionAddResumeCurrent::new(
            &mut resume_data,
            &admin_id, 
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
    /*#[test]
    fn test_verify_if_is_admin_who_add_resume() {
        
        let admin_id = String::from("admin1");
        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let ts = TransactionAddResumeCurrent::new(
            &admin_id, 
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        ts.execute();

        unsafe {
            let resume = GLOBAL_DB.get_resume(&resume_id);
            assert!(resume.is_none());
        }
    }*/
}