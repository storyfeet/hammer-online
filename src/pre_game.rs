use crate::session::Session;
use crate::scs_error::SCServerErr;

use std::sync::Mutex;
use rocket::{State};
use rocket::http::Cookies;
use rocket_contrib::Json;
use serde_derive::{Serialize};
use rand::{thread_rng,Rng};


use shoehorn_circle::{Game,supply::Supply};

#[derive(Serialize,Debug)]
pub struct PreGames(Mutex<Vec<PreGame>>);

#[derive(Serialize,Debug,Clone)]
pub struct PreGame{
    name:String,
    players:Vec<String>,
    gid:Option<u32>,
}

//for use in impl Pregames
fn locked_in(v:&Vec<PreGame>,pname:&str)->bool{
    for pg in v {
        if pg.gid == None {continue};
        if let Some(_) = pg.players.iter().find(|p|*p == pname) {
            return true
        }
    }
    false
}

impl PreGames {
    pub fn new()->Self{
        PreGames(Mutex::new(Vec::new()))
    }



    pub fn join_game(&self,gname:String,pname:String)->Result<Vec<PreGame>,SCServerErr>{
        let mut ar = self.0.lock()?;

        if locked_in(&mut ar, &pname) {
            return Ok((*ar).clone());
        }

        let mut found = false; 

        for pg in (&mut *ar).into_iter() {
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
            gid:None,
        };
        ar.push(pg);
        Ok((*ar).clone())
    }

    pub fn leave_game(&self,pname:String)->Result<Vec<PreGame>,SCServerErr>{
        let mut ar = self.0.lock()?;

        if locked_in(&mut ar, &pname) {
            return Ok((*ar).clone());
        }

        for pg in (&mut *ar).into_iter() {
            pg.players.retain(|p|p != &pname);
        }
        ar.retain(|pg|! pg.players.is_empty());
        Ok((*ar).clone())
    }

    pub fn view(&self)->Result<Vec<PreGame>,SCServerErr>{
        Ok( (*self.0.lock()?).clone())
    }

    pub fn get_gid(&self,pname:&str)->Option<u32>{
        let ar = self.0.lock().unwrap();
        for gm in &*ar {
            if gm.gid == None {
                continue;
            }
            if let Some(_) = gm.players.iter().find(|p| *p == pname){
                return gm.gid;
            }
        }
        return None
    }

}

#[post("/join_game", data="<gname>")]
fn join_game(gname:Json<String>,state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{

    let sess = state.inner();
    let gname = gname.into_inner();

    let user = sess.logins.user_from_cookie(&cookies)?;

    let res = sess.pre_games.join_game(gname,user.username)?;

    Ok(Json(
        res
    ))
}

#[post("/leave_game")]
fn leave_game(state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{
    let sess = state.inner();
    let user = sess.logins.user_from_cookie(&cookies)?;
    Ok(Json(sess.pre_games.leave_game(user.username)?))
}

#[get("/view_games")]
fn view_games(state:State<Session>)->Result<Json<Vec<PreGame>>,SCServerErr>{
    let sess = state.inner();
    Ok(Json(sess.pre_games.view()?))
}

#[post("/begin_game")]
fn begin_game(state:State<Session>,cookies:Cookies)->Result<Json<Vec<PreGame>>,SCServerErr>{
    let sess = state.inner();
    let user = sess.logins.user_from_cookie(&cookies)?;

    let mut ar = sess.pre_games.0.lock()?;
    'outer: for pg in &mut (*ar) {
        if pg.players.len() == 0 { continue}//should not be possible
        if pg.players[0] == user.username {
            loop {
                let n = thread_rng().gen::<u32>();
                if sess.active.insert(
                                n,
                                Game::build(Supply::from_map(sess.cards.clone()))
                                    .player_names(pg.players.clone())
                                    .done()
                            )? {
                    pg.gid = Some(n);
                    break 'outer;
                }
            }
        }
    }
    Ok(Json(ar.clone()))


}

