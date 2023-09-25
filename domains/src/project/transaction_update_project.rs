use std::borrow::BorrowMut;

use entities::link::Link;
use time::Date;
use crate::{transaction::Transaction, errors::{ErrorDomain, project::ErrorProject}};
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
pub struct TransactionUpdateProjectTitle<'a> {
    project_id: &'a String,
    title: &'a String,
}
impl TransactionUpdateProjectTitle<'_> {
    pub fn new<'a>(project_id: &'a String, title: &'a String,) -> TransactionUpdateProjectTitle<'a> {
        TransactionUpdateProjectTitle { 
            project_id, 
            title 
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionUpdateProjectTitle<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            let mut project = project.unwrap().clone();
            project.set_title(self.title);
            repo.update_project(project);
            Ok(())
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
    }
}

pub struct TransactionUpdateProjectDescription<'a> {
    project_id: &'a String,
    description: &'a String,
}
impl TransactionUpdateProjectDescription<'_> {
    pub fn new<'a>(project_id: &'a String, description: &'a String,) -> TransactionUpdateProjectDescription<'a> {
        TransactionUpdateProjectDescription { 
            project_id, 
            description 
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionUpdateProjectDescription<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            let mut project = project.unwrap().clone();
            project.set_description(self.description);
            repo.update_project(project);
            Ok(())
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
    }
}

pub struct TransactionUpdateProjectDateStart<'a> {
    project_id: &'a String,
    date_start: &'a Date,
}
impl TransactionUpdateProjectDateStart<'_> {
    pub fn new<'a>(project_id: &'a String, date_start: &'a Date,) -> TransactionUpdateProjectDateStart<'a> {
        TransactionUpdateProjectDateStart { 
            project_id, 
            date_start 
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionUpdateProjectDateStart<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            if !project.unwrap().date_end.is_none() {
                if &project.unwrap().date_end.unwrap() > self.date_start {
                    let mut project = project.unwrap().clone();
                    project.set_date_start(self.date_start);
                    repo.update_project(project);
                    Ok(())
                } else {
                    Err(ErrorProject::DateEndMustBeSuperiorDateStart)
                }
            } else {
                let mut project = project.unwrap().clone();
                project.set_date_start(self.date_start);
                repo.update_project(project);
                Ok(())
            }
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
    }
}

pub struct TransactionUpdateProjectDateEnd<'a> {
    project_id: &'a String,
    date_end: &'a Date,
}
impl TransactionUpdateProjectDateEnd<'_> {
    pub fn new<'a>(project_id: &'a String, date_end: &'a Date,) -> TransactionUpdateProjectDateEnd<'a> {
        TransactionUpdateProjectDateEnd { 
            project_id, 
            date_end 
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionUpdateProjectDateEnd<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = repo.get_project(self.project_id);
        
        if !project.is_none() {
            if self.date_end > &project.unwrap().date_start {
                let mut project = project.unwrap().clone();
                project.set_date_end(Some(self.date_end));
                repo.update_project(project);
                Ok(())
            } else {
                Err(ErrorProject::DateEndMustBeSuperiorDateStart)
            }
        } else {
            Err(ErrorProject::ProjectNotExist)
        }
    }
}

pub struct TransactionAddLinkProject<'a> {
    project_id: &'a String,
    link_id: &'a String,
    title: &'a String,
    address: &'a String
}
impl TransactionAddLinkProject<'_> {
    pub fn new<'a>(project_id: &'a String, link_id: &'a String, title: &'a String, address: &'a String) -> TransactionAddLinkProject<'a> {
        TransactionAddLinkProject { 
            project_id,
            link_id,
            title, 
            address 
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionAddLinkProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let link = Link::new(self.link_id, self.title, self.address);
        let project = self.db.get_project(self.project_id);

        if project.is_none() {
            return Err(ErrorProject::ProjectNotExist)
        }

        self.db.create_link_project(self.project_id, link);
        Ok(())
    }
}

pub struct TransactionDeleteLinkProject<'a> {
    project_id: &'a String,
    link_id: &'a String,
}
impl TransactionDeleteLinkProject<'_> {
    pub fn new<'a>(project_id: &'a String, link_id: &'a String) -> TransactionDeleteLinkProject<'a> {
        TransactionDeleteLinkProject {
            project_id,
            link_id,
        }
    }
}
impl Transaction<(), ErrorProject, Box<dyn ProjectTransactionRepository>>  for TransactionDeleteLinkProject<'_> {
    fn execute(&mut self, repo: Box<dyn ProjectTransactionRepository>) -> Result<(), ErrorProject> {
        let repo = repo.borrow_mut();
        let project = self.db.get_project(self.project_id);

        if project.is_none() {
            return Err(ErrorProject::ProjectNotExist)
        }

        let link = repo.get_link_project(self.project_id, self.link_id);

        if link.is_none() {
            return Err(ErrorProject::LinkNotExist)
        }

        repo.delete_link_project(self.project_id, self.link_id);
        Ok(())
    }
}