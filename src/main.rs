#![feature(plugin, decl_macro,custom_derive)]
#![plugin(rocket_codegen)]

extern crate card_deck;
extern crate rand;
extern crate rocket;
extern crate sqlite;
extern crate bcrypt;
extern crate rocket_contrib; //Consider #[macro_use]

#[macro_use] extern crate serde_derive;

use rocket::{State};
use rocket::http::{Cookies,Cookie};
use rocket::request::{Form};
use rocket::response::{NamedFile};

use std::io;
use std::sync::Mutex;

mod login;
use login::{DbUser,Cred};
mod session;
use session::Session;

mod scs_error;
//use scs_error::SCServerErr;

mod pre_game;




#[get("/")]
fn index()->io::Result<NamedFile>{
    NamedFile::open("site/index.html")
}


#[post("/login", data="<cred>")]
fn login(cred:Form<Cred>,state:State<Session>,mut cookies:Cookies)->io::Result<NamedFile>{

    let s = state.inner();
    let cred = cred.into_inner();

    println!("Login happening");
    let uid = match DbUser::get(cred){
        Ok(u)=> s.logins.add_user(u),
        _=> return NamedFile::open("site/no-login.html"),
    };

    cookies.add(Cookie::new("user_id",uid.to_string()));

    NamedFile::open("site/home.html")
}

#[post("/new-user",data="<cred>")]
fn new_user(cred:Form<Cred>,state:State<Session>,mut cookies:Cookies)->io::Result<NamedFile>{
    //make sure can add user to db
    let cred = cred.into_inner(); 

    let user = match DbUser::new(cred){
        Ok(u)=>u,
        Err(e)=>{
            println!("Login Error :{:?}",e);
            return NamedFile::open("site/no-login.html");
        },
    };

    //create session for use
    let s = state.inner();
    let uid = s.logins.add_user(user);
    cookies.add(Cookie::new("user_id",uid.to_string()));

    NamedFile::open("site/home.html")
}



fn main() {
    rocket::ignite().mount("/",routes![index,pre_game::new_game,pre_game::join_game,login,new_user])
        .manage(Mutex::new("Hello".to_string()))
        .manage(Session::new())
        .launch();
}
