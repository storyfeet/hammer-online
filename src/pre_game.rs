use std::sync::Mutex;
use rocket::{State};
use rocket::http::Cookies;
use rocket_contrib::Json;
use session::Session;
use scs_error::SCServerErr;

#[derive(Serialize,Debug)]
pub struct PreGames(Mutex<Vec<PreGame>>);

#[derive(Serialize,Debug,Clone)]
pub struct PreGame{
    name:String,
    players:Vec<String>,
}

impl PreGames {
    pub fn new()->Self{
        PreGames(Mutex::new(Vec::new()))
    }

    pub fn add_game(&self,gname:String,pname:String)->Result<(),SCServerErr>{
        let mut ar = self.0.lock()?;

        for mut pg in (&mut *ar).into_iter() {
            if pg.name == gname {
                return Err(SCServerErr::ItemExistsAlready);
            }
            pg.players.retain(|p|p != &pname);
            
        }

        let pg = PreGame{
            name:gname,
            players:vec![pname],
        };
        ar.push(pg);
        Ok(())
    }

    pub fn view(&self)->Result<Vec<PreGame>,SCServerErr>{
        Ok( (*self.0.lock()?).clone())
    }

    pub fn join_game(&self,gname:String,pname:String)->Result<(),SCServerErr>{
        let mut ar = self.0.lock()?;
        for mut pg in (&mut *ar).into_iter() {
            pg.players.retain(|p|p != &pname);
            if pg.name == gname {
                pg.players.push(pname.clone()); 
            }
            
        }
        Ok(())
    }
}


#[post("/new_game", data="<gname>")]
fn new_game(gname:Json<String>,state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{

    let sess = state.inner();
    let gname = gname.into_inner();

    let uid:u64 = cookies.get("user_id").ok_or(SCServerErr::NoCookie)?.value().parse()?;
    let user = sess.logins.get_user(uid).ok_or(SCServerErr::NoUser)?;

    sess.pre_games.add_game(gname,user.username)?;


    Ok(Json(
        sess.pre_games.view()?
    ))
}


#[post("/join_game", data="<gname>")]
fn join_game(gname:Json<String>,state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{

    let sess = state.inner();
    let gname = gname.into_inner();

    let uid:u64 = cookies.get("user_id").ok_or(SCServerErr::NoCookie)?.value().parse()?;
    let user = sess.logins.get_user(uid).ok_or(SCServerErr::NoUser)?;

    sess.pre_games.join_game(gname,user.username)?;

    Ok(Json(
        sess.pre_games.view()?
    ))
}

