use login::Logins;
use pre_game::PreGames;

pub struct Session{
    pub logins:Logins,
    pub pre_games:PreGames,
}

impl Session{
    pub fn new()->Self{
        Session{
            logins:Logins::new(),
            pre_games:PreGames::new(),
        }
    }
}
