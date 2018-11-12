use std::collections::HashMap;
use std::sync::Mutex;

use bcrypt::{hash,verify};
use rand::random;

use rocket::http::Cookies;
use serde_derive::{Deserialize};

use sqlite;
use sqlite::State;

use crate::scs_error::SCServerErr;


const USER_DB:&str = "dbase/users.db";

#[derive(Clone,PartialEq,Debug,Deserialize,FromForm)]
pub struct Cred{
    pub username:String,
    password:String,
}

#[derive(Clone,PartialEq,Debug,Deserialize,FromForm)]
pub struct DbUser {
    pub username:String,
}


impl DbUser{
    pub fn new(c:Cred)->Result<DbUser,SCServerErr>{
        let conn = sqlite::open(USER_DB).expect("Could not open USER_DB");
        let hpass = hash(&c.password,10)?;
        let mut st = conn.prepare("insert into users (username,password) values ( ? , ?)").unwrap();
        st.bind(1,&c.username as &str)?;
        st.bind(2,&hpass as &str)?;

        st.next()?;//next put I guess

        Ok(DbUser{username:c.username})
    }

    pub fn get(c:Cred)->Result<DbUser,SCServerErr>{    
        let conn= sqlite::open(USER_DB)?;

        let mut st = conn.prepare("select username, password from users where username = ?;")?;

        st.bind(1,&c.username as &str)?; 

        if let Ok(State::Row) = st.next(){
            let phash = st.read::<String>(1)?;

            return match verify(&c.password,&phash) {
                Ok(true)=>Ok(DbUser{username:c.username.clone()}),
                _ => Err(SCServerErr::PasswordFail),
            }
        }
        Err(SCServerErr::NotFound)
    }

}

pub struct Logins(Mutex<HashMap<u64,DbUser>>);

impl Logins{
    pub fn new()->Self{
        Logins(Mutex::new(HashMap::new()))
    }

    pub fn add_user(&self, user:DbUser)->u64{
        loop {
            let n:u64= random();
            let mut map = self.0.lock().unwrap();//TODO Fix Error
            if let Some(_) = map.get(&n) {continue;}

            map.insert(n,user);
            return n;
        }
    }

    pub fn get_user(&self,id:u64)->Option<DbUser>{ 
        let map = self.0.lock().unwrap();
        map.get(&id).map(|x| (x).clone())
    }

    pub fn user_from_cookie(&self,ck:&Cookies)->Result<DbUser,SCServerErr>{
        let uid:u64 = ck.get("user_id").ok_or(SCServerErr::NoCookie)?.value().parse()?;
        self.get_user(uid).ok_or(SCServerErr::NoUser)
    }
    
}


#[cfg(test)]
mod tests{
    use login::*;
    #[test]
    fn test_add_users(){
        let sess = Logins::new(); 
        let daveid = sess.add_user(DbUser{username:"dave".to_string()});
        let _peteid = sess.add_user(DbUser{username:"pete".to_string()});

        assert_eq!(sess.get_user(daveid),Some(DbUser{username:"dave".to_string()}));
    }
}




