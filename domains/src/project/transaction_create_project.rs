use std::borrow::BorrowMut;

use entities::project::Project;
use time::Date;
use crate::errors::project::ErrorProject;
use crate::{transaction::Transaction, errors::ErrorDomain};
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
pub struct TransactionCreateCurrentProject<'a> {
    project_id: &'a String,
    title: &'a String,
    description: &'a String,
    date_start: &'a Date
}

impl TransactionCreateCurrentProject<'_> {
    pub fn new<'a>(project_id: &'a String, title: &'a String, description: &'a String, date_start: &'a Date) -> TransactionCreateCurrentProject<'a> {
        TransactionCreateCurrentProject {
            project_id,
            title, 
            description, 
            date_start
        }
    }
}

impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository> > for TransactionCreateCurrentProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject>{
        let repo = repo.borrow_mut();
        let project = Project::new(
            self.project_id,
            self.title,
            self.description,
            self.date_start,
            None
        );
        repo.create_project(project);
        Ok(())
    }
}

pub struct TransactionCreateCompletProject<'a> {
    project_id: &'a String,
    title: &'a String,
    description: &'a String,
    date_start: &'a Date,
    date_end: &'a Date
}

impl TransactionCreateCompletProject<'_> {
    pub fn new<'a>(project_id: &'a String, title: &'a String, description: &'a String, date_start: &'a Date, date_end: &'a Date) -> TransactionCreateCompletProject<'a> {
        TransactionCreateCompletProject { 
            project_id,
            title,
            description,
            date_start,
            date_end
         }
    }
}

impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionCreateCompletProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = Project::new(
            self.project_id,
            self.title,
            self.description,
            self.date_start,
            Some(self.date_end)
        );
        repo.create_project(project);

        Ok(())
    }
}