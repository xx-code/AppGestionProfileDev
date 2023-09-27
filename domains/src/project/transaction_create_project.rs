use entities::project::Project;
use time::Date;
use crate::errors::project::ErrorProject;
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

    pub fn execute(&self, repo: &mut impl ProjectTransactionRepository) -> Result<(), ErrorProject> {
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

    pub fn execute(&self, repo: &mut impl ProjectTransactionRepository) -> Result<(), ErrorProject> {
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