use crate::{
    data_persistence::{ DataPersistence, Indexing}, 
    resume_transaction_repository::ResumeTransactionRepository,
    resume::Resume,
    
};

pub struct ResumeTransactionPersistence<'a> {
    db: &'a mut  DataPersistence,
}

impl Indexing for ResumeTransactionPersistence<'_>{}
impl ResumeTransactionPersistence<'_> {
    pub fn build<'a>(db: &'a mut  DataPersistence) -> ResumeTransactionPersistence<'a> {
        ResumeTransactionPersistence { db }
    }
}
impl ResumeTransactionRepository for ResumeTransactionPersistence<'_> {
    fn add_resume(&mut self, resume: Resume) {
        self.db.resumes.push(resume);
    }
    fn get_resume(&self, resume_id: &String) -> Option<&Resume> {
        let index = self.get_index(&self.db.resumes, resume_id);
        
        if !index.is_none() {
           return Some(&self.db.resumes[index.unwrap()])
        }
        None
    }
    fn delete_resume(&mut self, resume_id: &String) {
        let index = self.get_index(&self.db.resumes, resume_id);

        if !index.is_none() {
            self.db.resumes.remove(index.unwrap());
        }
    }
}