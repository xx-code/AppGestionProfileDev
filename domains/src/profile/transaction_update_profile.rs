use std::borrow::BorrowMut;

use entities::link::Link;

use crate::repositories::profile_transaction_repository::ProfileTransactionRepository;
use crate::{transaction::Transaction, errors::{ErrorDomain, profile::ErrorProfile}};

pub struct TransactionUpdateFirstnameProfile<'a> {
    profile_id: &'a String,
    firstname: &'a String,
}
impl TransactionUpdateFirstnameProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, firstname: &'a String) -> TransactionUpdateFirstnameProfile<'a> {
        TransactionUpdateFirstnameProfile {
            profile_id, 
            firstname 
        }
    }
}
impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionUpdateFirstnameProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let profile =  repo.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_firstname(self.firstname);
            repo.update_profile(profile);

            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }
}

pub struct TransactionUpdateLastnameProfile<'a> {
    profile_id: &'a String,
    lastname: &'a String,
}
impl TransactionUpdateLastnameProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, lastname: &'a String) -> TransactionUpdateLastnameProfile<'a> {
        TransactionUpdateLastnameProfile {
            profile_id, 
            lastname
        }
    }
}
impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionUpdateLastnameProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let  profile =  repo.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_lastname(self.lastname);
            repo.update_profile(profile);
            
            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }
}

pub struct TransactionUpdateEmailAddressProfile<'a> {
    profile_id: &'a String,
    email_address: &'a String,
}
impl TransactionUpdateEmailAddressProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, email_address: &'a String) -> TransactionUpdateEmailAddressProfile<'a> {
        TransactionUpdateEmailAddressProfile { 
            profile_id, 
            email_address
        }
    }
}
impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionUpdateEmailAddressProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let  profile =  repo.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_email_address(self.email_address);
            repo.update_profile(profile);

            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }
}

pub struct TransactionUpdatePhoneNumberProfile<'a> {
    profile_id: &'a String,
    phone_number: &'a String,
}
impl TransactionUpdatePhoneNumberProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, phone_number: &'a String) -> TransactionUpdatePhoneNumberProfile<'a> {
        TransactionUpdatePhoneNumberProfile {
            profile_id,
            phone_number
        }
    }
}
impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionUpdatePhoneNumberProfile<'_>{
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let  profile =  repo.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_phone_number(self.phone_number);
            repo.update_profile(profile);

            Ok(())
        } else {
            Err(ErrorProfile::ProfileNotExist)
        }
    }   
}

pub struct TransactionAddLinkProfile<'a> {
    profile_id: &'a String,
    link_id: &'a String,
    link_title: &'a String,
    link_address: &'a String
}

impl TransactionAddLinkProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, link_id: &'a String, link_title: &'a String, link_address: &'a String) -> TransactionAddLinkProfile<'a> {
        TransactionAddLinkProfile {
            profile_id,
            link_id,
            link_title,
            link_address
        }
    }

}

impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionAddLinkProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let link = Link::new(self.link_id, self.link_title, self.link_address);
        let profile = repo.get_profile(self.profile_id);
        if profile.is_none() {
            return Err(ErrorProfile::ProfileNotExist)
        }
        repo.create_link_profile(self.profile_id, link);

        Ok(())
    }
}

pub struct TransactionDeleteLinkProfile<'a> {
    profile_id: &'a String,
    link_id: &'a String,
}

impl TransactionDeleteLinkProfile<'_> {
    pub fn new<'a>(profile_id: &'a String, link_id: &'a String) -> TransactionDeleteLinkProfile<'a> {
        TransactionDeleteLinkProfile {
            profile_id,
            link_id
        }
    }
}

impl Transaction<(), ErrorProfile, Box<dyn ProfileTransactionRepository>> for TransactionDeleteLinkProfile<'_> {
    fn execute(&mut self, repo: Box<dyn ProfileTransactionRepository>) -> Result<(), ErrorProfile> {
        let repo = repo.borrow_mut();
        let profile = repo.get_profile(self.profile_id);
        if profile.is_none() {
            return Err(ErrorProfile::ProfileNotExist)
        }
        let link = repo.get_link_profile(self.profile_id, self.link_id);
        if link.is_none() {
            return Err(ErrorProfile::LinkProfileNotExit)
        }
        repo.delete_link_profile(self.profile_id, self.link_id);

        Ok(())
    }
}