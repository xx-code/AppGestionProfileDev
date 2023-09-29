use domains::{repositories::resume_transaction_repository::ResumeTransactionRepository, errors::resume::ErrorResume};
use time::Date;
pub struct RequestAddResume<'a> {
    title: &'a String,
    description: &'a String,
    type_resume: &'a String,
    date_start: &'a Date,
    date_end: Option<&'a Date>
}

pub struct RequestUpdateResume<'a> {
    title: Option<&'a String>,
    description: Option<&'a String>,
    type_resume: Option<&'a String>,
    date_start: Option<&'a Date>,
    date_end: Option<&'a Date>
}
pub struct ResponseGetResume {
    title: String,
    description: String,
    type_resume: String,
    date_start: Date,
    date_end: Option<Date>
}

pub struct ResponseGetAllResume {
    resumes: Vec<ResponseGetResume>
}

pub type ResumeID = String;

pub trait InteractorResume {
    fn add_resume(&self, repo: &mut impl ResumeTransactionRepository, request: &RequestAddResume) -> Result<ResumeID, ErrorResume>;
    fn get_resume(&self, repo: &impl ResumeTransactionRepository, resume_id: &ResumeID) -> Result<ResponseGetResume, ErrorResume>;
    fn get_all_resume(&self, repo: &impl ResumeTransactionRepository) -> Result<ResponseGetAllResume, ErrorResume>;
    fn delete_resume(&self, repo: &mut impl ResumeTransactionRepository) -> Result<bool, ErrorResume>;
}