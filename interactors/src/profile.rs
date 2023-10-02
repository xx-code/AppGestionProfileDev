use crate::boundaries::profile::{
    InteractorProfile,
    ProfileID,
    RequestCreateProfile,
    RequestUpdateProfile,
    ResponseGetProfile, 
    RequestAddLinkProfile
};
use domains::errors::admin::ErrorAdmin;
use domains::profile::transaction_create_profile::TransactionCreateProfile;
use domains::profile::transaction_delete_profile::TransactionDeleteProfile;
use domains::profile::transaction_get_profile::TransactionGetProfile;
use domains::profile::transaction_update_profile::{TransactionUpdateFirstnameProfile, TransactionUpdateLastnameProfile, TransactionUpdateEmailAddressProfile, TransactionUpdatePhoneNumberProfile, TransactionAddLinkProfile, TransactionDeleteLinkProfile};
use domains::repositories::profile_transaction_repository::ProfileTransactionRepository;
use domains::errors::profile::ErrorProfile;
use uuid::Uuid;

pub struct Profile;

impl InteractorProfile for Profile {
    fn create_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestCreateProfile) -> Result<ProfileID, ErrorAdmin> {
        let profile_id = Uuid::new_v4().to_string();
        let ts = TransactionCreateProfile::new(request.admin_id, &profile_id, request.firstname, request.lastname, request.email_address, request.phone_number);

        let response = ts.execute(repo);

        match response {
            Ok(()) => Ok(profile_id.to_string()),
            Err(e) => Err(e)
        }
    }
    fn get_profile(&self, repo: &impl ProfileTransactionRepository, profile_id: &String) -> Result<ResponseGetProfile, ErrorProfile> {
        let ts = TransactionGetProfile::new(profile_id);
        let response = ts.execute(repo);

        match response {
            Ok(profile) => Ok(ResponseGetProfile { 
                firstname: profile.get_firstname().clone(), 
                lastname: profile.get_lastname().clone(), 
                email_address: profile.get_email_address().clone(),
                phone_number: profile.get_phone_number().clone() }),
            Err(e) => Err(e)
        }
    }
    fn delete_profile(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &String) -> Result<bool, ErrorProfile> {
        let ts = TransactionDeleteProfile::new(profile_id);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn add_link(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &ProfileID, request: &RequestAddLinkProfile) -> Result<bool, ErrorProfile> {
        let link_id = Uuid::new_v4().to_string();
        let ts = TransactionAddLinkProfile::new(profile_id, &link_id, request.title, request.address);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn delete_link(&self, repo: &mut impl ProfileTransactionRepository, profile_id: &ProfileID, link_id: &String) -> Result<bool, ErrorProfile> {
        let ts = TransactionDeleteLinkProfile::new(profile_id, link_id);
        let response = ts.execute(repo);

        match response {
            Ok(_) => Ok(true),
            Err(e) => Err(e)
        }
    }

    fn update_profile(&self, repo: &mut impl ProfileTransactionRepository, request: &RequestUpdateProfile) -> Result<bool, ErrorProfile> {
        let mut error_response = None;

        if !request.firstname.is_none() {
            let ts = TransactionUpdateFirstnameProfile::new(request.profile_id, request.firstname.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.lastname.is_none() {
            let ts = TransactionUpdateLastnameProfile::new(request.profile_id, request.firstname.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.email_address.is_none() {
            let ts = TransactionUpdateEmailAddressProfile::new(request.profile_id, request.email_address.unwrap());
            let respone = ts.execute(repo);
            if respone.is_err() {
                error_response = Some(respone.err().unwrap());
            }
        }

        if !request.phone_number.is_none() {
            let ts = TransactionUpdatePhoneNumberProfile::new(request.profile_id, request.phone_number.unwrap());
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