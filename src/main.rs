use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "King", "Queen", "Jack",  "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
        
        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
    
            }
        }
        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Here is your deck: {:#?}", deck);
}
