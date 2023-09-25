use std::borrow::BorrowMut;

use crate::repositories::resume_transaction_repository::ResumeTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, resume::ErrorResume}
};

pub struct TransactionDeleteResume<'a> {
    resume_id: &'a String
}

impl TransactionDeleteResume<'_> {
    pub fn new<'a>(resume_id: &'a String) -> TransactionDeleteResume<'a> {
        TransactionDeleteResume {
            resume_id 
        }
    }
}
impl Transaction<(), ErrorResume, Box<dyn ResumeTransactionRepository>> for TransactionDeleteResume<'_> {
    fn execute(&mut self, repo: Box<dyn ResumeTransactionRepository>) -> Result<(), ErrorResume> {
        let repo = repo.borrow_mut();
        let resume = repo.get_resume(self.resume_id);

        if !resume.is_none() {
            repo.delete_resume(self.resume_id);
            Ok(())
        } else {
            Err(ErrorResume::ResumeNotExist)
        }
    }
}