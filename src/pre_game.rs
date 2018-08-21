use std::sync::Mutex;
use rocket::{State};
use rocket::http::Cookies;
use rocket_contrib::Json;
use session::Session;

pub struct PreGames(Mutex<Vec<PreGame>>);

#[derive(Serialize)]
pub struct PreGame{
    name:String,
    players:Vec<String>,
}

impl PreGames {
    pub fn new()->Self{
        PreGames(Mutex::new(Vec::new()))
    }
}


#[derive(Serialize)]
struct Response{
    reply:String,
    total:String,
}

#[post("/new_game", data="<sayer>")]
fn new_game(sayer:Json<String>,state:State<Session>,cookies:Cookies)->Option<Json<Response>>{

    let sess = state.inner();

    let uid:u64 = cookies.get("user_id")?.value().parse().ok()?;

    let user = sess.logins.get_user(uid)?;


    Some(Json(Response{
        reply:format!("{} said: {}",user.username,sayer.to_string()),
        total:"".to_string(),
    }))
}

