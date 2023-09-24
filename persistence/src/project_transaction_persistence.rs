use entities::{entity::Entity, project::Project, link::Link};
use domains::repositories::project_transaction_repository::ProjectTransactionRepository;
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

    fn get_projects(&self,) -> Vec<Project> {
        return self.db.projects.clone()
    }

    fn get_pages_number(&self, content_size: usize) -> usize {
        let page_numbers =  self.db.projects.len() as f32 / content_size as f32;

        page_numbers.ceil() as usize
    }

    fn get_project_by_pages(&self, page: usize, content_size: usize) -> Vec<Project> {
        let start_idx = (page - 1) * content_size;
        let mut end_idx = page * content_size;

        if end_idx > self.db.projects.len() {
            end_idx = self.db.projects.len();
        }

        let projects = self.db.projects[start_idx..end_idx].to_vec();
        return projects
    }

    fn update_project(&mut self, project: Project) {
        let index = self.get_index(&self.db.projects, project.get_id()).unwrap();
        self.db.projects[index] = project;
    }

    fn delete_project(&mut self, project_id: &String) {
        let index = self.get_index(&self.db.projects, project_id).unwrap();
        self.db.projects.remove(index);
    }

    fn get_link_project(&mut self, project_id: &String, link_id: &String) -> Option<&Link>{
        let project = self.get_project(project_id).unwrap();
        project.get_link(link_id)
    }

    fn create_link_project(&mut self, project_id: &String, link:Link) {
        let index = self.get_index(&self.db.projects,project_id).unwrap();
        self.db.projects[index].add_link(link);
    }

    fn delete_link_project(&mut self, project_id: &String, link_id: &String) {
        let index = self.get_index(&self.db.projects, project_id).unwrap();
        self.db.projects[index].remove_link(link_id);
    }
}