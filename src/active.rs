use crate::session::Session;
use crate::scs_error::SCServerErr;

use rocket::{State};
use rocket::http::{Cookies,Cookie};
use rocket_contrib::Json;
use rocket::response::{NamedFile};
use serde_derive::{Serialize,Deserialize};
use std::io;
//use std::str::FromStr;

use shoehorn_circle::{Request,RequestType,Action};


#[get("/play/<gid>")]
fn play(gid:String,mut ck:Cookies)->io::Result<NamedFile>{
    ck.add(Cookie::new("gid",gid));
    NamedFile::open("site/play.html")
}

#[post("/game_after/<gid>",data="<a_from>")]
fn game_after(gid:String,a_from:Json<usize>,state:State<Session>)->Result<Json<Vec<Action>>,SCServerErr>{
    let sess = state.inner();

    let a_from = a_from.into_inner();

    let gid =  gid.parse::<u32>()?;

    let res = sess.active.on_do(gid,|gm| {
        gm.since(a_from).to_vec()
    })?;

    Ok(Json(res))
}


#[derive(Clone,Serialize,Deserialize,PartialEq,Debug)]
struct DoReq {
    from:usize,
    requests:Vec<RequestType>,
}


#[post("/request_actions/<gid>",data="<doreq>")]
fn request_actions(gid:String,doreq:Json<DoReq>,state:State<Session>,ck:Cookies)->Result<Json<Vec<Action>>,SCServerErr>{
    
    
    let gid =  gid.parse::<u32>()?;
    let sess = state.inner();
    let user = sess.logins.user_from_cookie(&ck)?;

    let doreq = doreq.into_inner();

    let res:Result<Vec<Action>,SCServerErr> = sess.active.on_do(gid,|gm|{
        for r in doreq.requests{
            let rq = Request{
                player_name:user.username.clone(),
                act:r,
            };
            gm.player_action(rq)?;
        }
        Ok(gm.since(doreq.from).to_vec())
    })?;

    Ok(Json(res?))
}




