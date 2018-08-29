#![feature(plugin, decl_macro,custom_derive)]
#![plugin(rocket_codegen)]

extern crate card_deck;
extern crate rand;
extern crate rocket;
extern crate sqlite;
extern crate bcrypt;
extern crate rocket_contrib; //Consider #[macro_use]
extern crate arc_map;
extern crate shoehorn_circle;

#[macro_use] extern crate serde_derive;

use rocket::{State};
use rocket::http::{Cookies,Cookie};
use rocket::request::{Form};
use rocket::response::{NamedFile,Redirect};

use std::io;
use std::sync::Mutex;

mod login;
use login::{DbUser,Cred};
mod session;
use session::Session;

mod scs_error;
//use scs_error::SCServerErr;

mod pre_game;

mod active;



#[get("/")]
fn index()->io::Result<NamedFile>{
    NamedFile::open("site/index.html")
}


#[get("/<path>")]
fn static_site(path:String)->io::Result<NamedFile>{
    NamedFile::open(format!("site/{}",path))
}


#[post("/login", data="<cred>")]
fn login(cred:Form<Cred>,state:State<Session>,mut cookies:Cookies)->Redirect{


    let s = state.inner();
    let cred = cred.into_inner();


    println!("Login happening");
    let uid = match DbUser::get(cred.clone()){
        Ok(u)=> s.logins.add_user(u),
        _=> return Redirect::to("no-login.html"),
    };

    cookies.add(Cookie::new("user_id",uid.to_string()));
    cookies.add(Cookie::new("user_name",cred.username));

    Redirect::to("home.html")
}

#[post("/new-user",data="<cred>")]
fn new_user(cred:Form<Cred>,state:State<Session>,mut cookies:Cookies)->Redirect{
    //make sure can add user to db
    let cred = cred.into_inner(); 

    let user = match DbUser::new(cred){
        Ok(u)=>u,
        Err(e)=>{
            println!("Login Error :{:?}",e);
            return Redirect::to("/no-login.html");
        },
    };

    //create session for use
    let s = state.inner();
    let uid = s.logins.add_user(user);
    cookies.add(Cookie::new("user_id",uid.to_string()));

    Redirect::to("/home.html")
}



fn main() {
    rocket::ignite().mount("/",routes![
                                index,login,new_user,static_site,
                                pre_game::view_games,
                                pre_game::join_game,
                                pre_game::leave_game,
                                pre_game::begin_game,
                                active::show_from, 
                            ])
        .manage(Mutex::new("Hello".to_string()))
        .manage(Session::new())
        .launch();
}
