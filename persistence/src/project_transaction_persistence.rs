use entities::project::Project;
use repositories::project_transaction_repository::ProjectTransactionRepository;
use crate::data_persistence::{DataPersistence, Indexing};

pub struct ProjectTransactionPersistence<'a> {
    db: &'a mut  DataPersistence
}

impl Indexing for ProjectTransactionPersistence<'_>{}
impl ProjectTransactionPersistence<'_> {
    pub fn build<'a>(db: &'a mut DataPersistence) -> ProjectTransactionPersistence<'a> {
        ProjectTransactionPersistence { db }
    }
}

impl ProjectTransactionRepository for ProjectTransactionPersistence <'_>{
    fn create_project(&mut self, project: Project) {
        self.db.projects.push(project);
    }
    fn get_project(&self, project_id: &String) -> Option<&Project> {
        let index = self.get_index(&self.db.projects, project_id);

        if !index.is_none() {
            return Some(&self.db.projects[index.unwrap()])
        }
        None
    }
}