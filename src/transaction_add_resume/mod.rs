#[cfg(test)]
mod test {
    use time::Date;
    use crate::DB::GLOBAL_DB;
    fn setup() {
        let admin_id = String::from("admin1");
        let username = String::from("new_user");
        let password = String::from("password");

        let new_admin = TransactionCreateAdmin::new(
            &admin_id, 
            &username,
            &password
        );

        new_admin.execute();
    }
    #[test]
    fn test_add_resume_current() {
        let resume_id = String::from("resume1");
        let title = String::from("title - element");
        let description = String::from("description element");
        let type_resume = ResumeType::Education;
        let date_start = Date::from_calendar_date(2021, time::Month::January, 1).unwrap();

        let ts = TransactionAddResumeCurrent::new(
            &String::from("admin1"), 
            &resume_id,
            &title, 
            &description, 
            &type_resume, 
            &date_start
        );
        ts.execute();

        unsafe {
            let resume = GLOBAL_DB.get_resume(&resume_id);
            assert_eq!(resume.get_title(), &title);
            assert_eq!(resume.get_description(), &description);
            assert_eq!(resume.get_type_resume(), &type_resume);
            assert_eq!(resume.get_date_start(), &date_start);
        }
    }
}