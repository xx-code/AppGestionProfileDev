use entities::link::Link;
use time::Date;
use crate::{transaction::Transaction, errors::{ErrorDomain, project::ErrorProject}};
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
pub struct TransactionUpdateProjectTitle<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    title: &'a String,
}
impl TransactionUpdateProjectTitle<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, title: &'a String,) -> TransactionUpdateProjectTitle<'a> {
        TransactionUpdateProjectTitle { 
            db, 
            project_id, 
            title 
        }
    }
}
impl Transaction<()>  for TransactionUpdateProjectTitle<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);
        
        if !project.is_none() {
            let mut project = project.unwrap().clone();
            project.set_title(self.title);
            self.db.update_project(project);
            Ok(())
        } else {
            Err(Box::new(ErrorProject::ProjectNotExist))
        }
    }
}

pub struct TransactionUpdateProjectDescription<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    description: &'a String,
}
impl TransactionUpdateProjectDescription<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, description: &'a String,) -> TransactionUpdateProjectDescription<'a> {
        TransactionUpdateProjectDescription { 
            db, 
            project_id, 
            description 
        }
    }
}
impl Transaction<()>  for TransactionUpdateProjectDescription<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);
        
        if !project.is_none() {
            let mut project = project.unwrap().clone();
            project.set_description(self.description);
            self.db.update_project(project);
            Ok(())
        } else {
            Err(Box::new(ErrorProject::ProjectNotExist))
        }
    }
}

pub struct TransactionUpdateProjectDateStart<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    date_start: &'a Date,
}
impl TransactionUpdateProjectDateStart<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, date_start: &'a Date,) -> TransactionUpdateProjectDateStart<'a> {
        TransactionUpdateProjectDateStart { 
            db, 
            project_id, 
            date_start 
        }
    }
}
impl Transaction<()>  for TransactionUpdateProjectDateStart<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);
        
        if !project.is_none() {
            if !project.unwrap().date_end.is_none() {
                if &project.unwrap().date_end.unwrap() > self.date_start {
                    let mut project = project.unwrap().clone();
                    project.set_date_start(self.date_start);
                    self.db.update_project(project);
                    Ok(())
                } else {
                    Err(Box::new(ErrorProject::DateEndMustBeSuperiorDateStart))
                }
            } else {
                let mut project = project.unwrap().clone();
                project.set_date_start(self.date_start);
                self.db.update_project(project);
                Ok(())
            }
        } else {
            Err(Box::new(ErrorProject::ProjectNotExist))
        }
    }
}

pub struct TransactionUpdateProjectDateEnd<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    date_end: &'a Date,
}
impl TransactionUpdateProjectDateEnd<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, date_end: &'a Date,) -> TransactionUpdateProjectDateEnd<'a> {
        TransactionUpdateProjectDateEnd { 
            db, 
            project_id, 
            date_end 
        }
    }
}
impl Transaction<()>  for TransactionUpdateProjectDateEnd<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);
        
        if !project.is_none() {
            if self.date_end > &project.unwrap().date_start {
                let mut project = project.unwrap().clone();
                project.set_date_end(Some(self.date_end));
                self.db.update_project(project);
                Ok(())
            } else {
                Err(Box::new(ErrorProject::DateEndMustBeSuperiorDateStart))
            }
        } else {
            Err(Box::new(ErrorProject::ProjectNotExist))
        }
    }
}

pub struct TransactionAddLinkProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    link_id: &'a String,
    title: &'a String,
    address: &'a String
}
impl TransactionAddLinkProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, link_id: &'a String, title: &'a String, address: &'a String) -> TransactionAddLinkProject<'a> {
        TransactionAddLinkProject { 
            db, 
            project_id,
            link_id,
            title, 
            address 
        }
    }
}
impl Transaction<()>  for TransactionAddLinkProject<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let link = Link::new(self.link_id, self.title, self.address);
        let project = self.db.get_project(self.project_id);

        if project.is_none() {
            return Err(Box::new(ErrorProject::ProjectNotExist))
        }

        self.db.create_link_project(self.project_id, link);
        Ok(())
    }
}

pub struct TransactionDeleteLinkProject<'a> {
    db: Box<dyn ProjectTransactionRepository + 'a>,
    project_id: &'a String,
    link_id: &'a String,
}
impl TransactionDeleteLinkProject<'_> {
    pub fn new<'a>(db: Box<dyn ProjectTransactionRepository + 'a>, project_id: &'a String, link_id: &'a String) -> TransactionDeleteLinkProject<'a> {
        TransactionDeleteLinkProject { 
            db, 
            project_id,
            link_id,
        }
    }
}
impl Transaction<()>  for TransactionDeleteLinkProject<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let project = self.db.get_project(self.project_id);

        if project.is_none() {
            return Err(Box::new(ErrorProject::ProjectNotExist))
        }

        let link = self.db.get_link_project(self.project_id, self.link_id);

        if link.is_none() {
            return Err(Box::new(ErrorProject::LinkNotExist))
        }

        self.db.delete_link_project(self.project_id, self.link_id);
        Ok(())
    }
}