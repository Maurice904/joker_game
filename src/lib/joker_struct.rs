use ortalib::Joker;

#[derive(Default)]
pub struct JokerGroupData {
    pub hands_jokers: Vec<Joker>, 
    pub on_scored_jokers: Vec<Joker>, 
    pub on_scored_retriggers: Vec<Joker>, 
    pub on_hold_jokers: Vec<Joker>, 
    pub on_hold_retriggers: Vec<Joker>, 
    pub inde_jokers: Vec<Joker>,
    pub joker_map: u64
}

impl JokerGroupData {
    
    pub fn contains(&self, joker_key: u64) -> bool {
        (self.joker_map & joker_key) != 0
    }

}