use ortalib::{Card, Enhancement, PokerHand};

use crate::{joker_keys, hand_struct::HandMetaData, joker_struct::JokerGroupData};


pub fn check_hand(hand_data: &HandMetaData, jokers: &JokerGroupData) -> (PokerHand, HandMetaData) 
{
    check_flush_five(hand_data, jokers)
}

pub fn check_flush(hand_data: &HandMetaData, jokers:&JokerGroupData) -> (bool, u8, u8)
{
    let suit_count = &hand_data.suit_count;
    let color_count = &hand_data.color_count;

    let (suit_pos, max_by_suit) = suit_count
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(i, count)| (i, *count))
        .unwrap_or((0, 0));

    let (color_pos, max_by_color) = color_count
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .map(|(i, count)| (i, *count))
        .unwrap_or((0, 0));

    if max_by_suit == 5 
    {
        (true, suit_pos as u8, 7)
    } 
    else if max_by_color == 4 && jokers.contains(joker_keys::FOUR_FINGERS)  && jokers.contains(joker_keys::SMEARED_JOKER)
    {
        (true, 7, color_pos as u8)
    } 
    else if max_by_suit == 4 && jokers.contains(joker_keys::FOUR_FINGERS)
    {
        (true, suit_pos as u8, 7)
    } 
    else if max_by_color == 5 && jokers.contains(joker_keys::SMEARED_JOKER)
    {
        (true, 7, color_pos as u8)
    } 
    else 
    {
        (false, 7, 7)
    }
}

pub fn check_flush_five(hand_data: &HandMetaData, jokers: &JokerGroupData) -> (PokerHand, HandMetaData) 
{
    let mut max = 0;
    let rank_count = &hand_data.rank_count;
    for count in rank_count.iter() 
    {
        if *count > max 
        {
            max = *count;
        }
    }

    if max != 5
    {
        return check_flush_house(hand_data, jokers);
    }

    let (is_flush, _, _) = check_flush(hand_data, jokers);
    let mut output_data = hand_data.clone();
    if is_flush {
        output_data.contains_flush = true;
        (PokerHand::FlushFive, output_data)
    } else {
        (PokerHand::FiveOfAKind, output_data)
    }

}

pub fn check_flush_house(hand_data: &HandMetaData, jokers:&JokerGroupData) -> (PokerHand, HandMetaData) 
{
    if !hand_data.has_three() || hand_data.number_of_pair() < 2
    {
        return check_flush_straight(hand_data, jokers);
    } 
    
    let (is_flush, _, _) = check_flush(hand_data, jokers);
    let mut output_data = hand_data.clone();
    if is_flush 
    {
        output_data.contains_flush = true;
        return (PokerHand::FlushHouse, output_data);
    } 
    (PokerHand::FullHouse, output_data)
}



