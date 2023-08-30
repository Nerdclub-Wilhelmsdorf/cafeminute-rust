use rocket::form::Form;
use crate::{structs::SetOpen, structs::Schedule, server_pswd, db};
use std::sync::Mutex;
use map_macro::hash_map;
static OPEN: Mutex<bool> = Mutex::new(false);



#[post("/setopen", data = "<form>")]
pub fn setopen(form: Form<SetOpen>) -> String {
    let pswd = form.into_inner().pswd;
    if server_pswd == pswd {
        let mut open = OPEN.lock().unwrap();
        *open = !*open;
    return format!("{}", "success");
    }
    format!("{}", "forbidden")
}

#[get("/isopen")]
pub fn isopen() -> String {
    let open = OPEN.lock().unwrap();
    return format!("{}", *open);
}

#[get("/getschedule")]
pub fn getschedule() -> String {
    let db = db::DB{dir: String::from("schedule")};
    if !db.has_key(&String::from("schedule.json")) {
        return format!("{}", "No schedule");
    }
    let schedule = db.read_document(String::from("schedule"));
    let values: Vec<String> = schedule.values().cloned().collect();
    let value_string = values.join("◌◌◞◌◌◌");
    return format!("{}", value_string);
}
#[post("/addschedule", data = "<form>")]
pub fn addschedule(form: Form<Schedule>) -> String {
    let pswd = form.clone().pswd;
    if server_pswd == pswd {
        let mon = form.clone().mon;
        let tue = form.clone().tue;
        let wed = form.clone().wed;
        let thu = form.clone().thu;
        let fri = form.clone().fri;
        let mut schedule = db::DB{dir: String::from("schedule")};
        let mut map = hash_map!{
            "mon" => mon.as_str(),
            "tue" => tue.as_str(),
            "wed" => wed.as_str(),
            "thu" => thu.as_str(),
            "fri" => fri.as_str()
        };
        schedule.add_document(String::from("schedule"), map);
        return format!("{}", "success");
    }else{
        return format!("{}", "forbidden");
}}
