use ortalib::{Card, Enhancement, Suit};

#[derive(Debug, Clone, Default)]
pub struct HandMetaData {
    pub rank_count: [u8; 13],
    pub suit_count: [u8; 4],
    pub color_count: [u8; 2],
    pub type_count: [u8; 6],
    pub cards: Vec<Card>,
    pub wild_count: u8,
    pub contains_flush: bool,
    pub contains_straight: bool,
}

impl HandMetaData {

    pub fn new(rank_count: [u8; 13], 
        suit_count: [u8; 4], 
        color_count: [u8; 2], 
        type_count: [u8; 6], 
        cards_played:Vec<Card>, 
        wild_count: u8, 
        contains_flush: Option<bool>, 
        contains_straight: Option<bool>
    ) -> Self {
        HandMetaData {
            rank_count,
            suit_count,
            color_count,
            type_count,
            wild_count,
            cards: cards_played,
            contains_flush: contains_flush.unwrap_or(false),
            contains_straight: contains_straight.unwrap_or(false),
        }
    }

    pub fn number_of_pair(&self) -> u8 {
        let mut pair_count = 0;
        for i in 2..6 {
            pair_count += self.type_count[i];
        }
        pair_count
    }

    pub fn has_three(&self) -> bool {
        for i in 3..6 {
            if self.type_count[i] >= 1 {
                return true;
            }
        }
        false
    }

    pub fn has_four(&self) -> bool {
        for i in 4..6 {
            if self.type_count[i] >= 1 {
                return true;
            }
        }
        false
    }

    pub fn has_five(&self) -> bool {
        self.type_count[5] >= 1
    }

    pub fn get_from_hand(card_played: &[Card]) -> HandMetaData {
        let mut rank_count:[u8;13] = [0;13];
        let mut suit_count:[u8;4] = [0;4];
        let mut color_count:[u8;2] = [0;2];
        let wild_count = HandMetaData::count_hands(card_played, &mut rank_count, &mut suit_count, &mut color_count);
        let mut type_count: [u8;6] = [0;6];
        for i in 0..13 {
            if rank_count[i] > 0 {
                type_count[rank_count[i] as usize] += 1;
            }
        }
        HandMetaData::new(rank_count, suit_count, color_count, type_count, card_played.to_vec(),wild_count, None, None)
    }

    fn count_hands(card_played: &[Card], rank_count: &mut [u8;13], suit_count: &mut [u8;4], color_count: &mut [u8;2]) -> u8 {
        let mut wild_count:u8 = 0;
        for card in card_played.iter() {
            match card.enhancement {
                Some(Enhancement::Wild) => {
                    suit_count.iter_mut().take(4).for_each(|count| *count += 1);
                    color_count.iter_mut().take(2).for_each(|count| *count += 1);
                    wild_count += 1;
                }
                _ => {
                    suit_count[card.suit as usize] += 1;
                    match card.suit {
                        Suit::Diamonds | Suit::Hearts => color_count[1] += 1,
                        Suit::Clubs | Suit::Spades => color_count[0] += 1,
                    }
                }
            }
            rank_count[card.rank as usize] += 1;
        }
        wild_count
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use ortalib::{Card, Rank, Suit, Enhancement};

    #[test]
    fn test_count_normal_cards() {
        let cards = vec![
            Card::new(Rank::Ace, Suit::Hearts, None, None),
            Card::new(Rank::King, Suit::Spades, None, None),
        ];
        let mut rank_count = [0; 13];
        let mut suit_count = [0; 4];
        let mut color_count = [0; 2];
        
        let wild_count = HandMetaData::count_hands(&cards, &mut rank_count, &mut suit_count, &mut color_count);
        
        assert_eq!(wild_count, 0);
        assert_eq!(rank_count[Rank::Ace as usize], 1);
        assert_eq!(rank_count[Rank::King as usize], 1);
        assert_eq!(suit_count[Suit::Hearts as usize], 1);
        assert_eq!(suit_count[Suit::Spades as usize], 1);
        assert_eq!(color_count[1], 1); // Red (Hearts)
        assert_eq!(color_count[0], 1); // Black (Spades)
    }

    #[test]
    fn test_count_wild_cards() {
        let mut cards = vec![
            Card::new(Rank::Two, Suit::Clubs, None, None),
        ];
        cards[0].enhancement = Some(Enhancement::Wild);
        
        let mut rank_count = [0; 13];
        let mut suit_count = [0; 4];
        let mut color_count = [0; 2];
        
        let wild_count = HandMetaData::count_hands(&cards, &mut rank_count, &mut suit_count, &mut color_count);
        
        assert_eq!(wild_count, 1);
        assert_eq!(suit_count, [1, 1, 1, 1]); // All suits incremented
        assert_eq!(color_count, [1, 1]); // Both colors incremented
        assert_eq!(rank_count[Rank::Two as usize], 1);
    }
}