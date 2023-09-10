use entities::project::Project;
use repositories::project_transaction_repository::ProjectTransactionRepository;
use time::Date;
use crate::transaction::Transaction;
pub struct TransactionCreateCurrentProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    title: &'a String,
    description: &'a String,
    date_start: &'a Date
}

impl TransactionCreateCurrentProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, title: &'a String, description: &'a String, date_start: &'a Date) -> TransactionCreateCurrentProject<'a> {
        TransactionCreateCurrentProject { 
            db, 
            project_id,
            title, 
            description, 
            date_start
        }
    }
}

impl Transaction for TransactionCreateCurrentProject<'_> {
    fn execute(&mut self){
        let project = Project::new(
            self.project_id,
            self.title,
            self.description,
            self.date_start,
            None
        );
        self.db.create_project(project);
    }
}

pub struct TransactionCreateCompletProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    title: &'a String,
    description: &'a String,
    date_start: &'a Date,
    date_end: &'a Date
}

impl TransactionCreateCompletProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, title: &'a String, description: &'a String, date_start: &'a Date, date_end: &'a Date) -> TransactionCreateCompletProject<'a> {
        TransactionCreateCompletProject { 
            db, 
            project_id,
            title,
            description,
            date_start,
            date_end
         }
    }
}

impl Transaction for TransactionCreateCompletProject<'_> {
    fn execute(&mut self) -> () {
        let project = Project::new(
            self.project_id,
            self.title,
            self.description,
            self.date_start,
            Some(self.date_end)
        );
        self.db.create_project(project);
    }
}