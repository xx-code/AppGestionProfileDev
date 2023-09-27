use crate::{errors::ErrorDomain, repositories::{admin_transaction_repository::AdminTransactionRepository, profile_transaction_repository::ProfileTransactionRepository, resume_transaction_repository::ResumeTransactionRepository, skill_transaction_repository::SkillTransactionRepository}};
pub trait Transaction<T, E:ErrorDomain, R>
/*where 
        R: 
        AdminTransactionRepository + 
        ProfileTransactionRepository + 
        ResumeTransactionRepository + 
        SkillTransactionRepository*/
{
    fn execute(&mut self, repository: R) -> Result<T, E>;
    
}