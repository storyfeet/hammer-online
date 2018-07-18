#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;

use rocket::request::{Form};
use rocket::response::{NamedFile};
use rocket_contrib::{Json};

use std::io;

#[derive(Deserialize)]
struct Sayer {
    said:String,
}

#[derive(Serialize)]
struct Response{
    reply:String,
}

#[get("/")]
fn index()->io::Result<NamedFile>{
    NamedFile::open("site/index.html")
}

#[post("/say", data="<sayer>")]
fn say(sayer:Json<Sayer>)->Json<Response>{
    Json(Response{reply:format!("You said: {}",sayer.said).to_string()})

}

fn main() {
    rocket::ignite().mount("/",routes![index,say]).launch();
}
