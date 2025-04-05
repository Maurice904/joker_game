use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};

use clap::Parser;
use ortalib::{Chips, Mult, Round};
use my_lib::joker_func;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;
    let (chips, mult) = score(round, opts.explain);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }
    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round, explain: bool) -> (Chips, Mult) {
    if explain {
        println!("joker card : {:?}", &round.jokers);
        println!("cards played: {:?}", &round.cards_played);
        println!("cards held in hand: {:?}", &round.cards_held_in_hand);
    }
    let joker_group_data = joker_func::group_jokers(&round);
    let (poker_hand, scoring_card_data, hand_data) = joker_func::get_hand(&round, &joker_group_data);
    if explain {
        println!("counted cards: {:?}", &scoring_card_data.cards);
        println!("poker hand: {:?}", &poker_hand);
    }
    joker_func::get_score(&scoring_card_data, &joker_group_data, &poker_hand, &round, explain, &hand_data)
}