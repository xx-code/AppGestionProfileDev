use entities::project::Project;
use entities::link::Link;
pub trait ProjectTransactionRepository {
    fn create_project(&mut self, project: Project);
    fn create_link_project(&mut self, project_id: &String, link: Link);
    fn delete_link_project(&mut self, project_id: &String, link_id: &String);
    fn get_pages_number(&self, content_size: usize) -> usize;
    fn get_link_project(&mut self, project_id: &String, link_id: &String)-> Option<&Link>;
    fn get_project(&self, project_id: &String) -> Option<&Project>;
    fn get_project_by_pages(&self, page: usize, content_size: usize) -> Vec<Project>;
    fn get_projects(&self,) -> Vec<Project>;
    fn update_project(&mut self, project: Project);
    fn delete_project(&mut self, project_id: &String);
}