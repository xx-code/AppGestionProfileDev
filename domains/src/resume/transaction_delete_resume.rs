use crate::repositories::resume_transaction_repository::ResumeTransactionRepository;
use crate::errors::resume::ErrorResume;

pub struct TransactionDeleteResume<'a> {
    resume_id: &'a String
}

impl TransactionDeleteResume<'_> {
    pub fn new<'a>(resume_id: &'a String) -> TransactionDeleteResume<'a> {
        TransactionDeleteResume {
            resume_id 
        }
    }

    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> {
        let resume = repo.get_resume(self.resume_id);

        if !resume.is_none() {
            repo.delete_resume(self.resume_id);
            Ok(())
        } else {
            Err(ErrorResume::ResumeNotExist)
        }
    }
}