use time::Date;
use crate::transaction::Transaction;
use crate::DB::GLOBAL_DB;
use crate::resume::{
    Resume,
    ResumeType
};
pub struct TransactionAddResumeCurrent<'a> {
    admin_id: &'a String,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
}
impl TransactionAddResumeCurrent<'_> {
    fn new<'a>(admin_id: &'a String, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date) -> TransactionAddResumeCurrent<'a> {
        TransactionAddResumeCurrent { 
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
    fn execute(&self) -> () {
        let mut do_let_add = false;
        unsafe {
            let admin = GLOBAL_DB.get_admin(self.admin_id);
            do_let_add = !admin.is_none();
        }
        let resume = Resume::new(
            self.resume_id,
            self.title,
            self.description,
            self.type_resume,
            self.date_start,
            None
        );

        unsafe {
            if do_let_add {
                GLOBAL_DB.add_resume(resume);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
      transaction_create_admin::TransactionCreateAdmin,
      transaction::Transaction,
      DB::GLOBAL_DB,
      resume::ResumeType,
      transaction_add_resume::TransactionAddResumeCurrent,
    };
    use time::Date;
    
    fn setup() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        ts.execute();
    }
    #[test]
    fn test_add_resume_current() {
        setup();
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
            let resume = GLOBAL_DB.get_resume(&resume_id).unwrap();
            assert_eq!(resume.get_title(), &title);
            assert_eq!(resume.get_description(), &description);
            assert_eq!(resume.get_type_resume(), &type_resume);
            assert_eq!(resume.get_date_start(), &date_start);
        }
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