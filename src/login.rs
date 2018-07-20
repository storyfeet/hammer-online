use std::collections::HashMap;
use std::sync::Mutex;

use rand::random;

use sqlite;

const USER_DB:&str = "dbase/users.db";

#[derive(Clone,PartialEq,Debug,Deserialize,FromForm)]
pub struct User {
    pub username:String,
    password:String,
}

impl User{
    pub fn dbnew(&self)->Result<(),String>{
        let conn = sqlite::open(USER_DB).expect("Could not open USER_DB");
        match conn.execute(format!("insert into users (username,password) values ('{}','{}' );",self.username,self.password)){
            Ok(_)=>Ok(()),
            Err(e)=>{
                println!("User already exists");
                Err(format!("Could not add user: {}",e))
            }
        }
    }

    pub fn pwcheck(&self)->bool{
        let conn= sqlite::open(USER_DB).expect("Could not open USER_DB for pwcheck");
        let mut res = false;
        conn.iterate(format!("select * from users where username = '{}';", self.username),|pairs|{
            for &(k,v) in pairs.iter(){
                println!("{} = {:?}",k,v);
                match k {
                    "password"=>res =( v==Some(&self.password)),
                    _=>{},
                }
            }
            true
        }).expect("Coult not read db in pwcheck");
        return res;
        
    }

}

pub struct Session{
    users:Mutex<HashMap<u64,User>>,
}

impl Session{
    pub fn new()->Self{
        Session{
            users:Mutex::new(HashMap::new()),
        }
    }

    pub fn add_user(&self, user:User)->u64{
        loop {
            let n:u64= random();
            let mut map = self.users.lock().unwrap();
            if let Some(_) = map.get(&n) {continue;}

            map.insert(n,user);
            return n;
        }
    }

    pub fn get_user(&self,id:u64)->Option<User>{ 
        let map = self.users.lock().unwrap();
        map.get(&id).map(|x| x.clone())
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




