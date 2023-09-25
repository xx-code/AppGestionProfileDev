use domains::{
    repositories::admin_transaction_repository::AdminTransactionRepository, 
    admin::transaction_create_admin::TransactionCreateAdmin, transaction::Transaction
};
use json::object::Object;
use uuid::Uuid;
use password_hash::PasswordHash;

pub struct AdminController<'a> {
    db: Box<dyn AdminTransactionRepository + 'a>,
}

impl AdminController<'_> {
    pub fn new<'a>(db: Box<dyn AdminTransactionRepository + 'a>) -> AdminController {
        AdminController { db }
    }
}

impl AdminController<'_> {
    pub fn create_new_admin(&self, request: Object) -> Result<json::JsonValue, json::JsonValue> {
        let admin_id = Uuid::new_v4().to_string();

        let mut username = String::new();
        let mut password = String::new();

        if request["username"].is_null() || request["password"].is_null() {
            let err = json::object! {
                success: false,
                error: "bad admin creation request",
                msg: r#"
                    structure reqeust is:
                    {
                        username: "username",
                        password: "password"
                    }
                "#
            };

            return Err(err)
        }

        username = request["username"].to_string();

        password = PasswordHash::new(&request["password"].as_str().unwrap()).unwrap().to_string();


        let transaction = TransactionCreateAdmin::new(
            self.db, 
            &admin_id, 
            &username, 
            &password
        );
        let res = transaction.execute();
        
        let mut error_response = String::new();

        if res.is_ok() {
            return Ok(json::object! {
                success: true
            });
        } else {
            let err = res.err().unwrap();
            
            return Err(json::object! {
                success: false,
                error: error_response,
                msg: ""
            })
        }
    }
}