pub fn check_flush_straight(hand_data: &HandMetaData, jokers: &JokerGroupData) -> (PokerHand, HandMetaData) 
{
    let rank_count = &hand_data.rank_count;

    let mut max_iter = 0;
    let mut cur = if rank_count[12] > 0 { 1 } else { 0 };
    let shortcut = jokers.contains(joker_keys::SHORTCUT);
    let mut allow_gap = shortcut;
    let mut straight_end_at = 15;

    rank_count.iter()
        .enumerate()
        .for_each(|(i, &count)| {
            if count > 0 {
                cur += 1;
                allow_gap = shortcut;
            } else if !allow_gap {
                if cur > max_iter {
                    max_iter = cur;
                    straight_end_at = (i + 12) % 13;
                }
                cur = 0;
                allow_gap = shortcut;
            } else {
                allow_gap = false;
            }
        });

    if cur > max_iter 
    {
        max_iter = cur;
        straight_end_at = 12;
    }

    if !(max_iter == 5 || (max_iter == 4 && jokers.contains(joker_keys::FOUR_FINGERS)))
    {
        return check_four(hand_data, jokers);
    }
    let mut valid_cards = Vec::new();
    let mut i = straight_end_at;
    while rank_count[i] == 0
    {
        i = (i + 12) % 13;
    }
    valid_cards.push(i);
    i = (i + 12) % 13;
    allow_gap = shortcut;
    for _ in 0..12 
    {
        if rank_count[i] > 0 
        {
            valid_cards.push(i);
            allow_gap = shortcut;
        } 
        else if !allow_gap 
        {
            break;
        } 
        else 
        {
            allow_gap = false;
        }
        i = (i + 12) % 13;
    }
    let (is_flush, suit, color) = check_flush(hand_data, jokers);
    if jokers.contains(joker_keys::SPLASH)
    {
        let mut output_data = hand_data.clone();
        if is_flush
        {
            output_data.contains_flush = true;
            output_data.contains_straight = true;
            (PokerHand::StraightFlush, output_data)
        }
        else
        {
            output_data.contains_straight = true;
            (PokerHand::Straight, output_data)    
        }
    }
    else 
    {
        let mut output_cards = Vec::new();
        let card_played = &hand_data.cards;
        for card in card_played.iter() 
        {
            if valid_cards.contains(&(card.rank as usize)) 
            {
                output_cards.push(*card);
                valid_cards.retain(|x| *x != card.rank as usize);
            }
        }
        if is_flush
        {
            let flush_output = add_flush(card_played, suit, color);
            for card in flush_output.iter() 
            {
                if !output_cards.contains(card)
                {
                    output_cards.push(*card);
                }
            }
            let mut output_data = HandMetaData::get_from_hand(&output_cards);
            output_data.contains_flush = true;
            output_data.contains_straight = true;
            (PokerHand::StraightFlush, output_data)
        }
        else 
        {
            let mut output_data = HandMetaData::get_from_hand(&output_cards);
            output_data.contains_straight = true;
            (PokerHand::Straight, output_data)    
        }

    }

}

pub fn check_four(hand_data: &HandMetaData, jokers: &JokerGroupData) -> (PokerHand,HandMetaData) 
{
    let rank_count = &hand_data.rank_count;
    let card_played = &hand_data.cards;

    let (max_rank, max_count) = rank_count
        .iter()
        .enumerate()
        .max_by_key(|&(_, count)| *count)  // Use explicit reference pattern
        .map(|(i, count)| (i, *count))
        .unwrap_or((14, 0));

    let (is_flush, suit, color) = check_flush(hand_data, jokers);
    let mut output_cards = Vec::new();
    if max_count == 4
    {
        if jokers.contains(joker_keys::SPLASH)
        {
            let mut output_data = hand_data.clone();
            output_data.contains_flush = is_flush;
            return (PokerHand::FourOfAKind, output_data);
        }
        for card in card_played.iter() 
        {
            if card.rank as usize == max_rank 
            {
                output_cards.push(*card);
            }
        }
        let mut output_data = HandMetaData::get_from_hand(&output_cards);
        output_data.contains_flush = is_flush;
        return (PokerHand::FourOfAKind, output_data);
    }

    if is_flush 
    {
        if jokers.contains(joker_keys::SPLASH)
        {
            let mut output_data = hand_data.clone();
            output_data.contains_flush = true;
            return (PokerHand::Flush, output_data);
        }
        let mut output_data = HandMetaData::get_from_hand(&add_flush(card_played, suit, color));
        output_data.contains_flush = true;
        (PokerHand::Flush, output_data)
    } 
    else if max_count == 3
    {

        if jokers.contains(joker_keys::SPLASH)
        {
            let output_data = hand_data.clone();
            return (PokerHand::ThreeOfAKind, output_data);
        }
        for card in card_played.iter() 
        {
            if card.rank as usize == max_rank 
            {
                output_cards.push(*card);
            }
        }
        let output_data = HandMetaData::get_from_hand(&output_cards);
        (PokerHand::ThreeOfAKind, output_data)
    }
    else if max_count == 2
    {
        check_two_pairs(hand_data, jokers)
    }
    else if jokers.contains(joker_keys::SPLASH)
    {
        return (PokerHand::HighCard, hand_data.clone());
    }
    else  
    {
        output_cards.push(
            card_played.iter()
                .max_by_key(|card| card.rank as usize)
                .copied()
                .unwrap_or(card_played[0])
        );
        (PokerHand::HighCard, HandMetaData::get_from_hand(&output_cards))
    }

}


