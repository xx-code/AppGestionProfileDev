use crate::boundaries::resume::{
    InteractorResume,
    RequestAddResume,
    RequestUpdateResume,
    ResponseGetAllResume,
    ResponseGetResume,
    ResumeID
};
use domains::repositories::resume_transaction_repository::ResumeTransactionRepository;
use domains::errors::resume::ErrorResume;
use domains::resume::transaction_add_resume::{TransactionAddResumeComplet, TransactionAddResumeCurrent};
use domains::resume::transaction_delete_resume::TransactionDeleteResume;
use domains::resume::transaction_get_resume::{TransactionGetAllResume, TransactionGetResume};
use domains::resume::transaction_update_resume::{TransactionUpdateResumeTitle, TransactionUpdateResumeDescription, TransactionUpdateResumeDateStart, TransactionUpdateResumeDateEnd, TransactionUpdateResumeTypeResume};
use entities::resume::ResumeType;
use uuid::Uuid;
pub struct Resume;

impl InteractorResume for Resume { 
    fn add_resume(&self, repo: &mut impl ResumeTransactionRepository, request: &RequestAddResume) -> Result<ResumeID, ErrorResume> {
        let response;

        let resume_type;

        match request.type_resume.as_str() {
            "Eduation" =>  resume_type = ResumeType::Education,
            "Job" => resume_type = ResumeType::Job,
            _ => return Err(ErrorResume::ResumeTypeDoesntMatch)
        };

        let resume_id = Uuid::new_v4().to_string();

        if !request.date_end.is_none() {
            let ts = TransactionAddResumeComplet::new(&resume_id, request.title, request.description, &resume_type, request.date_start, request.date_end.unwrap());
            response = ts.execute(repo);
        } else {
            let ts = TransactionAddResumeCurrent::new(&resume_id, request.title, request.description, &resume_type, request.date_start);
            response = ts.execute(repo);
        }

        match response {
            Ok(_) => Ok(resume_id.to_string()),
            Err(e) => Err(e)
        }

    }
    fn delete_resume(&self, repo: &mut impl ResumeTransactionRepository, resume_id: &ResumeID) -> Result<bool, ErrorResume> {
        let ts = TransactionDeleteResume::new(resume_id);
        let res = ts.execute(repo);

        match res {
           Ok(_) => Ok(true),
           Err(e) => Err(e)
        }
    }
    fn get_all_resume(&self, repo: &impl ResumeTransactionRepository) -> Result<ResponseGetAllResume, ErrorResume> {
        let ts = TransactionGetAllResume::new();
        let res = ts.execute(repo);

        match res {
            Ok(all_resume) => {
                let mut resumes = Vec::new();

                for resume in all_resume {
                    let type_resume = match resume.get_type_resume() {
                        ResumeType::Education => String::from("Education"),
                        ResumeType::Job => String::from("Job")
                    };
                    resumes.push(
                        ResponseGetResume {
                            title: resume.get_title().clone(),
                            type_resume: type_resume,
                            description: resume.get_description().clone(),
                            date_end: resume.get_date_end(),
                            date_start: resume.get_date_start().clone()
                        }
                    );
                }

                Ok(
                    ResponseGetAllResume{
                        resumes
                    }
                )
            },
            Err(e) => Err(e)
        }
    }
    fn get_resume(&self, repo: &impl ResumeTransactionRepository, resume_id: &ResumeID) -> Result<ResponseGetResume, ErrorResume> {
        let ts = TransactionGetResume::new(resume_id);
        let res = ts.execute(repo);

        match res {
            Ok(resume) => {
                let type_resume = match resume.get_type_resume() {
                    ResumeType::Education => String::from("Education"),
                    ResumeType::Job => String::from("Job")
                };

                Ok(
                    ResponseGetResume {
                        title: resume.get_title().clone(),
                        type_resume: type_resume,
                        description: resume.get_description().clone(),
                        date_start: resume.get_date_start().clone(),
                        date_end: resume.get_date_end().clone()
                    }
                )
            },
            Err(e) => Err(e)
        }
        
    }
    fn update_resume(&self, repo: &mut impl ResumeTransactionRepository, request: &RequestUpdateResume) -> Result<bool, ErrorResume> {
        let mut error_response = None;

        if !request.title.is_none() {
            let ts = TransactionUpdateResumeTitle::new(request.resume_id, request.title.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.description.is_none() {
            let ts = TransactionUpdateResumeDescription::new(request.resume_id, request.description.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.date_start.is_none() {
            let ts = TransactionUpdateResumeDateStart::new(request.resume_id, request.date_start.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.date_end.is_none() {
            let ts = TransactionUpdateResumeDateEnd::new(request.resume_id, request.date_end.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.type_resume.is_none() {
            let resume_type;
            match request.type_resume.unwrap().as_str() {
                "Eduation" =>  resume_type = ResumeType::Education,
                "Job" => resume_type = ResumeType::Job,
                _ => return Err(ErrorResume::ResumeTypeDoesntMatch)
            };

            let ts = TransactionUpdateResumeTypeResume::new(request.resume_id, &resume_type);
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if error_response.is_none() {
            Ok(true)
        } else {
            Err(error_response.unwrap())
        }
    }
}