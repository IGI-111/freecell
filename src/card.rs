pub const CARD_HEIGHT: i32 = 96;
pub const CARD_WIDTH: i32 = 71;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Card {
    pub suit: u8,
    pub value: u8,
}

impl Card {
    pub fn deck() -> Vec<Card> {
        let mut d = Vec::new();
        for suit in 0..4 {
            for value in 0..13 {
                d.push(Card { suit, value })
            }
        }
        d
    }
}
