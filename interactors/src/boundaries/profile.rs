use domains::{repositories::profile_transaction_repository::ProfileTransactionRepository, errors::{profile::ErrorProfile, admin::ErrorAdmin}};

pub struct RequestCreateProfile<'a> {
    pub admin_id: &'a String,
    pub firstname: &'a String,
    pub lastname: &'a String,
    pub email_address: &'a String,
    pub phone_number: &'a String
}

pub struct ResponseGetProfile {
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    pub phone_number: String
}

pub struct RequestUpdateProfile<'a> {
    pub profile_id: &'a String,
    pub firstname: Option<&'a String>,
    pub lastname: Option<&'a String>,
    pub email_address: Option<&'a String>,
    pub phone_number: Option<&'a String>,
}

pub struct RequestAddLinkProfile<'a> {
    pub profile_id: &'a String,
    pub title: &'a String,
    pub address: &'a String
}

pub type ProfileID = String;

pub trait InteractorProfile {
    fn create_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestCreateProfile) -> Result<ProfileID, ErrorAdmin>;
    fn add_link(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &ProfileID, request: &RequestAddLinkProfile) -> Result<bool, ErrorProfile>;
    fn delete_link(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &ProfileID, link_id: &String) -> Result<bool, ErrorProfile>;
    fn get_profile(&self, repo: &impl ProfileTransactionRepository, profile_id: &String) -> Result<ResponseGetProfile, ErrorProfile>;
    fn update_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestUpdateProfile) -> Result<bool, ErrorProfile>;
    fn delete_profile(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &String) -> Result<bool, ErrorProfile>;
}