use crate::boundaries::project::{
    InteractorProject,
    RequestAddLink,
    RequestCreateProject,
    RequestGetProjectsByPaging,
    ResponseGetLink,
    ResponseGetProject,
    ProjectID,
    ResponseGetProjects
};
use domains::project::transaction_create_project::{TransactionCreateCompletProject, TransactionCreateCurrentProject};
use domains::project::transaction_delete_project::TransactionDeleteProject;
use domains::project::transaction_get_project::{TransactionGetProject, TransactionGetAllProject, TransactionGetProjectByPage};
use domains::project::transaction_update_project::{TransactionAddLinkProject, TransactionUpdateProjectDateStart, TransactionUpdateProjectDateEnd, TransactionUpdateProjectTitle, TransactionUpdateProjectDescription, TransactionDeleteLinkProject};
use domains::repositories::project_transaction_repository::ProjectTransactionRepository;
use domains::errors::project::ErrorProject;
use uuid::Uuid;

pub struct Project;

impl InteractorProject for Project {
    fn create_profile(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestCreateProject) -> Result<ProjectID, ErrorProject> {
        let response;

        let project_id = Uuid::new_v4().to_string();

        if request.date_end.is_none() {
            let ts = TransactionCreateCurrentProject::new(&project_id, request.title, request.description, request.date_start);
            response = ts.execute(repo);
        } else {
            let ts = TransactionCreateCompletProject::new(&project_id, request.title, request.description, request.date_start, request.date_end.unwrap());
            response = ts.execute(repo);
        }

        match response {
            Ok(_) => Ok(project_id.to_string()),
            Err(e) => Err(e)
        }
    }
    fn get_project(&self, repo: &impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<ResponseGetProject, ErrorProject> {
        let ts = TransactionGetProject::new(project_id);
        let response = ts.execute(repo);

        match response {
            Ok(project) => {
                let mut links = Vec::new();
                for link in project.get_links() {
                    links.push({
                        ResponseGetLink {
                            title: link.get_title().clone(),
                            address: link.get_address().clone()
                        }
                    });
                }

                Ok(ResponseGetProject {
                    title: project.get_title().clone(),
                    description: project.get_description().clone(),
                    date_start: project.get_date_start().clone(),
                    date_end: project.get_date_end().clone(),
                    links: links
                })
            },
            Err(e) => Err(e)
        }
    }
    fn add_link(&self, repo: &mut impl ProjectTransactionRepository, request: &RequestAddLink) -> Result<ProjectID, ErrorProject> {
        let link_id = Uuid::new_v4().to_string();
        let ts = TransactionAddLinkProject::new(request.project_id, &link_id, request.title, request.address);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(request.project_id.clone()),
            Err(e) => Err(e)
        }

    }
    fn delete_project(&self, repo: &mut impl ProjectTransactionRepository, project_id: &ProjectID) -> Result<bool, ErrorProject> {
        let ts = TransactionDeleteProject::new(project_id);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }
    fn get_all_project(&self, repo: &impl ProjectTransactionRepository) -> Result<ResponseGetProjects, ErrorProject> {
        let ts = TransactionGetAllProject::new();
        let response = ts.execute(repo);

        match response {
            Ok(projects) => {
                let mut res_projects = Vec::new();
                for project in projects {
                    let mut links = Vec::new();
                    for link in project.get_links() {
                        links.push(ResponseGetLink {
                            title: link.get_title().clone(),
                            address: link.get_address().clone()
                        })
                    }
                    res_projects.push(ResponseGetProject {
                        title: project.get_title().clone(),
                        description: project.get_description().clone(),
                        date_start: project.get_date_start().clone(),
                        date_end: project.get_date_end().clone(),
                        links: links
                    });
                }
                Ok(ResponseGetProjects {
                    projects: res_projects
                })
            },
            Err(e) => Err(e)
        }
    }
    
    fn get_project_by_pages(&self,  repo: &impl ProjectTransactionRepository, request: &RequestGetProjectsByPaging) -> Result<ResponseGetProjects, ErrorProject> {
        let ts = TransactionGetProjectByPage::new(request.page, request.content_size);
        let response = ts.execute(repo);

        match response {
            Ok(projects) => {
                let mut res_projects = Vec::new();
                for project in projects {
                    let mut links = Vec::new();
                    for link in project.get_links() {
                        links.push(ResponseGetLink {
                            title: link.get_title().clone(),
                            address: link.get_address().clone()
                        })
                    }
                    res_projects.push(ResponseGetProject {
                        title: project.get_title().clone(),
                        description: project.get_description().clone(),
                        date_start: project.get_date_start().clone(),
                        date_end: project.get_date_end().clone(),
                        links: links
                    });
                }
                Ok(ResponseGetProjects {
                    projects: res_projects
                })
            },
            Err(e) => Err(e)
        }
    }

    fn delete_link(&self, repo: &mut impl ProjectTransactionRepository, project_id: &ProjectID, link_id: &String) -> Result<bool, ErrorProject> {
        let ts = TransactionDeleteLinkProject::new(project_id, link_id);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn update_project(&self, repo: &mut impl ProjectTransactionRepository, request: &crate::boundaries::project::RequestUpdateProject) -> Result<bool, ErrorProject> {
        let mut error_response = None;

        if !request.date_start.is_none() {
            let ts = TransactionUpdateProjectDateStart::new(request.project_id, request.date_start.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.date_end.is_none() {
            let ts = TransactionUpdateProjectDateEnd::new(request.project_id, request.date_end.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.title.is_none() {
            let ts = TransactionUpdateProjectTitle::new(request.project_id, request.title.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.description.is_none() {
            let ts = TransactionUpdateProjectDescription::new(request.project_id, request.title.unwrap());
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