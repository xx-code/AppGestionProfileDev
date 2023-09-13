use crate::repositories::profile_transaction_repository::ProfileTransactionRepository;
use crate::{transaction::Transaction, errors::{ErrorDomain, profile::ErrorProfile}};

pub struct TransactionUpdateFirstnameProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    firstname: &'a String,
}
impl TransactionUpdateFirstnameProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, firstname: &'a String) -> TransactionUpdateFirstnameProfile<'a> {
        TransactionUpdateFirstnameProfile {
            db,
            profile_id, 
            firstname 
        }
    }
}
impl Transaction for TransactionUpdateFirstnameProfile<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_firstname(self.firstname);
            self.db.update_profile(profile);
            Ok(())
        } else {
            Err(Box::new(ErrorProfile::ProfileNotExist))
        }
    }
}

pub struct TransactionUpdateLastnameProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    lastname: &'a String,
}
impl TransactionUpdateLastnameProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, lastname: &'a String) -> TransactionUpdateLastnameProfile<'a> {
        TransactionUpdateLastnameProfile {
            db,
            profile_id, 
            lastname
        }
    }
}
impl Transaction for TransactionUpdateLastnameProfile<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_lastname(self.lastname);
            self.db.update_profile(profile);
            Ok(())
        } else {
            Err(Box::new(ErrorProfile::ProfileNotExist))
        }
    }
}

pub struct TransactionUpdateEmailAddressProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    email_address: &'a String,
}
impl TransactionUpdateEmailAddressProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, email_address: &'a String) -> TransactionUpdateEmailAddressProfile<'a> {
        TransactionUpdateEmailAddressProfile { 
            db,
            profile_id, 
            email_address
        }
    }
}
impl Transaction for TransactionUpdateEmailAddressProfile<'_> {
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_email_address(self.email_address);
            self.db.update_profile(profile);
            Ok(())
        } else {
            Err(Box::new(ErrorProfile::ProfileNotExist))
        }
    }
}

pub struct TransactionUpdatePhoneNumberProfile<'a> {
    db: Box<dyn ProfileTransactionRepository + 'a>,
    profile_id: &'a String,
    phone_number: &'a String,
}
impl TransactionUpdatePhoneNumberProfile<'_> {
    pub fn new<'a>(db: Box<dyn ProfileTransactionRepository + 'a>, profile_id: &'a String, phone_number: &'a String) -> TransactionUpdatePhoneNumberProfile<'a> {
        TransactionUpdatePhoneNumberProfile {
            db,
            profile_id,
            phone_number
        }
    }
}
impl Transaction for TransactionUpdatePhoneNumberProfile<'_>{
    fn execute(&mut self) -> Result<(), Box<dyn ErrorDomain>> {
        let  profile =  self.db.get_profile(self.profile_id);
        if !profile.is_none() {
            let mut profile = profile.unwrap().clone();
            profile.set_phone_number(self.phone_number);
            self.db.update_profile(profile);
            Ok(())
        } else {
            Err(Box::new(ErrorProfile::ProfileNotExist))
        }
    }   
}