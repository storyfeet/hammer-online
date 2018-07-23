use std::collections::HashMap;
use std::sync::Mutex;

use bcrypt::{DEFAULT_COST,hash,verify};
use rand::random;

use sqlite;
use sqlite::State;

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
    pub fn new(c:Cred)->Result<DbUser,String>{
        let conn = sqlite::open(USER_DB).expect("Could not open USER_DB");
        let hpass = match hash(&c.password,10){
            Ok(s)=>s,
            Err(e)=>return Err(format!("{}",e)),
        };
        let mut st = conn.prepare("insert into users (username,password) values ( ? , ?)").unwrap();
        st.bind(1,&c.username as &str);
        st.bind(2,&hpass as &str);
        match st.next(){
            Ok(_)=>Ok(DbUser{username:c.username}),
            Err(e)=>{
                println!("User already exists");
                Err(format!("Could not add user: {}",e))
            }
        }
    }

    pub fn get(c:Cred)->Result<DbUser,String>{    
        let conn= sqlite::open(USER_DB).expect("Could not open USER_DB for pwcheck");
        let mut res = Err("Not found".to_string());

        let mut st = conn.prepare("select username, password from users where username = ?;").expect("Error preparing user read statement");

        st.bind(1,&c.username as &str); 

        if let Ok(State::Row) = st.next(){
            let phash = st.read::<String>(1).expect("Unable to read pw row in Dbase");
            res = match verify(&c.password,&phash) {
                Ok(true)=>Ok(DbUser{username:c.username.clone()}),
                _ => Err("Password doesn't match".to_string()),
            }
        }
        return res;
    }

}

pub struct Session{
    users:Mutex<HashMap<u64,DbUser>>,
}

impl Session{
    pub fn new()->Self{
        Session{
            users:Mutex::new(HashMap::new()),
        }
    }

    pub fn add_user(&self, user:DbUser)->u64{
        loop {
            let n:u64= random();
            let mut map = self.users.lock().unwrap();
            if let Some(_) = map.get(&n) {continue;}

            map.insert(n,user);
            return n;
        }
    }

    pub fn get_user(&self,id:u64)->Option<DbUser>{ 
        let map = self.users.lock().unwrap();
        map.get(&id).map(|x| (x).clone())
    }
}


#[cfg(test)]
mod tests{
    use login::*;
    #[test]
    fn test_add_users(){
        let sess = Session::new(); 
        let daveid = sess.add_user(User{username:"dave".to_string()});
        let _peteid = sess.add_user(User{username:"pete".to_string()});

        assert_eq!(sess.get_user(daveid),Some(User{username:"dave".to_string()}));
    }
}




