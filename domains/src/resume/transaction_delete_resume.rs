use repositories::resume_transaction_repository::ResumeTransactionRepository;
use crate::transaction::Transaction;

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
    fn execute(&mut self) -> () {
        self.db.delete_resume(self.resume_id);
    }
}