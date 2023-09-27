#[cfg(test)]
mod tests {
    use controllers::admin::{controller::AdminController, self};
    use persistence::{
        data_persistence::DataPersistence, 
        admin_transaction_persistence::AdminTransactionPersistence
    };
    
    #[test]
    fn test_use_controller_admin_to_create_admin() {
        let mut db = DataPersistence::new();
        let mut admin_db = AdminTransactionPersistence::build(&mut db);

        let admin_controller = AdminController::new();

        let request = json::object!{
            "username": "new_user",
            "password": "123tiomo"
        };

        let true_response = json::object! {
            success: true,
        };

        let response = admin_controller.create_new_admin(&request, &mut admin_db);

        assert_eq!(response.is_ok(), true);
        assert_eq!(response.ok().unwrap(), true_response);
    }

    #[test]
    fn test_use_baddly_controller_admin_to_create_admin() {
        let mut db = DataPersistence::new();

        let mut admin_db = AdminTransactionPersistence::build(&mut db);
        let admin_controller = AdminController::new();

        let request = json::object!{
            usename: "value mont"
        };
        let true_response = json::object!{   
            success: false,
            error: "bad admin creation request",
            msg: "
                structure request is:
                {
                    username: \"username\",
                    password: \"password\"
                }
            "
        };

        let response = admin_controller.create_new_admin(&request, &mut admin_db);

        assert_eq!(response.is_ok(), false);
        assert_eq!(response.err().unwrap(), true_response);
    }

    #[test]
    fn test_login_admin_user() {
        let mut db = DataPersistence::new();
        let mut admin_db = AdminTransactionPersistence::build(&mut db);

        let admin_controller = AdminController::new();

        let request = json::object!{
            "username": "new_user",
            "password": "123tiomo"
        };

        let true_response = json::object! {
            success: true,
        };

        let response = admin_controller.create_new_admin(&request, &mut admin_db);

        let response = admin_controller.login_admin(&request, &mut admin_db);

        assert_eq!(response.is_ok(), true);
        assert_eq!(response.ok().unwrap(), true_response);
    }

    #[test]
    fn test_error_in_login_admin_user() {
        let mut db = DataPersistence::new();
        let mut db = DataPersistence::new();
        let mut admin_db = AdminTransactionPersistence::build(&mut db);

        let admin_controller = AdminController::new();

        let request = json::object!{
            "username": "new_user",
            "password": "123tiomo"
        };

        let bad_request = json::object! {
            "username": "new_user",
            "password": "11tiomo"
        };

        let true_response = json::object! {
            success: false,
            error: "bad request",
            msg: ""
        };

        let response = admin_controller.create_new_admin(&request, &mut admin_db);
        
        let response = admin_controller.login_admin(&bad_request, &mut admin_db);

        assert_eq!(response.is_ok(), false);
        assert_eq!(response.err().unwrap(), true_response);
    }
}