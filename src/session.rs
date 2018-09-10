use std::sync::Arc;

use login::Logins;
use pre_game::PreGames;
use arc_map::ArcMap;
use shoehorn_circle::{Game,card_set};

pub struct Session{
    pub logins:Logins,
    pub pre_games:PreGames,
    pub cards:Arc<card_set::CardSet>,
    pub active:ArcMap<u32,Game>,
}

impl Session{
    pub fn new()->Self{
        let c = Arc::new(card_set::CardSet::load("site/cards/cards.lz").unwrap()); //TODO allow config
        //println!("Cards = {:?}",c);
        Session{
            logins:Logins::new(),
            pre_games:PreGames::new(),
            cards:c,
            active:ArcMap::new(),
        }
    }
}
