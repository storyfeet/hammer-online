
pub struct BasicAction{
    name:String;
    text:String;
}

pub enum Action{
    Chat(BasicAction),
    Do(BasicAction),
    Say(BasicAction),
    Bid({name:String,amount:u8}),
    Reward({name:String,dice:u8,tokens:u8,card:String}),
}

pub struct Player{
    username:String,
    role:String,
    cards:Vec<Card>,
    tokens:u8,
    dice:u8,

}



pub struct Game{
    players:Vec<Player>,
    actions:Vec<Action>,
}

