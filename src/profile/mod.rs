/*pub struct ProfileData {
    pub admin_id: String,
    pub profile_id: String,
    pub firstname: String,
    pub lastname: String,
    pub email_address: String,
    pub phone_number: String
}*/

pub struct Profile {
    admin_id: String,
    profile_id: String,
    firstname: String,
    lastname: String,
    email_address: String,
    phone_number: String
}

impl Profile {
    pub fn new(admin_id: &String, profile_id: &String, firstname: &String, lastname: &String, email_address: &String, phone_number: &String) -> Profile {
        Profile {
            admin_id: admin_id.clone(),
            profile_id: profile_id.clone(),
            firstname: firstname.clone(),
            lastname: lastname.clone(),
            email_address: email_address.clone(),
            phone_number: phone_number.clone(),
        }
    }

    pub fn get_admin_id(&self,) -> &String {
        &self.admin_id
    }

    pub fn get_id(&self,) -> &String {
        &self.profile_id
    }

    pub fn get_firstname(&self,) -> &String {
        &self.firstname
    }

    pub fn get_lastname(&self,) -> &String {
        &self.lastname
    }

    pub fn get_email_address(&self,) -> &String {
        &self.email_address
    }

    pub fn get_phone_number(&self,) -> &String {
        &self.phone_number
    }
}