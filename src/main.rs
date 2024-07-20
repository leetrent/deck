#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}
fn main() {
    let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
    let values = ["Ace", "King", "Queen", "Jack",  "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    
    let mut cards = vec![];
    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);

        }
    }
    let deck: Deck = Deck { cards };
    println!("Here is your deck: {:#?}", deck);
}
