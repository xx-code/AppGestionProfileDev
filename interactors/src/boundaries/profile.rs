use domains::{repositories::profile_transaction_repository::ProfileTransactionRepository, errors::profile::ErrorProfile};

pub struct RequestCreateProfile<'a> {
    admin_id: &'a String,
    firstname: &'a String,
    lastname: &'a String,
    email_address: &'a String,
    phone_number: &'a String
}

pub struct ResponseGetProfile {
    firstname: String,
    lastname: String,
    email_address: String,
    phone_number: String
}

pub struct RequestUpdateProfile<'a> {
    firstname: Option<&'a String>,
    lastname: Option<&'a String>,
    email_address: Option<&'a String>,
    phone_number: Option<&'a String>,
}

pub type ProfileID = String;

pub trait InteractorProfile {
    fn create_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestCreateProfile) -> Result<ProfileID, ErrorProfile>;
    fn get_profile(&self, repo: &impl ProfileTransactionRepository, profile_id: &String) -> Result<ResponseGetProfile, ErrorProfile>;
    fn update_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestUpdateProfile) -> Result<bool, ErrorProfile>;
    fn delete_profile(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &String) -> Result<bool, ErrorProfile>;
}