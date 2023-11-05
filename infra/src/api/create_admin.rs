use persistence::data_persistence::DataPersistence;

#[post("/")]
fn index() {
}

pub fn routes(db: &mut DataPersistence) -> Vec<rocket::Route>{
    routes![index]
}