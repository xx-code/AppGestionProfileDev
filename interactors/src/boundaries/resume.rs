use domains::{repositories::resume_transaction_repository::ResumeTransactionRepository, errors::resume::ErrorResume};
use time::Date;
pub struct RequestAddResume<'a> {
    pub title: &'a String,
    pub description: &'a String,
    pub type_resume: &'a String,
    pub date_start: &'a Date,
    pub date_end: Option<&'a Date>
}

pub struct RequestUpdateResume<'a> {
    pub resume_id: &'a String,
    pub title: Option<&'a String>,
    pub description: Option<&'a String>,
    pub type_resume: Option<&'a String>,
    pub date_start: Option<&'a Date>,
    pub date_end: Option<&'a Date>
}
pub struct ResponseGetResume {
    pub title: String,
    pub description: String,
    pub type_resume: String,
    pub date_start: Date,
    pub date_end: Option<Date>
}

pub struct ResponseGetAllResume {
    pub resumes: Vec<ResponseGetResume>
}

pub type ResumeID = String;

pub trait InteractorResume {
    fn add_resume(&self, repo: &mut impl ResumeTransactionRepository, request: &RequestAddResume) -> Result<ResumeID, ErrorResume>;
    fn get_resume(&self, repo: &impl ResumeTransactionRepository, resume_id: &ResumeID) -> Result<ResponseGetResume, ErrorResume>;
    fn get_all_resume(&self, repo: &impl ResumeTransactionRepository) -> Result<ResponseGetAllResume, ErrorResume>;
    fn delete_resume(&self, repo: &mut impl ResumeTransactionRepository, resume_id: &ResumeID) -> Result<bool, ErrorResume>;
    fn update_resume(&self, repo: &mut impl ResumeTransactionRepository, request: &RequestUpdateResume) -> Result<bool, ErrorResume>;
}