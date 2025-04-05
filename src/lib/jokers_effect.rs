use core::panic;

use ortalib::{Card, Chips, Edition, Joker, JokerCard, Mult, PokerHand, Rank, Round, Suit};

use crate::{card_type, hand_struct::HandMetaData, joker_func, joker_struct::JokerGroupData, joker_keys};

pub fn get_on_score(mut chips: Chips, mut mult: Mult, scoring_hand_data: &HandMetaData,joker_group_data: &JokerGroupData, explain: bool) -> (Chips, Mult)
{
    let valid_cards = &scoring_hand_data.cards;
    let on_scored_jokers = &joker_group_data.on_scored_jokers;
    let on_scored_retriggers = &joker_group_data.on_scored_retriggers;
    let all_face = joker_group_data.contains(joker_keys::PAREIDOLIA);
    let has_smeared = joker_group_data.contains(joker_keys::SMEARED_JOKER);
    let mut first_face = true;
    for card in valid_cards.iter() 
    {
        let loop_time = get_retrigger(card, on_scored_retriggers, all_face);

        for _ in 0..loop_time 
        {   
            chips += card.rank.rank_value();

            if explain 
            {
                println!("{}{} +{} chips ({} x {})", card.rank, card.suit, card.rank.rank_value(), chips, mult);
            }
            if explain 
            {
                if let Some(enhancement) = card.enhancement
                {
                print!("{}{} {} ", card.rank, card.suit, enhancement);
                }
            }
            
            
            (chips, mult) = card_type::get_enhancement(chips, mult, card.enhancement, true, explain);
            
            if explain
            {
                if let Some(edition) = card.edition
                {
                    print!("{}{} {} ", card.rank, card.suit, edition);
                }
            }


            (chips, mult) = card_type::get_edition(chips, mult, card.edition, explain);
            
            for joker in on_scored_jokers.iter() 
            {
                (chips, mult) = get_on_score_joker_effect(joker, chips, mult, card, all_face, has_smeared, first_face, explain);
            }
            
            if explain && loop_time > 1 {
                println!("retrigger {} Sock and Buskin", card);
            }
        }

        if first_face 
        {
            if all_face 
            {
                first_face = false;
            } 
            else 
            {
                match card.rank 
                {
                    Rank::Jack | Rank::Queen | Rank::King => 
                    {
                        first_face = false;
                    }
                    _ => {}
                    
                }
            }
        }
    }
    
    (chips, mult)
}

