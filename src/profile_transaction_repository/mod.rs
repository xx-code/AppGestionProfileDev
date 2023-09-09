use crate::profile::Profile;

pub trait ProfileTransactionRepository {
    fn create_profile(&mut self, profile: Profile);
    fn update_profile(&mut self, profile: Profile);
    fn get_profile(&self, profile_id: &String) -> Option<&Profile>;
    fn delete_profile(&mut self, profile_id: &String);
}