#[cfg(test)]
mod test {
    use crate::{
        transaction_create_admin::TransactionCreateAdmin, 
        transaction::Transaction, 
        DB::GLOBAL_DB,
       transaction_create_profile::TransactionCreateProfile,
    };

    fn setup() {
        let admin_id = String::from("admin_1");
        let username = String::from("usern");
        let password = String::from("password");

        let profile_id = String::from("profile1");
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
            &profile_id,
            &firstname,
            &lastname,
            &email_address,
            &phone_number,
        );
        ts.execute();
    }

    #[test]
    fn test_delete_profile() {
        setup();
        let profile_id = String::from("profile1");
        let ts = TransactionDeleteProfile::new(&profile_id);
        ts.execute();

        unsafe {
            let profile = GLOBAL_DB.get_profile(&profile_id);
            assert!(profile.is_none());
        }
    }
}