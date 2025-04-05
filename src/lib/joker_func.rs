use ortalib::{Chips, Joker, JokerCard, Mult, PokerHand, Round};

use crate::{hand_func, jokers_effect, joker_keys, hand_struct::HandMetaData, joker_struct::JokerGroupData};

fn check_clone(vec: &mut Vec<Joker>, card: &JokerCard, count: &mut usize, is_inde: bool, cur_i : usize) {
    if *count > 0 {
        if is_inde {
            for i in cur_i - *count..cur_i {
                vec[i] = card.joker.clone();
            }
        } else {
            for _ in 0..*count {
                vec.push(card.joker.clone());
            }
        }
        *count = 0;
    };
}

pub fn group_jokers(round: &Round) -> JokerGroupData {
    let mut hands_jokers = vec![];
    let mut on_scored_jokers = vec![];
    let mut on_scored_retriggers = vec![];
    let mut on_hold_jokers = vec![];
    let mut on_hold_retriggers = vec![];
    let mut inde_jokers = vec![];
    let mut clone_count = 0;
    let mut joker_map: u64 = 0;
    for i in 0..round.jokers.len() {
        let card = round.jokers.get(i).unwrap();
        match card.joker{
            Joker::Joker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::JOKER;
            }
            Joker::JollyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::JOLLY_JOKER;
            }
            Joker::ZanyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::ZANY_JOKER;
            }
            Joker::MadJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::MAD_JOKER;
            }
            Joker::CrazyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::CRAZY_JOKER;
            }   
            Joker::DrollJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::DROLL_JOKER;
            }
            Joker::SlyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::SLY_JOKER;
            }
            Joker::WilyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::WILY_JOKER;
            }
            Joker::CleverJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::CLEVER_JOKER;
            }
            Joker::DeviousJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::DEVIOUS_JOKER;
            }
            Joker::CraftyJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::CRAFTY_JOKER;
            }
            Joker::AbstractJoker => {
                check_clone(&mut inde_jokers, card, &mut clone_count, true, i);
                joker_map |= joker_keys::ABSTRACT_JOKER;
            }
            Joker::RaisedFist => {
                on_hold_jokers.push(card.joker.clone());
                check_clone(&mut on_hold_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::RAISED_FIST;
            }
            Joker::Blackboard => {
                check_clone(&mut inde_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::BLACKBOARD;
            }
            Joker::Baron => {
                on_hold_jokers.push(card.joker.clone());
                check_clone(&mut on_hold_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::BARON;
            }
            Joker::GreedyJoker => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::GREEDY_JOKER;
            }
            Joker::LustyJoker => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::LUSTY_JOKER;
            }
            Joker::WrathfulJoker => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::WRATHFUL_JOKER;
            }
            Joker::GluttonousJoker => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::GLUTTONOUS_JOKER;
            }
            Joker::Fibonacci => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::FIBONACCI;
            }
            Joker::ScaryFace => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SCARY_FACE;
            }
            Joker::EvenSteven => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::EVEN_STEVEN;
            }
            Joker::OddTodd => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::ODD_TODD;
            }
            Joker::Photograph => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::PHOTOGRAPH;
            }
            Joker::SmileyFace => {
                on_scored_jokers.push(card.joker.clone());
                check_clone(&mut on_scored_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SMILEY_FACE;
            }
            Joker::FlowerPot => {
                check_clone(&mut inde_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::FLOWER_POT;
            }
            Joker::Shortcut => {
                hands_jokers.push(card.joker.clone());
                check_clone(&mut hands_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SHORTCUT;
            }
            Joker::Mime => {
                on_hold_retriggers.push(card.joker.clone());
                check_clone(&mut on_hold_retriggers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::MIME;
            }
            Joker::FourFingers => {
                hands_jokers.push(card.joker.clone());
                check_clone(&mut hands_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::FOUR_FINGERS;
            }
            Joker::Pareidolia => {
                joker_map |= joker_keys::PAREIDOLIA;
            }
            Joker::Splash => {
                hands_jokers.push(card.joker.clone());
                check_clone(&mut hands_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SPLASH;
            }
            Joker::SockAndBuskin => {
                on_scored_retriggers.push(card.joker.clone());
                check_clone(&mut on_scored_retriggers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SOCK_AND_BUSKIN;
            }
            Joker::SmearedJoker => {
                hands_jokers.push(card.joker.clone());
                check_clone(&mut hands_jokers, card, &mut clone_count, false, i);
                joker_map |= joker_keys::SMEARED_JOKER;
            }
            Joker::Blueprint => {
                clone_count += 1;
                joker_map |= joker_keys::BLUEPRINT;
            }
        }
        inde_jokers.push(card.joker.clone());
    }
    JokerGroupData {
        hands_jokers,
        on_scored_jokers,
        on_scored_retriggers,
        on_hold_jokers,
        on_hold_retriggers,
        inde_jokers,
        joker_map,
    }
}



pub fn get_hand(round: &Round, joker_group_data: &JokerGroupData) -> (PokerHand, HandMetaData, HandMetaData) {
    let hand_data = HandMetaData::get_from_hand(&round.cards_played);
    let (poker_hand, scoring_card_data) = hand_func::check_hand(&hand_data, &joker_group_data);
    (poker_hand,scoring_card_data, hand_data)
}



pub fn get_score(scoring_card_data: &HandMetaData, joker_group_data: &JokerGroupData, poker_hand: &PokerHand, round : &Round, explain: bool, hand_data: &HandMetaData) -> (Chips, Mult) 
{
    let (mut chips, mut mult) = poker_hand.hand_value();
    if explain {
        println!("{}, ({} X {})", &poker_hand, chips, mult);
    }
    (chips, mult) = jokers_effect::get_on_score(chips, mult,scoring_card_data, joker_group_data, explain);
    (chips, mult) = jokers_effect::get_on_hold(chips, mult, joker_group_data, &round.cards_held_in_hand, explain);
    (chips, mult) = jokers_effect::get_independent(chips, mult, round,joker_group_data,hand_data,scoring_card_data ,explain);
    (chips, mult)
}


#[cfg(test)]
mod tests {
    use super::*;
    use ortalib::{Joker, JokerCard};

    #[test]
    fn test_check_clone() {
        let card = JokerCard::new(Joker::Joker, None);
        let mut vec = vec![Joker::Blueprint, Joker::Blueprint, Joker::Blueprint];
        let mut count = 3;
        check_clone(&mut vec, &card, &mut count, true, 3);
        assert_eq!(vec.len(), 3);
        for joker in vec.iter() {
            assert_eq!(joker, &card.joker);
        }
        assert_eq!(count, 0);
    }

}