use crate::repositories::resume_transaction_repository::ResumeTransactionRepository;
use crate::{
    transaction::Transaction, 
    errors::{ErrorDomain, resume::ErrorResume}
};

pub struct TransactionDeleteResume<'a> {
    db: Box<dyn ResumeTransactionRepository + 'a>,
    resume_id: &'a String
}

impl TransactionDeleteResume<'_> {
    pub fn new<'a>(db: Box<dyn ResumeTransactionRepository + 'a>, resume_id: &'a String) -> TransactionDeleteResume<'a> {
        TransactionDeleteResume { 
            db, 
            resume_id 
        }
    }
}
impl Transaction for TransactionDeleteResume<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let resume = self.db.get_resume(self.resume_id);

        if !resume.is_none() {
            self.db.delete_resume(self.resume_id);
            Ok(())
        } else {
            Err(Box::new(ErrorResume::ResumeNotExist))
        }
    }
}