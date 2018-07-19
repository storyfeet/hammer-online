use std::collections::HashMap;
use std::sync::Mutex;

use rand::random;


#[derive(Clone,PartialEq,Debug,Deserialize,FromForm)]
pub struct User {
    pub username:String,
    
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




