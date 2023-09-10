use crate::{resume_transaction_persistence::ResumeTransactionPersistence, transaction::Transaction, resume_transaction_repository::ResumeTransactionRepository};
pub struct TransactionDeleteResume<'a> {
    db: &'a mut ResumeTransactionPersistence<'a>,
    resume_id: &'a String
}

impl TransactionDeleteResume<'_> {
    pub fn new<'a>(db: &'a mut ResumeTransactionPersistence<'a>, resume_id: &'a String) -> TransactionDeleteResume<'a> {
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

#[cfg(test)]
mod tests {
    use crate::transaction::Transaction;
    use crate::resume::ResumeType;
    use crate::resume_transaction_persistence::ResumeTransactionPersistence;
    use crate::resume_transaction_repository::ResumeTransactionRepository;
    use time::Date;
    use crate::data_persistence::DataPersistence;
    use super::TransactionDeleteResume;

    #[test]
    fn test_delete_resume() {
        let mut db = DataPersistence::new(); 

        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();
        let date_end = Date::from_calendar_date(2020, time::Month::April, 3).unwrap();

        let mut resume_data = ResumeTransactionPersistence::build(&mut db);

        let mut ts = TransactionDeleteResume::new(
            &mut resume_data,
            &resume_id,
        );
        ts.execute();

        let resume_data = ResumeTransactionPersistence::build(&mut db);
        let resume = resume_data.get_resume(&resume_id);
        assert!(resume.is_none());
    }
}