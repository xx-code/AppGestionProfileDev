#[cfg(test)]
mod tests {
    use controllers::errors::admin::ErrorControllerAdmin;
    use persistence::{
        data_persistence::DataPersistence, 
        admin_transaction_persistence::AdminTransactionPersistence
    };
    
    #[test]
    fn test_use_controller_admin_to_create_admin() {
        let db = DataPersistence::new();
        let admin_controller = AdminController::new(
            db
        );

        let request = json::object!{
            "username": "new_user",
            "password": "123tiomo"
        };
        let true_response = json::object! {
            success: true,
        };

        let response = admin_controller.create_new_admin(&request);

        assert_eq!(response.is_ok(), true);
        assert_eq!(response.ok().unwrap(), true_response);
    }

    fn test_use_baddly_controller_admin_to_create_admin() {
        let db = DataPersistence::new();
        let admin_controller = setup(&mut db);

        let request = json::object!{
            usename: "value mont"
        };
        let true_response = json::object!{   
            success: false,
            error: "bad admin creation request",
            msg: "
                structure reqeust is:
                {
                    username: \"username\",
                    password: \"password\"
                }
            "
        };

        let response = admin_controller.create_new_admin(&request);

        assert_eq!(response.is_ok(), false);
        assert_eq!(response.err().unwrap(), true_response);
    }

}