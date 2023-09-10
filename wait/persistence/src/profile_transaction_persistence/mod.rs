use crate::profile_transaction_repository::ProfileTransactionRepository;
use crate::data_persistence::{DataPersistence, Indexing};
use entities::entity::Entity;
use entities::admin::Admin;
use entities::profile::Profile;
pub struct ProfileTransactionPersistence<'a> {
    db: &'a mut  DataPersistence
}

impl ProfileTransactionPersistence<'_> {
    pub fn build<'a>(db: &'a mut  DataPersistence) -> ProfileTransactionPersistence<'a>{
        ProfileTransactionPersistence { db }
    }
}
impl Indexing for ProfileTransactionPersistence<'_>{}
impl ProfileTransactionRepository for ProfileTransactionPersistence<'_> {
    fn create_profile(&mut self, profile: Profile) {
        self.db.profiles.push(profile)
    }
    fn is_admin_exist(&self, admin_id: &String) -> bool {
        let index = self.get_index(&self.db.admins, admin_id);
        if index.is_none() {
            return false;
        }
        return true;
    }
    fn get_profile(&self, profile_id: &String) -> Option<&Profile> {
        let index = self.get_index(&self.db.profiles, profile_id);
        
        if !index.is_none() {
           return Some(&self.db.profiles[index.unwrap()])
        }
        None
    }
    fn update_profile(&mut self, profile: Profile) {
        let index = self.get_index(&self.db.profiles, profile.get_id()).unwrap();
        self.db.profiles[index] = profile;
    }
    fn delete_profile(&mut self, profile_id: &String) {
        let index = self.get_index(&self.db.profiles, profile_id);
        if !index.is_none() {
            self.db.profiles.remove(index.unwrap());
        }
    }
}