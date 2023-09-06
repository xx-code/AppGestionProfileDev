#[cfg(test)]
mod tests {
    use std::alloc::GlobalAlloc;

    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction,
        DB::GLOBAL_DB,
    };

    #[test]
    fn transaction_create_profile() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let firstname = String::from("first");
        let lastname = String::from("last");
        let email_address = String::from("address");
        let phone_number = String::from("07056389");

        let ts = TransactionCreateAdmin::new(
            &admin_id,
            &username,
            &password,
        );
        ts.execute();

        let ts = TransactionCreateProfile::new(
            &admin_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&admin_id);
            
            assert_eq!(profile.get_firstname(), &firstname);
            assert_eq!(profile.get_lastname(), &lastname);
            assert_eq!(profile.get_email_address(), &email_address);
            assert_eq!(profile.get_phone_number(), &phone_number);
        }
    }

    #[test]
    fn down() {
        unsafe {
            GLOBAL_DB.clean_admin();
            GLOBAL_DB.clean_profile();
        }
    }
}