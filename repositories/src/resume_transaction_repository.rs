use entities::resume::Resume;

pub trait ResumeTransactionRepository {
    fn add_resume(&mut self, resume: Resume);
    fn get_resume(&self, resume_id: &String) -> Option<&Resume>;
    fn delete_resume(&mut self, resume_id: &String);
    fn update_resume(&mut self, resume: Resume);
}