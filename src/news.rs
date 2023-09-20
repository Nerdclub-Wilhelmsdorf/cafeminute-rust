/*	e.POST("addnews", addnews) //working //secured
	e.GET("getnewsids", getnewsids)
	e.GET("getnews", getnews)
	e.GET("getallnews", getallnews)
	e.GET("listnews", listnews)
	e.PATCH("changenews", changenews) */
use map_macro::hash_map;
use rocket::form::Form;
extern crate strip_markdown;
use strip_markdown::*;

use crate::{structs::{AddNews, GetProducts, ChangeProduct, RemoveProduct}, SERVER_PSWD, db, products::create_product_id};

 #[post("/addnews", data = "<form>")]
 pub fn addnews(form: Form<AddNews>) -> String {
    let pswd = form.clone().pswd;
    if pswd == SERVER_PSWD {
        let db: db::DB = db::DB{dir: String::from("news")};
        let heading = form.clone().heading;
        let content = form.clone().content;
        let image = form.clone().image;
        let event = form.clone().event;
        let content_raw = strip_markdown(content.as_str());
        let mut map = hash_map!{
            "heading" => heading.as_str(),
            "content" => content.as_str(),
            "image" => image.as_str(),
            "event" => event.as_str(),
            "contentRaw" => content_raw.as_str()
        };
        if event == "true" {
            map.insert("participants", "0");
        };
        let date = get_date();
        map.insert("date", date.as_str());
        db.add_document(create_product_id(), map);
        return "success".to_string();
        }
    return "forbidden".to_string();
    }

#[get("/getnewsids")]
pub fn getnewsids() -> String {
    let db = db::DB{dir: String::from("news")};
    let documents = db.list_documents();
    return documents.join(",");
    }
    #[get("/getnews", data = "<form>")]

pub fn getnews(form: Form<GetProducts>) -> String {
    let db = db::DB{dir: String::from("news")};
        let mut json_string = form.clone().id;
        json_string.push_str(".json");
        if !db.has_key(&json_string) {
            return "invalid ID".to_string();
        }
        let document = form.clone().id;
        let mut vec: Vec<String> = Vec::new();
        let values = db.read_document(document.to_owned());
        vec.push(format!("{}: {};", document, values["content"]));
        vec.push(format!("{}: {};", document, values["contentRaw"]));
        vec.push(format!("{}: {};", document, values["date"]));
        vec.push(format!("{}: {};", document, values["heading"]));
        vec.push(format!("{}: {};", document, values["image"]));
        vec.push(format!("{}: {};", document, values["event"]));
        if values["event"] == "true" {
            vec.push(format!("{}: {};", document, values["participants"]));
        }
        return vec.join("\n");
    }
#[get("/getallnews")]
pub fn getallnews() -> String {
    let db = db::DB{dir: String::from("news")};
    let documents = db.list_documents();
    let mut vec: Vec<String> = Vec::new();
    for document in documents {
        let values = db.read_document(document.to_owned());
        vec.push(format!("{}: {};", document, values["content"]));
        vec.push(format!("{}: {};", document, values["contentRaw"]));
        vec.push(format!("{}: {};", document, values["date"]));
        vec.push(format!("{}: {};", document, values["heading"]));
        vec.push(format!("{}: {};", document, values["image"]));
        vec.push(format!("{}: {};", document, values["event"]));
        if values["event"] == "true" {vec.push(format!("{}: {};", document, values["participants"]))} else{vec.push(format!("{}: {};", document, "nv"))};
        vec.push("|".to_string());
    }
    return vec.join("\n");
}
#[get("/listnews")]
pub fn listnews() -> String {
    let db = db::DB{dir: String::from("news")};
    let documents = db.list_documents();
    let mut vec: Vec<String> = Vec::new();
    for document in documents {
        let values = db.read_document(document.to_owned());
        vec.push(format!("{}: {}", document, values["heading"]))
    }
    return vec.join("\n");
}
fn get_date() -> String{
   let now = chrono::offset::Local::now();
   let date = now.format("%Y-%m-%d").to_string();
   return date;
}
#[patch("/changenews", data = "<form>")]
pub fn changenews(form: Form<ChangeProduct>) -> String {
    let db = db::DB{dir: String::from("news")};
    let pswd = form.clone().pswd;
    let id = form.clone().id;
    if pswd != SERVER_PSWD {
        return "forbidden".to_string();
    };
    let mut json_string = id.clone();
    json_string.push_str(".json");
    if !db.has_key(&json_string) {
        return "invalid ID".to_string();
    };
    let key = form.clone().key;
    let value = form.clone().value;
    let mut product = db.read_document(id.to_owned());
    let mut contains = false;
    for (key_loop, _value_loop) in product.clone() {
        if key_loop == key {
            contains = true;
        }
    }
    if !contains {
        return "invalid key".to_string();
    }
    product.remove(key.as_str());
    product.insert(key, value);
    println!("{:?}", id);
    db.add_document_owned(id, product);
    return "success".to_string();
}
#[delete("/removenews", data = "<form>")]
pub fn removenews(form: Form<RemoveProduct>) -> String {
    let db = db::DB{dir: String::from("news")};
    let pswd = form.clone().pswd;
    let id = form.clone().id;
    if pswd != SERVER_PSWD {
        return "forbidden".to_string();
    };
    let mut json_string = id.clone();
    json_string.push_str(".json");
    if !db.has_key(&json_string) {
        return "invalid ID".to_string();
    };
    db.remove_key(json_string);
    return "success".to_string();
}
#[patch("/addparticipant", data = "<form>")]
pub fn addparticipant(form: Form<RemoveProduct>) -> String {
    let db = db::DB{dir: String::from("news")};
    let id = form.clone().id;
    let mut json_string = id.clone();
    json_string.push_str(".json");
    if !db.has_key(&json_string) {
        return "invalid ID".to_string();
    };
    let mut product = db.read_document(id.to_owned());
    if product["event"] != "true" {
        return "not an event".to_string();
    }
    let mut participants = product["participants"].parse::<i32>().unwrap();
    participants += 1;

    product.remove("participants");
    product.insert("participants".to_string(), participants.to_string());
    db.add_document_owned(id, product);
    return "success".to_string();
}

#[patch("/removeparticipant", data = "<form>")]
pub fn removeparticipant(form: Form<RemoveProduct>) -> String {
    let db = db::DB{dir: String::from("news")};
    let id = form.clone().id;
    let mut json_string = id.clone();
    json_string.push_str(".json");
    if !db.has_key(&json_string) {
        return "invalid ID".to_string();
    };
    let mut product = db.read_document(id.to_owned());
    if product["event"] != "true" {
        return "not an event".to_string();
    }
    let mut participants = product["participants"].parse::<i32>().unwrap();
    if participants == 0 {
        return "no participants".to_string();
    }
    participants -= 1;
    product.remove("participants");
    product.insert("participants".to_string(), participants.to_string());
    db.add_document_owned(id, product);
    return "success".to_string();
}
