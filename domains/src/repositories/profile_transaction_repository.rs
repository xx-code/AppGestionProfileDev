use entities::profile::Profile;
use entities::link::Link;

pub trait ProfileTransactionRepository {
    fn create_profile(&mut self, profile: Profile);
    fn create_link_profile(&mut self, profile_id: &String, link: Link);
    fn delete_link_profile(&mut self, profile_id: &String, link_id: &String);
    fn update_profile(&mut self, profile: Profile);
    fn is_admin_exist(&self, admin_id: &String) -> bool;
    fn get_link_profile(&mut self, profile_id: &String, link_id: &String)-> Option<&Link>;
    fn get_profile(&self, profile_id: &String) -> Option<&Profile>;
    fn delete_profile(&mut self, profile_id: &String);
}