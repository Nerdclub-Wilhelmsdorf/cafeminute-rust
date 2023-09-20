use rocket::{form::Form, Shutdown};
mod products;
mod structs;
mod db;
mod open_close;
mod usercount;
mod news;


use crate::structs::SetOpen;
#[macro_use] extern crate rocket;
#[get("/")]
fn root() -> &'static str {
   return "Root Route!"
}
static SERVER_PSWD: &str = "123";
#[post("/shutdown", data = "<form>")]
fn shutdown(form: Form<SetOpen>,shutdown: Shutdown) -> &'static str {
   if form.pswd != SERVER_PSWD {
       return "Wrong password";
   }
   shutdown.notify();
   return "Shutting down server";
}
#[launch]
fn rocket() -> _ {
   let figment = rocket::Config::figment()
   .merge(("address", "0.0.0.0"))
   .merge(("port", 1312));
   rocket::custom(figment).mount("/", routes![root])
                   .mount("/", routes![open_close::setopen])
                    .mount("/", routes![open_close::isopen])
                    .mount("/", routes![usercount::getcustomers])
                    .mount("/", routes![usercount::setcustomers])
                    .mount("/", routes![open_close::getschedule])
                    .mount("/", routes![open_close::addschedule])

                    .mount("/", routes![products::addproduct])
                    .mount("/", routes![products::listproducts])
                    .mount("/", routes![products::getproducts])
                    .mount("/", routes![products::getproduct])
                    .mount("/", routes![products::getproductids])
                    .mount("/", routes![products::changeproduct])
                    .mount("/", routes![products::removeproduct])

                    .mount("/", routes![news::addnews])
                    .mount("/", routes![news::getnewsids])
                    .mount("/", routes![news::getnews])
                    .mount("/", routes![news::getallnews])
                    .mount("/", routes![news::listnews])
                    .mount("/", routes![news::changenews])
                    .mount("/", routes![news::removenews])
                    .mount("/", routes![news::addparticipant])
                    .mount("/", routes![news::removeparticipant])
                    .mount("/", routes![shutdown])


}
