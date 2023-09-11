use time::Date;
use repositories::resume_transaction_repository::ResumeTransactionRepository;
use entities::resume::{
    Resume,
    ResumeType
};
use crate::{transaction::Transaction, errors::{ErrorDomain, resume::ErrorResume}};

pub struct TransactionAddResumeCurrent<'a> {
    db_resume: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
}
impl TransactionAddResumeCurrent<'_> {
    pub fn new<'a>(
        db_resume: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date) -> TransactionAddResumeCurrent<'a> {
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
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let resume = Resume::new(
            self.resume_id,
            self.title,
            self.description,
            self.type_resume,
            self.date_start,
            None
        );

        self.db_resume.add_resume(resume);
        Ok(())
    }
}

pub struct TransactionAddResumeComplet<'a> {
    db_resume: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
    date_end: &'a Date,
}
impl TransactionAddResumeComplet<'_> {
    pub fn new<'a>(
        db_resume: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date, date_end: &'a Date) -> TransactionAddResumeComplet<'a> {
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
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
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
            Ok(())
        } else {
            Err(Box::new(ErrorResume::DateEndMustBeSuperiorDateStart))
        }
        
    }
}