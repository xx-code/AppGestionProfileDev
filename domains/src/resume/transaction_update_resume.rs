use time::Date;
use entities::resume::ResumeType;
use crate::errors::resume::ErrorResume;
use crate::repositories::resume_transaction_repository::ResumeTransactionRepository;
pub struct TransactionUpdateResumeTitle<'a> {
    resume_id: &'a String,
    title: &'a String,
}
impl TransactionUpdateResumeTitle<'_> {
    pub fn new<'a>(resume_id: &'a String, title: &'a String) -> TransactionUpdateResumeTitle<'a> {
        TransactionUpdateResumeTitle {
            resume_id,
            title
        }
    }
    
    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> {
        let resume =  repo.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_title(self.title);
            repo.update_resume(resume);
            Ok(())
        } else {
            Err(ErrorResume::ResumeNotExist)
        }
    }
}

pub struct TransactionUpdateResumeDescription<'a> {
    resume_id: &'a String,
    description: &'a String,
}
impl TransactionUpdateResumeDescription<'_> {
    pub fn new<'a>(resume_id: &'a String, description: &'a String) -> TransactionUpdateResumeDescription<'a> {
        TransactionUpdateResumeDescription {
            resume_id,
            description
        }
    }
    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> { 
        let resume =  repo.get_resume(self.resume_id);
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_description(self.description);
            repo.update_resume(resume);
            Ok(())
        } else {
            Err(ErrorResume::ResumeNotExist)
        }
    }
}

pub struct TransactionUpdateResumeTypeResume<'a> {
    resume_id: &'a String,
    type_resume: &'a ResumeType,
}
impl TransactionUpdateResumeTypeResume<'_> {
    pub fn new<'a>(resume_id: &'a String, type_resume: &'a ResumeType) -> TransactionUpdateResumeTypeResume<'a> {
        TransactionUpdateResumeTypeResume {
            resume_id,
            type_resume
        }
    }

    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> { 
        let resume = repo.get_resume(self.resume_id);
        
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            resume.set_type_resume(self.type_resume);
            repo.update_resume(resume);

            Ok(())
        } else {
            Err(ErrorResume::ResumeNotExist)
        }
    }
}

pub struct TransactionUpdateResumeDateStart<'a> {
    resume_id: &'a String,
    date_start: &'a Date,
}
impl TransactionUpdateResumeDateStart<'_> {
    pub fn new<'a>(resume_id: &'a String, date_start: &'a Date) -> TransactionUpdateResumeDateStart<'a> {
        TransactionUpdateResumeDateStart {
            resume_id,
            date_start
        }
    }

    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> { 
        let resume =  repo.get_resume(self.resume_id);
        
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            if !resume.get_date_end().is_none() {
                if self.date_start < &resume.get_date_end().unwrap() {
                    resume.set_date_start(self.date_start);
                    repo.update_resume(resume);
                    return Ok(())
                } else {
                    return Err(ErrorResume::DateEndMustBeSuperiorDateStart)
                }
            } else {
                resume.set_date_start(self.date_start);
                repo.update_resume(resume);
                return Ok(())
            }
        } else {
            return Err(ErrorResume::ResumeNotExist)
        }
    }
}

pub struct TransactionUpdateResumeDateEnd<'a> {
    resume_id: &'a String,
    date_end: &'a Date,
}
impl TransactionUpdateResumeDateEnd<'_> {
    pub fn new<'a>(resume_id: &'a String, date_end: &'a Date) -> TransactionUpdateResumeDateEnd<'a> {
        TransactionUpdateResumeDateEnd {
            resume_id,
            date_end
        }
    }

    pub fn execute(&self, repo: &mut impl ResumeTransactionRepository) -> Result<(), ErrorResume> { 
        let resume =  repo.get_resume(self.resume_id);
        
        if !resume.is_none() {
            let mut resume = resume.unwrap().clone();
            if self.date_end > &resume.get_date_start() {
                resume.set_date_end(Some(self.date_end));
                repo.update_resume(resume);
                return Ok(())
            } else {
                return Err(ErrorResume::DateEndMustBeSuperiorDateStart)
            }
        } else {
            return Err(ErrorResume::ResumeNotExist)
        }
    }
}