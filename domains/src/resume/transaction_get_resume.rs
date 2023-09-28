use crate::{
    repositories::resume_transaction_repository::ResumeTransactionRepository, 
    errors::resume::ErrorResume
};
use entities::resume::ResumeType;
use time::Date;


pub struct ResumeDto {
    pub title: String,
    pub description: String,
    pub type_resume: String,
    pub date_start: Date,
    pub date_end: Option<Date>
}

pub struct TransactionGetResume<'a> {
    resume_id: &'a String,
}

impl TransactionGetResume<'_> {
    pub fn new<'a>(resume_id: &'a String) -> TransactionGetResume<'a> {
        TransactionGetResume {
            resume_id
        }
    }

    pub fn execute(&self, repo: &impl ResumeTransactionRepository) -> Result<ResumeDto, ErrorResume> {
        let resume = repo.get_resume(self.resume_id);

        if resume.is_none() {
            return Err(ErrorResume::ResumeNotExist)
        }

        let res = resume.unwrap();

        let resume_type = match res.get_type_resume() {
            ResumeType::Education => String::from("Education"),
            ResumeType::Job => String::from("Job")
        };

        return Ok(ResumeDto { 
            title: res.get_title().clone(), 
            description: res.get_description().clone(), 
            type_resume: resume_type, 
            date_start: res.get_date_start().clone(), 
            date_end: res.get_date_end().clone() 
        })
    }
}

pub struct TransactionGetAllResume {
}

impl TransactionGetAllResume {
    pub fn new() -> TransactionGetAllResume {
        TransactionGetAllResume { }     
    }

    pub fn execute(&self, repo: &impl ResumeTransactionRepository) -> Result<Vec<ResumeDto>, ErrorResume> {
        let resumes = repo.get_resumes();

        let mut resumes_dto = Vec::new();

        for resume in resumes {
            let resume_type = match resume.get_type_resume() {
                ResumeType::Education => format!("Education"),
                ResumeType::Job => format!("Job")
            };
            resumes_dto.push(ResumeDto {
                title: resume.get_title().clone(),
                description: resume.get_description().clone(),
                date_start: resume.get_date_start().clone(),
                date_end: resume.get_date_end().clone(),
                type_resume: resume_type
            });
        }

        return Ok(resumes_dto)
    }
}
