use entities::project::Project;
pub trait ProjectTransactionRepository {
    fn create_project(&mut self, project: Project);
    fn get_project(&self, project_id: &String) -> Option<&Project>;
}