use entities::profile::Profile;

pub trait ProfileTransactionRepository {
    fn create_profile(&mut self, profile: Profile);
    fn update_profile(&mut self, profile: Profile);
    fn is_admin_exist(&self, admin_id: &String) -> bool;
    fn get_profile(&self, profile_id: &String) -> Option<&Profile>;
    fn delete_profile(&mut self, profile_id: &String);
}