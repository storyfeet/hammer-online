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

    pub fn join_game(&self,gname:String,pname:String)->Result<Vec<PreGame>,SCServerErr>{
        let mut ar = self.0.lock()?;
        let mut found = false; 

        for mut pg in (&mut *ar).into_iter() {
            pg.players.retain(|p|p != &pname);

            if pg.name == gname {
                pg.players.push(pname.clone()); 
                found = true;
            }
        }

        ar.retain(|pg|! pg.players.is_empty());

        if found {
            return Ok((*ar).clone())
        }

        let pg = PreGame{
            name:gname,
            players:vec![pname],
        };
        ar.push(pg);
        Ok((*ar).clone())
    }

    pub fn leave_game(&self,pname:String)->Result<Vec<PreGame>,SCServerErr>{
        let mut ar = self.0.lock()?;
        for mut pg in (&mut *ar).into_iter() {
            pg.players.retain(|p|p != &pname);
        }
        ar.retain(|pg|! pg.players.is_empty());
        Ok((*ar).clone())
    }

    pub fn view(&self)->Result<Vec<PreGame>,SCServerErr>{
        Ok( (*self.0.lock()?).clone())
    }

}

#[post("/join_game", data="<gname>")]
fn join_game(gname:Json<String>,state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{

    let sess = state.inner();
    let gname = gname.into_inner();

    let user = sess.logins.user_from_cookie(cookies)?;

    let res = sess.pre_games.join_game(gname,user.username)?;

    Ok(Json(
        res
    ))
}

#[post("/leave_game")]
fn leave_game(state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{
    let sess = state.inner();
    let user = sess.logins.user_from_cookie(cookies)?;
    Ok(Json(sess.pre_games.leave_game(user.username)?))
}

#[get("/view_games")]
fn view_games(state:State<Session>)->Result<Json<Vec<PreGame>>,SCServerErr>{
    let sess = state.inner();
    Ok(Json(sess.pre_games.view()?))
}

