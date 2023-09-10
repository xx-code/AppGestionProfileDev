use time::Date;
use repositories::resume_transaction_repository::ResumeTransactionRepository;
use entities::resume::ResumeType;
use crate::transaction::Transaction;


pub struct TransactionUpdateResumeTitle<'a> {
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    title: &'a String,
}
impl TransactionUpdateResumeTitle<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, title: &'a String) -> TransactionUpdateResumeTitle<'a> {
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
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    description: &'a String,
}
impl TransactionUpdateResumeDescription<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, description: &'a String) -> TransactionUpdateResumeDescription<'a> {
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
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    type_resume: &'a ResumeType,
}
impl TransactionUpdateResumeTypeResume<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, type_resume: &'a ResumeType) -> TransactionUpdateResumeTypeResume<'a> {
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
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    date_start: &'a Date,
}
impl TransactionUpdateResumeDateStart<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, date_start: &'a Date) -> TransactionUpdateResumeDateStart<'a> {
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
            if !resume.get_date_end().is_none() {
                if self.date_start < &resume.get_date_end().unwrap() {
                    resume.set_date_start(self.date_start);
                    self.db.update_resume(resume)
                } else {
                    println!("add error  ")
                }
            } else {
                resume.set_date_start(self.date_start);
                self.db.update_resume(resume);
            }
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}

pub struct TransactionUpdateResumeDateEnd<'a> {
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    date_end: &'a Date,
}
impl TransactionUpdateResumeDateEnd<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, date_end: &'a Date) -> TransactionUpdateResumeDateEnd<'a> {
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
            if self.date_end > &resume.get_date_start() {
                resume.set_date_end(Some(self.date_end));
                self.db.update_resume(resume);
            } else {
                println!("add error  ")
            }
        } else {
            println!("ADD test gestion error no profile")
        }
    }
}