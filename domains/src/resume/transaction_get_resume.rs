use crate::{
    repositories::resume_transaction_repository::ResumeTransactionRepository, 
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

    pub fn execute(&self, repo: &impl ResumeTransactionRepository) -> Result<Resume, ErrorResume> {
        let resume = repo.get_resume(self.resume_id);

        if resume.is_none() {
            return Err(ErrorResume::ResumeNotExist)
        }

        return Ok(resume.unwrap().clone())
    }
}

pub struct TransactionGetAllResume {
}

impl TransactionGetAllResume {
    pub fn new() -> TransactionGetAllResume {
        TransactionGetAllResume { }     
    }

    pub fn execute(&self, repo: &impl ResumeTransactionRepository) -> Result<Vec<Resume>, ErrorResume> {
        let resumes = repo.get_resumes();
        return Ok(resumes)
    }
}
