use card_deck::Deck;



pub struct Decks{
    goals:Deck,

}

fn deck_from<C>(cs:Vec<C>)->Deck<C>{
    Deck::build(k
}


pub enum CardType{
    Goal,
    Trait,
    Skill,
}

pub struct Card{
    name:String,
    kind:CardType,
    tokens:u8,
}



