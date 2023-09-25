use crate::{
    repositories::resume_transaction_repository::ResumeTransactionRepository, 
    transaction::Transaction, 
    errors::resume::ErrorResume
};
use entities::resume::Resume;

pub struct TransactionGetResume<'a> {
    resume_id: &'a String,
}

impl TransactionGetResume<'_> {
    pub fn new<'a>(resume_id: &'a String) -> TransactionGetResume<'a> {
        TransactionGetResume {
            resume_id
        }
    }
}

impl Transaction<Resume, ErrorResume, Box<dyn ResumeTransactionRepository>> for TransactionGetResume<'_> {
    fn execute(&mut self, repo: Box<dyn ResumeTransactionRepository>) -> Result<Resume, ErrorResume> {
        let resume = self.db.get_resume(self.resume_id);

        if resume.is_none() {
            return Err(ErrorResume::ResumeNotExist)
        }

        return Ok(resume.unwrap().clone())
    }
}

pub struct TransactionGetAllResume<'a> {
}

impl TransactionGetAllResume<'_> {
    pub fn new<'a>() -> TransactionGetAllResume<'a> {
        TransactionGetAllResume { }     
    }
}

impl Transaction<Vec<Resume>, ErrorResume, Box<dyn ResumeTransactionRepository>> for TransactionGetAllResume<'_> {
    fn execute(&mut self, repo: Box<dyn ResumeTransactionRepository>) -> Result<Vec<Resume>, ErrorResume> {
        let resumes = repo.get_resumes();
        return Ok(resumes)
    }
}