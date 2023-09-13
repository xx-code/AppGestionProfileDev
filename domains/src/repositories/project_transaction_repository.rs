use entities::project::Project;
pub trait ProjectTransactionRepository {
    fn create_project(&mut self, project: Project);
    fn get_project(&self, project_id: &String) -> Option<&Project>;
    fn update_project(&mut self, project: Project);
    fn delete_project(&mut self, project_id: &String);
}