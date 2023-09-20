

use map_macro::hash_map;
use rocket::{form::Form};
use crate::{structs::{AddProduct, GetProducts, ChangeProduct, RemoveProduct}, SERVER_PSWD, db};
use random_string::generate;
const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
#[post("/addproduct", data = "<form>")]
pub fn addproduct(form: Form<AddProduct>) -> String {
    let pswd = form.clone().pswd;
    if pswd == SERVER_PSWD {
        let db: db::DB = db::DB{dir: String::from("products")};
        let title = form.clone().title;
        let allergenic = form.clone().allergenic;
        let prize = form.clone().prize;
        let description = form.clone().description;
        let sale = form.clone().sale;
        let calories = form.clone().calories;
        let image = form.clone().image;
        let mut map = hash_map!{
            "title" => title.as_str(),
            "allergenic" => allergenic.as_str(),
            "prize" => prize.as_str(),
            "description" => description.as_str(),
            "sale" => sale.as_str(),
            "calories" => calories.as_str(),
            "image" => image.as_str()
        };
        let toname = to_name(title.as_str());
        map.insert("name", toname.as_str());
        db.add_document(create_product_id(), map);
        return "success".to_string();
        }
    return "forbidden".to_string();
    }
pub fn create_product_id() -> String {
    let id: String = generate(5, CHARS);
    let mut id_json = "".to_owned();
    id_json.push_str(id.as_str());
    id_json.push_str(".json");
    let db: db::DB = db::DB{dir: String::from("products")};
    if db.has_key(&id_json) {
        return create_product_id();
    }
    return id;
}

fn to_name(title: &str) -> String {
   return title.to_lowercase().replace(" ", "");
}
#[get("/listproducts")]
pub fn listproducts() -> String {
    let db = db::DB{dir: String::from("products")};
    let documents = db.list_documents();
    let mut vec: Vec<String> = Vec::new();
    for document in documents {
        let values = db.read_document(document.to_owned());
        vec.push(format!("{}: {}", document, values["name"]))
    }
    return vec.join("\n");
}


#[get("/getproducts")]
pub fn getproducts() -> String {
    let db = db::DB{dir: String::from("products")};
    let documents = db.list_documents();
    let mut vec: Vec<String> = Vec::new();
    for document in documents {
        let values = db.read_document(document.to_owned());
        vec.push(format!("{}: {};", document, values["name"]));
        vec.push(format!("{}: {};", document, values["title"]));
        vec.push(format!("{}: {};", document, values["prize"]));
        vec.push(format!("{}: {};", document, values["calories"]));
        vec.push(format!("{}: {};", document, values["image"]));
        vec.push(format!("{}: {};", document, values["allergenic"]));
        vec.push(format!("{}: {};", document, values["sale"]));
        vec.push(format!("{}: {};", document, values["description"]));
        vec.push("|".to_string());
    }
    return vec.join("\n");
}

#[get("/getproduct", data = "<form>")]
pub fn getproduct(form: Form<GetProducts>) -> String {
    let db = db::DB{dir: String::from("products")};
    let mut json_string = form.clone().id;
    json_string.push_str(".json");
    if !db.has_key(&json_string) {
        return "invalid ID".to_string();
    }
    let document = form.clone().id;
    let mut vec: Vec<String> = Vec::new();
    let values = db.read_document(document.to_owned());
    vec.push(format!("{}: {};", document, values["name"]));
    vec.push(format!("{}: {};", document, values["title"]));
    vec.push(format!("{}: {};", document, values["prize"]));
    vec.push(format!("{}: {};", document, values["calories"]));
    vec.push(format!("{}: {};", document, values["image"]));
    vec.push(format!("{}: {};", document, values["allergenic"]));
    vec.push(format!("{}: {};", document, values["sale"]));
    vec.push(format!("{}: {};", document, values["description"]));
    return vec.join("\n");
}
#[get("/getproductids")]
pub fn getproductids() -> String {
let db = db::DB{dir: String::from("products")};
let documents = db.list_documents();
return documents.join(",");
}
#[patch("/changeproduct", data = "<form>")]
pub fn changeproduct(form: Form<ChangeProduct>) -> String {
    let db = db::DB{dir: String::from("products")};
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
    print!("{:?}", product["description"]);
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
#[delete("/removeproduct", data = "<form>")]
pub fn removeproduct(form: Form<RemoveProduct>) -> String {
    let db = db::DB{dir: String::from("products")};
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
