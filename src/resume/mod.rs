use time::Date;
use crate::entity::Entity;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResumeType {
    Education,
    Job
}
#[derive(Debug, Clone)]
pub struct Resume {
    resume_id: String,
    title: String,
    description: String,
    type_resume: ResumeType,
    date_start: Date,
    date_end: Option<Date>
}

impl Entity for Resume {
    fn get_id(&self,) -> &String {
        &self.resume_id
    }
}
impl Resume {
    pub fn new(resume_id: &String, title: &String, description: &String, type_resume: &ResumeType, date_start: &Date, date_end: Option<&Date>) -> Resume {
        let mut date = None;
        if !date_end.is_none() {
            date = Some(date_end.unwrap().clone());
        }
        Resume { 
            resume_id: resume_id.clone(), 
            title: title.clone(),
            description: description.clone(),
            type_resume: type_resume.clone(),
            date_start: date_start.clone(),
            date_end: date 
        }
    }

    pub fn get_title(&self) -> &String {
        return &self.title
    }
    pub fn set_title(&mut self, title: &String) {
        self.title = title.to_owned();
    }

    pub fn get_description(&self) -> &String {
        return &self.description
    }
    pub fn set_description(&mut self, description: &String) {
        self.description = description.to_owned();
    } 

    pub fn get_type_resume(&self) -> &ResumeType {
        return &self.type_resume
    }
    pub fn set_type_resume(&mut self, type_resume: &ResumeType) {
        self.type_resume = type_resume.to_owned();
    }

    pub fn get_date_start(&self) -> &Date {
        return &self.date_start
    }
    pub fn set_date_start(&mut self, date: &Date) {
        self.date_start = date.to_owned();
    } 

    pub fn get_date_end(&self) -> Option<Date> {
        match self.date_end {
            Some(_date) => Some(self.date_end.unwrap()),
            None => None,
        }
    }
    pub fn set_date_end(&mut self, date: Option<&Date>) {
        match date {
            Some(date) => { self.date_end = Some(date.clone()) },
            None => { self.date_end = None }
        }
    } 
}