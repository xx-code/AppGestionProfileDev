use domains::{
    repositories::admin_transaction_repository::AdminTransactionRepository, 
    admin::transaction_create_admin::TransactionCreateAdmin, errors::admin::ErrorAdmin,
};
use uuid::Uuid;


pub struct AdminController;

impl AdminController{
    pub fn new() -> AdminController {
        AdminController
    }
}

impl AdminController {
    pub fn create_new_admin(&self, request: &json::JsonValue, repo: &mut impl AdminTransactionRepository) -> Result<json::JsonValue, json::JsonValue> {
        let admin_id = Uuid::new_v4().to_string();

        if request["username"].is_null() || request["password"].is_null() {
            let err = json::object! {
                success: false,
                error: "bad admin creation request",
                msg: r#"
                    structure request is:
                    {
                        username: "username",
                        password: "password"
                    }
                "#
            };

            return Err(err)
        }

        let username = request["username"].to_string();
        let password = request["password"].to_string();


        let transaction = TransactionCreateAdmin::new(
            &admin_id, 
            &username, 
            &password
        );

        let res = transaction.execute(repo);
        
        let mut error_response = String::new();

        if res.is_ok() {
            return Ok(json::object! {
                success: true
            });
        } else {
            let err = res.err().unwrap();

            match err {
                ErrorAdmin::AdminAlreadyExist => error_response = format!("Admin already exist"),
                ErrorAdmin::AdminNoExist => error_response = format!("Admin No Exist")
            }
            
            return Err(json::object! {
                success: false,
                error: error_response,
                msg: ""
            })
        }
    }

    pub fn login_admin(&self, request: &json::JsonValue, repo: &mut impl AdminTransactionRepository) -> Result<json::JsonValue, json::JsonValue>{

        if request["username"].is_null() || request["password"].is_null() {
            let err = json::object! {
                success: false,
                error: "bad admin creation request",
                msg: r#"
                    structure request is:
                    {
                        username: "username",
                        password: "password"
                    }
                "#
            };

            return Err(err)
        }

        let username = request["username"].to_string();

        let admin = repo.get_admin(&username).unwrap();


        Ok(json::object! { })
    }
}
