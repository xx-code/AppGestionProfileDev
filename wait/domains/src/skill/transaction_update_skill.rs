use crate::{
    skill_transaction_repository::SkillTransactionRepository,
    transaction::Transaction,
 }; 
 
 struct TransactionUpdateTitleSkill<'a> {
     db: Box<dyn SkillTransactionRepository + 'a>,
     skill_id: &'a String,
     title: &'a String
 }
 impl TransactionUpdateTitleSkill<'_> {
     fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, title: &'a String) -> TransactionUpdateTitleSkill<'a> {
         TransactionUpdateTitleSkill { 
             db, 
             skill_id, 
             title
         }
     }
 }
 impl Transaction for TransactionUpdateTitleSkill<'_> {
     fn execute(&mut self) -> () {
         let skill =  self.db.get_skill(self.skill_id);
         if !skill.is_none() {
             let mut skill = skill.unwrap().clone();
             skill.set_title(self.title);
             self.db.update_skill(skill);
         } else {
             println!("ADD test gestion error no skill")
         }
     }
 }
 
 struct TransactionUpdateIsCurrentSkill<'a> {
     db: Box<dyn SkillTransactionRepository + 'a>,
     skill_id: &'a String,
     is_current: bool
 }
 impl TransactionUpdateIsCurrentSkill<'_> {
     fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, is_current: bool) -> TransactionUpdateIsCurrentSkill<'a> {
         TransactionUpdateIsCurrentSkill { 
             db, 
             skill_id, 
             is_current
         }
     }
 }
 impl Transaction for TransactionUpdateIsCurrentSkill<'_> {
     fn execute(&mut self) -> () {
         let skill =  self.db.get_skill(self.skill_id);
         if !skill.is_none() {
             let mut skill = skill.unwrap().clone();
             skill.set_is_current(self.is_current);
             self.db.update_skill(skill);
         } else {
             println!("ADD test gestion error no skill")
         }
     }
 }
 
 struct TransactionUpdateLogoSkill<'a> {
     db: Box<dyn SkillTransactionRepository + 'a>,
     skill_id: &'a String,
     logo: &'a String
 }
 impl TransactionUpdateLogoSkill<'_> {
     fn new<'a>(db: Box<dyn SkillTransactionRepository + 'a>, skill_id: &'a String, logo: &'a String) -> TransactionUpdateLogoSkill<'a> {
         TransactionUpdateLogoSkill { 
             db, 
             skill_id, 
             logo
         }
     }
 }
 impl Transaction for TransactionUpdateLogoSkill<'_> {
     fn execute(&mut self) -> () {
         let skill =  self.db.get_skill(self.skill_id);
         if !skill.is_none() {
             let mut skill = skill.unwrap().clone();
             skill.set_logo(self.logo);
             self.db.update_skill(skill);
         } else {
             println!("ADD test gestion error no skill")
         }
     }
 }
 
 
 #[cfg(test)]
 mod tests {
     use crate::{
         skill_transaction_persistence::SkillTransactionPersistence, 
         data_persistence::DataPersistence, 
         transaction_add_skill::TransactionAddSkill,
         transaction::Transaction,
         skill_transaction_repository::SkillTransactionRepository,
     };
     use super::*;
 
     fn setup(db: &mut DataPersistence) {
         let skill_id = String::from("Skill1");
 
         let title = String::from("title skill");
         let is_current = false;
         let logo = String::from("logo");
 
         let skill_data = Box::new(SkillTransactionPersistence::build(db));
         let mut ts = TransactionAddSkill::new(
             skill_data,
             &skill_id,
             &title,
             is_current,
             &logo
         );
         ts.execute();
     }
 
     #[test]
     fn test_update_title_skill() {
         let mut db = DataPersistence::new();
         
         setup(&mut db);
 
         let skill_id = String::from("Skill1");
 
         let new_title = String::from("new title");
 
         let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
         let mut ts = TransactionUpdateTitleSkill::new(
             skill_data,
             &skill_id, 
             &new_title,
         );
         ts.execute();
         drop(ts);
 
         let skill_data = SkillTransactionPersistence::build(&mut db);
         let skill = skill_data.get_skill(&skill_id).unwrap();
         assert_eq!(skill.get_title(), &new_title);
     }
 
     #[test]
     fn test_update_logo_skill() {
         let mut db = DataPersistence::new();
         
         setup(&mut db);
 
         let skill_id = String::from("Skill1");
 
         let new_logo = String::from("new logo");
 
         let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
         let mut ts = TransactionUpdateLogoSkill::new(
             skill_data,
             &skill_id, 
             &new_logo,
         );
         ts.execute();
         drop(ts);
 
         let skill_data = SkillTransactionPersistence::build(&mut db);
         let skill = skill_data.get_skill(&skill_id).unwrap();
         assert_eq!(skill.get_logo(), &new_logo);
     }
 
 
     #[test]
     fn test_update_is_current_skill() {
         let mut db = DataPersistence::new();
         
         setup(&mut db);
 
         let skill_id = String::from("Skill1");
 
         let is_current = true;
 
         let skill_data = Box::new(SkillTransactionPersistence::build(&mut db));
         let mut ts = TransactionUpdateIsCurrentSkill::new(
             skill_data,
             &skill_id, 
             is_current,
         );
         ts.execute();
         drop(ts);
 
         let skill_data = SkillTransactionPersistence::build(&mut db);
         let skill = skill_data.get_skill(&skill_id).unwrap();
         assert_eq!(skill.get_is_current(), is_current);
     }
 }