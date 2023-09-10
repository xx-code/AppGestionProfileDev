use time::Date;
use crate::entity::Entity;
#[derive(Debug, Clone)]
pub struct Project {
    pub project_id: String,
    pub title: String,
    pub description: String,
    pub date_start: Date,
    pub date_end: Option<Date>
}

impl Entity for Project{
    fn get_id(&self,) -> &String {
        &self.project_id
    }
}
impl Project {
    pub fn new(project_id: &String, title: &String, description: &String, date_start: &Date, date_end: Option<&Date>) -> Project {
        let mut date = None;
        if !date_end.is_none() {
            date = Some(date_end.unwrap().clone());
        }
        Project { 
            project_id: project_id.clone(), 
            title: title.clone(), 
            description: description.clone(), 
            date_start: date_start.clone(), 
            date_end: date
        }
    }

    pub fn set_title(&mut self, title: &String) {
        self.title = title.to_owned()
    }
    pub fn set_description(&mut self, description: &String) {
        self.description = description.to_owned()
    }
    pub fn set_date_start(&mut self, date_start: &Date) {
        self.date_start = date_start.to_owned()
    }
    pub fn set_date_end(&mut self, date_end: Option<&Date>) {
        match date_end {
            Some(date) => { self.date_end = Some(date.clone()) },
            None => { self.date_end = None }
        }
    }
    pub fn get_title(&self,) -> &String {
        &self.title
    }
    pub fn get_description(&self,) -> &String {
        &self.description
    }
    pub fn get_date_start(&self,) -> &Date {
        &self.date_start
    }
    pub fn get_date_end(&self,) -> Option<Date> {
        match self.date_end {
            Some(_date) => Some(self.date_end.unwrap()),
            None => None,
        }
    }
}