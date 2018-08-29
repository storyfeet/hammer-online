use rocket::{State};
use rocket::http::{Cookies,Cookie};
use rocket_contrib::Json;
use session::Session;
use scs_error::SCServerErr;

use shoehorn_circle::Action;


#[post("/game_after",data="<a_from>")]
fn show_from(a_from:Json<usize>,state:State<Session>,mut ck:Cookies)->Result<Json<Vec<Action>>,SCServerErr>{
    let sess = state.inner();
    let user = sess.logins.user_from_cookie(&ck)?;

    let a_from = a_from.into_inner();

    let gid =  match ck.get("gid"){ 
        Some(d)=>d.value().parse::<u32>()?,
        None => {
            let ckid = state.pre_games.get_gid(&user.username)
                                    .ok_or(SCServerErr::NotFound)?;
            ckid         
        }
    };
    ck.add(Cookie::new("gid",gid.to_string()));

    let res = sess.active.on_do(gid,|gm| {
        gm.since(a_from).to_vec()
    })?;

    Ok(Json(res))
}







