use crate::{
    repositories::profile_transaction_repository::ProfileTransactionRepository, 
    errors::profile::ErrorProfile
};

#[derive(Debug, Clone, PartialEq)]
pub struct ProfileDto {
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    pub phone_number: String
}

pub struct TransactionGetProfile<'a> {
    profile_id: &'a String
}

impl TransactionGetProfile<'_> {
    pub fn new<'a>(profile_id: &'a String) -> TransactionGetProfile<'a> {
        TransactionGetProfile { profile_id }
    }

    pub fn execute(&self, repo: &impl ProfileTransactionRepository) -> Result<ProfileDto, ErrorProfile> {
        let profile = repo.get_profile(self.profile_id);

        if profile.is_none() {
            return Err(ErrorProfile::ProfileNotExist)
        }

        let response = profile.unwrap();

        Ok(ProfileDto {
            firstname: response.get_firstname().clone(),
            lastname: response.get_lastname().clone(),
            email_address: response.get_email_address().clone(),
            phone_number: response.get_phone_number().clone()
        })
    }
}