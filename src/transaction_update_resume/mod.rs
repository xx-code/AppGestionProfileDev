use crate::{
    resume_transaction_persistence::ResumeTransactionPersistence, 
    transaction::Transaction, 
    resume_transaction_repository::ResumeTransactionRepository,
    resume::ResumeType,
};
use time::Date;

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

pub struct TransactionUpdateResumeDescription<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    description: &'a String,
}
impl TransactionUpdateResumeDescription<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, description: &'a String) -> TransactionUpdateResumeDescription<'a> {
        TransactionUpdateResumeDescription { 
            db, 
            resume_id,
            description
        }
    }
}
impl Transaction for TransactionUpdateResumeDescription<'_> {
    fn execute(&mut self) -> () {
        let resume =  self.db.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_description(self.description);
            self.db.update_resume(resume);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateResumeTypeResume<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    type_resume: &'a ResumeType,
}
impl TransactionUpdateResumeTypeResume<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, type_resume: &'a ResumeType) -> TransactionUpdateResumeTypeResume<'a> {
        TransactionUpdateResumeTypeResume { 
            db, 
            resume_id,
            type_resume
        }
    }
}
impl Transaction for TransactionUpdateResumeTypeResume<'_> {
    fn execute(&mut self) -> () {
        let resume =  self.db.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_type_resume(self.type_resume);
            self.db.update_resume(resume);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateResumeDateStart<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    date_start: &'a Date,
}
impl TransactionUpdateResumeDateStart<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, date_start: &'a Date) -> TransactionUpdateResumeDateStart<'a> {
        TransactionUpdateResumeDateStart { 
            db, 
            resume_id,
            date_start
        }
    }
}
impl Transaction for TransactionUpdateResumeDateStart<'_> {
    fn execute(&mut self) -> () {
        let resume =  self.db.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_date_start(self.date_start);
            self.db.update_resume(resume);
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateResumeDateEnd<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String,
    date_end: &'a Date,
}
impl TransactionUpdateResumeDateEnd<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String, date_end: &'a Date) -> TransactionUpdateResumeDateEnd<'a> {
        TransactionUpdateResumeDateEnd { 
            db, 
            resume_id,
            date_end
        }
    }
}
impl Transaction for TransactionUpdateResumeDateEnd<'_> {
    fn execute(&mut self) -> () {
        let resume =  self.db.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_date_end(Some(self.date_end));
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
    use super::{
        TransactionUpdateResumeTitle,
        TransactionUpdateResumeDescription,
        TransactionUpdateResumeTypeResume,
        TransactionUpdateResumeDateStart,
        TransactionUpdateResumeDateEnd,
    };

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

    #[test]
    fn test_update_resume_description() {
        let mut db = DataPersistence::new(); 
        setup(&mut db);

        let resume_id = String::from("resume1");
        let new_description = String::from("description - new");

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);
        let mut ts = TransactionUpdateResumeDescription::new(
            &mut resume_data,
            &resume_id,
            &new_description, 
        );
        ts.execute();

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

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);
        let mut ts = TransactionUpdateResumeTypeResume::new(
            &mut resume_data,
            &resume_id,
            &new_type_resume,
        );
        ts.execute();

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

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);
        let mut ts = TransactionUpdateResumeDateStart::new(
            &mut resume_data,
            &resume_id,
            &new_date_start, 
        );
        ts.execute();

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

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);
        let mut ts = TransactionUpdateResumeDateEnd::new(
            &mut resume_data,
            &resume_id,
            &new_date_end, 
        );
        ts.execute();

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id).unwrap();
        assert_eq!(&resume.get_date_end().unwrap(), &new_date_end);
    }

    fn test_not_accept_date_start_more_than_date_end() {

    }
}