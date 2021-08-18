pub const CARD_HEIGHT: i32 = 96;
pub const CARD_WIDTH: i32 = 71;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Card {
    pub suit: u8,
    pub value: u8,
}

impl Card {
    pub fn is_ace(&self) -> bool {
        self.value == 0
    }
    pub fn is_red(&self) -> bool {
        self.suit % 2 == 0
    }
    pub fn follows(&self, previous: &Card) -> bool {
        previous.suit == self.suit && previous.value + 1 == self.value
    }
    pub fn follows_alternating(&self, previous: &Card) -> bool {
        previous.is_red() != self.is_red() && previous.value + 1 == self.value
    }

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
