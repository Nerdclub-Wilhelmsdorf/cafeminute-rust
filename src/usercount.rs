use std::sync::Mutex;

use rocket::form::Form;

use crate::{structs, SERVER_PSWD};


static USERS: Mutex<i8> = Mutex::new(1);

#[get("/getcustomers")]
pub fn getcustomers() -> String {
    let users = USERS.lock().unwrap();
    return format!("{}", *users);
}
#[post("/setcustomers", data = "<form>")]
pub fn setcustomers(form: Form<structs::SetCustomers>) -> String {
    let pswd = form.clone().pswd;
    let count = form.customers;
    if &count > &3 {
        return format!("{}", "forbidden");
    }
    if pswd == SERVER_PSWD {
        let mut users = USERS.lock().unwrap();
        *users = count;
        return format!("{}", "success");
    };
    format!("{}", "forbidden")
}

