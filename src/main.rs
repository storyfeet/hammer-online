#![feature(plugin, decl_macro,custom_derive)]
#![plugin(rocket_codegen)]

extern crate rand;
extern crate rocket;
extern crate sqlite;
extern crate bcrypt;
#[macro_use] extern crate rocket_contrib; //Consider #[macro_use]

#[macro_use] extern crate serde_derive;

use rocket::{State};
use rocket::http::{Cookies,Cookie};
use rocket::request::{Form};
use rocket::response::{NamedFile};
use rocket_contrib::{Json};

use std::io;
use std::sync::Mutex;

mod login;
use login::{Session,DbUser,Cred};


#[derive(Deserialize)]
struct Sayer {
    said:String,
}

#[derive(Serialize)]
struct Response{
    reply:String,
    total:String,
}

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
        Ok(u)=> s.add_user(u),
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
            println!("Login Error :{}",e);
            return NamedFile::open("site/no-login.html");
        },
    };

    //create session for use
    let s = state.inner();
    let uid = s.add_user(user);
    cookies.add(Cookie::new("user_id",uid.to_string()));

    NamedFile::open("site/home.html")
    
}


#[post("/say", data="<sayer>")]
fn say(sayer:Json<Sayer>,state:State<Session>,cookies:Cookies)->Option<Json<Response>>{

    let sess = state.inner();

    let uid:u64 = cookies.get("user_id")?.value().parse().ok()?;

    let user = sess.get_user(uid)?;


    Some(Json(Response{
        reply:format!("{} said: {}",user.username,sayer.said.to_string()),
        total:"".to_string(),
    }))

}

fn main() {
    rocket::ignite().mount("/",routes![index,say,login,new_user])
        .manage(Mutex::new("Hello".to_string()))
        .manage(Session::new())
        .launch();
}
