use ortalib::{Chips, Mult, Enhancement, Edition};

pub fn get_enhancement(mut chips: Chips, mut mult: Mult, enhancement_type: Option<Enhancement>, on_score: bool, explain: bool) -> (Chips, Mult) {
    if let Some(enhancement) = enhancement_type {
        match enhancement {
            Enhancement::Bonus => {
                if on_score {
                    chips += 30.0;
                    if explain {
                        println!("+30 chips ({} x {})", chips, mult);
                    }
                }
            }
            Enhancement::Steel => {
                if !on_score {
                    mult *= 1.5;
                    if explain {
                        println!("x1.5 mult ({} x {})", chips, mult);
                    }
                }
            }
            Enhancement::Mult => {
                if on_score {
                    mult += 4.0;
                    if explain {
                        println!("+4 mult ({} x {})", chips, mult);
                    }
                }
            }
            Enhancement::Glass => {
                if on_score {
                    mult *= 2.0;
                    if explain {
                        println!("x2 mult ({} x {})", chips, mult);
                    }
                }
            }
            _ => {}
        }
    }
    (chips, mult)
}

pub fn get_edition(mut chips: Chips, mut mult: Mult, edition_type: Option<Edition>, explain : bool) -> (Chips, Mult) {
    if let Some(edition) = edition_type {
        match edition {
            Edition::Foil => {
                chips += 50.0;
                if explain {
                    println!("+50 chips ({} x {})", chips, mult);
                }
                
            }
            Edition::Holographic => {
                mult += 10.0;
                if explain {
                    println!("+10 mult ({} x {})", chips, mult);
                }
                
            }
            Edition::Polychrome => {
                mult *= 1.5;
                if explain {
                    println!("x1.5 mult ({} x {})", chips, mult);
                }
            }
        }
    }
    (chips, mult)
}