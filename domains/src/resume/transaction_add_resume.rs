use std::borrow::BorrowMut;

use time::Date;
use entities::resume::{
    Resume,
    ResumeType
};
use crate::{transaction::Transaction, errors::{ErrorDomain, resume::ErrorResume}};
use crate::repositories::resume_transaction_repository::ResumeTransactionRepository;
pub struct TransactionAddResumeCurrent<'a> {
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
}
impl TransactionAddResumeCurrent<'_> {
    pub fn new<'a>(resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date) -> TransactionAddResumeCurrent<'a> {
        TransactionAddResumeCurrent {
            resume_id, 
            title, 
            description, 
            type_resume, 
            date_start 
        }
    }
}
impl Transaction<(), ErrorResume, Box<dyn ResumeTransactionRepository>> for TransactionAddResumeCurrent<'_> {
    fn execute(&mut self, repo: Box<dyn ResumeTransactionRepository>) -> Result<(), ErrorResume> {
        let repo = repo.borrow_mut();
        let resume = Resume::new(
            self.resume_id,
            self.title,
            self.description,
            self.type_resume,
            self.date_start,
            None
        );

        repo.add_resume(resume);
        Ok(())
    }
}

pub struct TransactionAddResumeComplet<'a> {
    resume_id: &'a String,
    title: &'a String,
    description: &'a String,
    type_resume: &'a ResumeType,
    date_start: &'a Date,
    date_end: &'a Date,
}
impl TransactionAddResumeComplet<'_> {
    pub fn new<'a>(resume_id: &'a String, title: &'a String, description: &'a String, type_resume: &'a ResumeType, date_start: &'a Date, date_end: &'a Date) -> TransactionAddResumeComplet<'a> {
        TransactionAddResumeComplet {
            resume_id, 
            title, 
            description, 
            type_resume, 
            date_start, 
            date_end
        }
    }
}
impl Transaction<(), ErrorResume, Box<dyn ResumeTransactionRepository>> for TransactionAddResumeComplet<'_> {
    fn execute(&mut self, repo: Box<dyn ResumeTransactionRepository>) -> Result<(), ErrorResume> {
        let repo = repo.borrow_mut();
        if self.date_start < self.date_end {
            let resume = Resume::new(
                self.resume_id,
                self.title,
                self.description,
                self.type_resume,
                self.date_start,
                Some(self.date_end),
            );
    
            repo.add_resume(resume);
            Ok(())
        } else {
            Err(ErrorResume::DateEndMustBeSuperiorDateStart)
        }
        
    }
}