fn get_retrigger(card: &Card, jokers: &Vec<Joker>, all_face: bool) -> usize 
{
    let mut n = 1;
    for joker in jokers.iter() {
        match joker
        {
            Joker::Mime => {
                n += 1;
            }
            Joker::SockAndBuskin => {
                if all_face {
                    n += 1;
                } else {
                    match card.rank {
                        Rank::Jack | Rank::Queen | Rank::King => {
                            n += 1;
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                panic!("not retriggered joker: {:?}", joker);
            }
        }
    }
    n
}

fn get_on_score_joker_effect(joker: &Joker, mut chips: Chips, mut mult: Mult, card: &Card, all_face: bool, has_smeared: bool, first_face: bool, explain: bool) -> (Chips, Mult) 
{
    match joker {
        Joker::GreedyJoker => {
            match card.suit {
                Suit::Diamonds => {
                    mult += 3.0;
                    if explain {
                        println!("Greedy Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                Suit::Hearts if has_smeared => {
                    mult += 3.0;
                    if explain {
                        println!("Greedy Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::LustyJoker => {
            match card.suit {
                Suit::Hearts => {
                    mult += 3.0;
                    if explain {
                        println!("Lusty Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                Suit::Diamonds if has_smeared => {
                    mult += 3.0;
                    if explain {
                        println!("Lusty Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::WrathfulJoker => {
            match card.suit {
                Suit::Spades => {
                    mult += 3.0;
                    if explain {
                        println!("Wrathful Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                Suit::Clubs if has_smeared => {
                    mult += 3.0;
                    if explain {
                        println!("Wrathful Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::GluttonousJoker => {
            match card.suit {
                Suit::Clubs => {
                    mult += 3.0;
                    if explain {
                        println!("Gluttonous Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                Suit::Spades if has_smeared => {
                    mult += 3.0;
                    if explain {
                        println!("Gluttonous Joker: +3 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::Fibonacci => {
            match card.rank {
                Rank::Ace | Rank::Two | Rank::Three | Rank::Five | Rank::Eight => {
                    mult += 8.0;
                    if explain {
                        println!("Fibonacci: +8 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::ScaryFace => {
            if all_face {
                chips += 30.0;
                if explain {
                    println!("Scary Face: +30 chips ({} x {})", chips, mult);
                }
            } else {
                match card.rank {
                    Rank::Jack | Rank::Queen | Rank::King => {
                        chips += 30.0;
                        if explain {
                            println!("Scary Face: +30 chips ({} x {})", chips, mult);
                        }
                    }
                    _ => {}
                    
                }
            }
        }
        Joker::EvenSteven => {
            match card.rank {
                Rank::Two | Rank::Four | Rank::Six | Rank::Eight | Rank::Ten => {
                    mult += 4.0;
                    if explain {
                        println!("Even Steven: +4 mult ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::OddTodd => {
            match card.rank {
                Rank::Ace | Rank::Three | Rank::Five | Rank::Seven | Rank::Nine => {
                    chips += 31.0;
                    if explain {
                        println!("Odd Todd: +31 chips ({} x {})", chips, mult);
                    }
                }
                _ => {}
            }
        }
        Joker::Photograph => {
            if first_face {
                if all_face {
                    mult *= 2.0;
                    if explain {
                        println!("Photograph: x2 mult ({} x {})", chips, mult);
                    }
                } else {
                    match card.rank {
                        Rank::Jack | Rank::Queen | Rank::King => {
                            mult *= 2.0;
                            if explain {
                                println!("Photograph: x2 mult ({} x {})", chips, mult);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Joker::SmileyFace => {
            if all_face {
                mult += 5.0;
                if explain {
                    println!("Smiley Face: +5 mult ({} x {})", chips, mult);
                }
            } else {
                match card.rank {
                    Rank::Jack | Rank::Queen | Rank::King => {
                        mult += 5.0;
                        if explain {
                            println!("Smiley Face: +5 mult ({} x {})", chips, mult);
                        }
                    }
                    _ => {}
                }
            }
        }

        _ => {
            panic!("not on score joker in on score section encountered: {:?}", joker);
        }
    }
    (chips, mult)
}

pub fn get_on_hold(mut chips: Chips, mut mult: Mult, joker_group_data: &JokerGroupData, cards_on_hand: &Vec<Card>, explain: bool) -> (Chips, Mult) 
{
    let on_hold_jokers = &joker_group_data.on_hold_jokers;
    let on_hold_retriggers = &joker_group_data.on_hold_retriggers;
    let mut min_rank = 14;
    for card in cards_on_hand.iter() {
        min_rank = min_rank.min(card.rank as usize);
    }
    let mut min_index = 0;
    for i in 0..cards_on_hand.len() {
        if cards_on_hand[i].rank as usize == min_rank {
            min_index = i;
        }
    }

    let mut use_fist = false;
    for i in 0..cards_on_hand.len() 
    {   
        if min_index == i 
        {
            use_fist = true;
        }
        let card = &cards_on_hand[i];
        let mut loop_time = get_retrigger(card, on_hold_retriggers, false);
        while loop_time > 0
        {
            if explain 
            {
                if let Some(enhancement) = card.enhancement
                {
                    print!("{}{} {} ", card.rank, card.suit, enhancement);
                }
            }
            (chips, mult) = card_type::get_enhancement(chips, mult, card.enhancement, false, explain);
            (chips, mult) = perform_on_hold_joker_effect(on_hold_jokers, chips, mult, card, use_fist, explain);

            if explain && loop_time > 1 
            {
                println!("retrigger {} Mime", card);
            }
            loop_time -= 1;
        }
        use_fist = false;

    }
    (chips, mult)
}

fn perform_on_hold_joker_effect(on_hold_jokers: &Vec<Joker> ,mut chips: Chips, mut mult: Mult, card: &Card, use_fist: bool, explain: bool) -> (Chips, Mult) 
{
    for joker in on_hold_jokers.iter() 
    {
        match joker {

            Joker::RaisedFist => {
                if use_fist {
                    mult += 2.0 * card.rank.rank_value();
                    if explain {
                        println!("Raised Fist: +2 x {} mult ({} x {})", card.rank.rank_value(), chips, mult);
                    }
                }
                
            }
            Joker::Baron => {
                match card.rank {
                    Rank::King => {
                        mult *= 1.5;
                        if explain {
                            println!("Baron: x1.5 mult ({} x {})", chips, mult);
                        }
                    }
                    _ => {}
                }
            }
    
            _ => {
                panic!("not on hold joker in on hold section encountered: {:?}", joker);
            }
        }
    }
    (chips, mult)
}


pub fn get_independent(mut chips: Chips, mut mult: Mult, round: &Round, joker_group_data: &JokerGroupData, hand_data: &HandMetaData, scoring_hand_data: &HandMetaData, explain: bool) -> (Chips, Mult) 
{
    let inde_jokers = &joker_group_data.inde_jokers;
    let jokers = &round.jokers;
    let has_smeared = joker_group_data.contains(joker_keys::SMEARED_JOKER);
    for i in 0..jokers.len() {
        let joker_card = jokers.get(i).unwrap();
        if explain {
            if let Some(edition) = joker_card.edition {
                match edition {
                    Edition::Polychrome => {}
                    _ => {
                        print!("{} {}", joker_card.joker, joker_card.edition.unwrap());
                    }
                }
            }
        }
        match joker_card.edition {
            Some(Edition::Polychrome) => {
            }
            _ => {
                (chips, mult) = card_type::get_edition(chips, mult, joker_card.edition, explain);
            }
        }

        
        (chips, mult) = get_inde_joker_effect(&inde_jokers[i], chips, mult, hand_data, &round.cards_held_in_hand, &round.jokers,explain, has_smeared, scoring_hand_data);
        
        if explain {
            if let Some(edition) = joker_card.edition {
                match edition {
                    Edition::Polychrome => {
                        print!("{} {}", joker_card.joker, joker_card.edition.unwrap());
                    }
                    _ => {}
                }
            }
        }
        match joker_card.edition {
            Some(Edition::Polychrome) => {
                (chips, mult) = card_type::get_edition(chips, mult, joker_card.edition, explain);
            }
            _ => {}
        }

    }
    (chips, mult)
}


fn get_inde_joker_effect(joker: &Joker, mut chips: Chips, mut mult: Mult, hand_data: &HandMetaData, cards_on_hand: &Vec<Card>,jokers: & Vec<JokerCard>, explain: bool, has_smeared: bool, scoring_hand_data: &HandMetaData) -> (Chips, Mult) {
    let wild_count = hand_data.wild_count;
    let scoring_suit_count = &scoring_hand_data.suit_count;
    let scoring_color_count = &scoring_hand_data.color_count;
    let scoring_wild_count = scoring_hand_data.wild_count;
    match joker {
        Joker::Joker => {
            mult += 4.0;
            if explain {
                println!("Joker: +4 mult ({} x {})", chips, mult);
            }

        }
        Joker::JollyJoker => {
            if hand_data.number_of_pair() >= 1 {
                mult += 8.0;
                if explain {
                    println!("Jolly Joker: +8 mult ({} x {})", chips, mult);
                }
            }
        }
        Joker::ZanyJoker => {
            if hand_data.has_three() {
                mult += 12.0;
                if explain {
                    println!("Zany Joker: +12 mult ({} x {})", chips, mult);
                }
            }
        }
        Joker::MadJoker => {
            if hand_data.number_of_pair() >= 2 {
                mult += 10.0;
                if explain {
                    println!("Mad Joker: +10 mult ({} x {})", chips, mult);
                }
            }
        }
        Joker::CrazyJoker => {
            if scoring_hand_data.contains_straight {
                mult += 12.0;
                if explain {
                    println!("Crazy Joker: +12 mult ({} x {})", chips, mult);
                }
            }

        }   
        Joker::DrollJoker => {
            if scoring_hand_data.contains_flush {
                mult += 10.0;
                if explain {
                    println!("Droll Joker: +10 mult ({} x {})", chips, mult);
                }
            }
        }
        Joker::SlyJoker => {
            if hand_data.number_of_pair() >= 1 {
                chips += 50.0;
                if explain {
                    println!("Sly Joker: +50 chips ({} x {})", chips, mult);
                }
            }
        }
        Joker::WilyJoker => {
            if hand_data.has_three() {
                chips += 100.0;
                if explain {
                    println!("Wily Joker: +100 chips ({} x {})", chips, mult);
                }
            }
        }
        Joker::CleverJoker => {
            if hand_data.number_of_pair() >= 2 {
                chips += 80.0;
                if explain {
                    println!("Clever Joker: +80 chips ({} x {})", chips, mult);
                }
            }
        }
        Joker::DeviousJoker => {
            if scoring_hand_data.contains_straight {
                    chips += 100.0;
                    if explain {
                        println!("Devious Joker: +100 chips ({} x {})", chips, mult);
                    }
            }
        }
        Joker::CraftyJoker => {
            if scoring_hand_data.contains_flush {
                chips += 80.0;
                if explain {
                    println!("Crafty Joker: +80 chips ({} x {})", chips, mult);
                }
            }
        }
        Joker::AbstractJoker => {
            mult += 3.0 * jokers.len() as f64;
            if explain {
                println!("Abstract Joker: +3 x {} mult ({} x {})", jokers.len(), chips, mult);
            }
        }

        Joker::Blackboard => {
            let mut no_red = true;
            for card in cards_on_hand.iter() {
                if card.suit == Suit::Diamonds || card.suit == Suit::Hearts {
                    no_red = false;
                    break;
                }
            }
            if no_red {
                mult *= 3.0;
                if explain {
                    println!("Blackboard: x3 mult ({} x {})", chips, mult);
                }
            }
        }

        Joker::FlowerPot => {

            if has_smeared {
                let colors = scoring_color_count.iter().filter(|&&count| count >= 2).count(); 
                let tot = scoring_color_count.iter().sum::<u8>();
                if colors == 2 && tot - scoring_wild_count >= 4 {
                    mult *= 3.0;
                    if explain {
                        println!("Flower Pot: x3 mult ({} x {})", chips, mult);
                    }
                }
            } else {
                let unique_suits = scoring_suit_count.iter().filter(|&&count| count > wild_count).count();
                if unique_suits as u8 + scoring_wild_count == 4 {
                    mult *= 3.0;
                    if explain {
                        println!("Flower Pot: x3 mult ({} x {})", chips, mult);
                    }
                }
            }
        }
        _ => {}
    }
    (chips, mult)
}


