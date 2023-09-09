use crate::{resume_transaction_persistence::ResumeTransactionPersistence, transaction::Transaction, resume_transaction_repository::ResumeTransactionRepository, resume};

pub struct TransactionUpdateResumeTitle<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    title: &'a String,
}

impl TransactionUpdateResumeTitle<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, title: &'a String) -> TransactionUpdateResumeTitle<'a> {
        TransactionUpdateResumeTitle { 
            db, 
            resume_id,
            title
        }
    }
}
impl Transaction for TransactionUpdateResumeTitle<'_> {
    fn execute(&mut self) -> () {
        let resume =  self.db.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_title(self.title);
            self.db.update_resume(resume);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

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
    use super::TransactionUpdateResumeTitle;

    fn setup(db: &mut DataPersistence) {
        let resume_id = String::from("resume1");
        let title: String = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2022, time::Month::April, 3).unwrap();

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
    #[test]
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
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(resume.get_title(), &new_title);
    }
}