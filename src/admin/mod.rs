pub struct Admin {
    admin_id: String,
    username: String,
    password: String,
}

impl Admin {
    pub fn new(admin_id: &String, username: &String, password: &String) -> Admin{
        Admin {
            admin_id: admin_id.clone(),
            username: username.clone(),
            password: password.clone(),
        }
    }
    pub fn get_id(&self,) -> &String {
        &self.admin_id
    }

    pub fn get_username(&self,) -> &String {
        &self.username
    }

    pub fn get_password(&self,) -> &String {
        &self.password
    }
}