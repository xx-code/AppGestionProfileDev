use crate::{
    repositories::resume_transaction_repository::ResumeTransactionRepository, 
    transaction::Transaction, 
    errors::resume::ErrorResume
};
use entities::resume::Resume;

pub struct TransactionGetResume<'a> {
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String,
}

impl TransactionGetResume<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String) -> TransactionGetResume<'a> {
        TransactionGetResume { 
            db,
            resume_id
        }
    }
}

impl Transaction<Resume> for TransactionGetResume<'_> {
    fn execute(&mut self) -> Result<Resume, Box<dyn crate::errors::ErrorDomain>> {
        let resume = self.db.get_resume(self.resume_id);

        if resume.is_none() {
            return Err(Box::new(ErrorResume::ResumeNotExist))
        }

        return Ok(resume.unwrap().clone())
    }
}

pub struct TransactionGetAllResume<'a> {
    db: Box<dyn ResumeTransactionRepository + 'a>
}

impl TransactionGetAllResume<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>) -> TransactionGetAllResume {
        TransactionGetAllResume { db }     
    }
}

impl Transaction<Vec<Resume>> for TransactionGetAllResume<'_> {
    fn execute(&mut self) -> Result<Vec<Resume>, Box<dyn crate::errors::ErrorDomain>> {
        let resumes = self.db.get_resumes();

        return Ok(resumes)
    }
}