pub fn add_flush(card_played: &[Card], suit:u8, color:u8) -> Vec<Card> 
{
    let mut output_cards = Vec::new();
    if suit != 7
    {
        for card in card_played.iter()
        {
            match card.enhancement 
            {
                Some(Enhancement::Wild) => {
                    output_cards.push(*card);
                }
                _ => {
                    if card.suit as u8 == suit
                    {
                        output_cards.push(*card);
                    }
                }
            }
        }
    } 
    else
    {
        for card in card_played.iter() 
        {
            match card.enhancement 
            {
                Some(Enhancement::Wild) => {
                    output_cards.push(*card);
                }
                _ => {
                    let card_color = card.suit as u8 % 2;
                    if card_color == color 
                    {
                        output_cards.push(*card);
                    }
                }
            }
        }
    }
    output_cards
}

pub fn check_two_pairs(hand_data: &HandMetaData, jokers: &JokerGroupData) -> (PokerHand, HandMetaData) 
{
    let pair_count = hand_data.number_of_pair();
    let card_played = &hand_data.cards;
    if pair_count == 2
    {
        if jokers.contains(joker_keys::SPLASH)
        {
            return (PokerHand::TwoPair, hand_data.clone());
        }
        let mut output_cards = Vec::new();
        for card in card_played.iter() 
        {
            if hand_data.rank_count[card.rank as usize] == 2
            {
                output_cards.push(*card);
            }
        }
        (PokerHand::TwoPair, HandMetaData::get_from_hand(&output_cards))
    }
    else if pair_count == 1
    {
        if jokers.contains(joker_keys::SPLASH)
        {
            return (PokerHand::Pair, hand_data.clone());
        }
        let mut output_cards = Vec::new();
        for card in card_played.iter() 
        {
            if hand_data.rank_count[card.rank as usize] == 2
            {
                output_cards.push(*card);
            }
        }
        return (PokerHand::Pair, HandMetaData::get_from_hand(&output_cards));
    }
    else 
    {
        if jokers.contains(joker_keys::SPLASH)
        {
            return (PokerHand::HighCard, hand_data.clone());
        }
        let output_cards = vec![
            card_played.iter()
                .max_by_key(|card| card.rank as usize)
                .copied()
                .unwrap_or(card_played[0])
        ];
        (PokerHand::HighCard, HandMetaData::get_from_hand(&output_cards))
    }
    
}



#[cfg(test)]
mod tests {
    use super::*;
    use ortalib::{Card, Rank, Suit};

    #[test]
    fn test_high_card_normal() {
        let cards = vec![
            Card::new(Rank::Ten, Suit::Hearts, None, None),
            Card::new(Rank::Five, Suit::Clubs, None, None),
            Card::new(Rank::King, Suit::Diamonds, None, None),
        ];
        let hand_data = HandMetaData::get_from_hand(&cards);
        let jokers = JokerGroupData::default();
        
        let (hand, result_data) = check_hand(&hand_data, &jokers);
        
        assert_eq!(hand, PokerHand::HighCard);
        assert_eq!(result_data.cards[0].rank, Rank::King);
    }

    #[test]
    fn test_high_card_with_splash() {
        let cards = vec![
            Card::new(Rank::Ten, Suit::Hearts, None, None),
            Card::new(Rank::Five, Suit::Clubs, None, None),
        ];
        let hand_data = HandMetaData::get_from_hand(&cards);
        let mut jokers = JokerGroupData::default();
        jokers.joker_map |= joker_keys::SPLASH;
        
        let (hand, result_data) = check_hand(&hand_data, &jokers);
        
        assert_eq!(hand, PokerHand::HighCard);
        assert_eq!(result_data.cards, cards); 
    }

    #[test]
    fn test_high_card_single_card() {
        let cards = vec![Card::new(Rank::Ace, Suit::Spades, None, None)];
        let hand_data = HandMetaData::get_from_hand(&cards);
        let jokers = JokerGroupData::default();
        
        let (hand, result_data) = check_hand(&hand_data, &jokers);
        
        assert_eq!(hand, PokerHand::HighCard);
        assert_eq!(result_data.cards[0].rank, Rank::Ace);
    }
}