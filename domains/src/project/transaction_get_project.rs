use std::collections::HashMap;
use crate::errors::project::ErrorProject;
use crate::repositories::project_transaction_repository::ProjectTransactionRepository;
use time::Date;

#[derive(Clone, PartialEq, Debug)]
pub struct LinkDto {
    title: String,
    address: String   
}
#[derive(Clone, PartialEq, Debug)]
pub struct ProjectDto {
    pub title: String,
    pub description: String,
    pub date_start: Date,
    pub date_end: Option<Date>,
    pub links: Vec<LinkDto> 
}

pub struct TransactionGetProject<'a> {
    project_id: &'a String
}

impl TransactionGetProject<'_> {
    pub fn new<'a>(project_id: &'a String) -> TransactionGetProject<'a> {
        TransactionGetProject { project_id }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<ProjectDto, ErrorProject> {
        let project = repo.get_project(self.project_id);

        if project.is_none() {
            return Err(ErrorProject::ProjectNotExist)
        }

        let res = project.unwrap();

        let mut links_dto = Vec::new();
        for link in res.get_links() {
            links_dto.push(
                LinkDto {
                    title: link.get_title().clone(),
                    address: link.get_address().clone()
                }
            );
        }

        return Ok(
            ProjectDto {
                title: res.get_title().clone(),
                description: res.get_description().clone(),
                date_start: res.get_date_start().clone(),
                date_end: res.get_date_end().clone(),
                links: links_dto
            }
        )
    }
}

pub struct TransactionGetAllProject {
}

impl TransactionGetAllProject {
    pub fn new() -> TransactionGetAllProject {
        TransactionGetAllProject { }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<Vec<ProjectDto>, ErrorProject> {
        let projects = repo.get_projects();

        let mut project_dto = Vec::new();

        // Refactor
        for project in projects {
            project.get_links();

            let mut links_dto = Vec::new();
            for link in project.get_links() {
                links_dto.push(
                    LinkDto {
                        title: link.get_title().clone(),
                        address: link.get_address().clone()
                    }
                );
            }

            project_dto.push(
                ProjectDto {
                    title: project.get_title().clone(),
                    description: project.get_description().clone(),
                    date_start: project.get_date_start().clone(),
                    date_end: project.get_date_end().clone(),
                    links: links_dto
                }
            )
        }

        return Ok(project_dto);
    }
}

pub struct TransactionGetProjectByPage {
    page: usize,
    content_size: usize
}

impl TransactionGetProjectByPage {
    pub fn new(page: usize, content_size: usize) -> TransactionGetProjectByPage {
        TransactionGetProjectByPage { 
            page, 
            content_size
        }
    }

    pub fn execute(&self, repo: &impl ProjectTransactionRepository) -> Result<Vec<ProjectDto>, ErrorProject> {
        if self.page > repo.get_pages_number(self.content_size) {
            return Err(ErrorProject::PagingNotAllowPageIndexMustBeLessThanPageNumber)
        }
        let projects = repo.get_project_by_pages(self.page, self.content_size);

        return Ok(projects);
    }
}