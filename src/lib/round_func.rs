use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_yaml;
use ortalib::Round;

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Round, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let round: Round = serde_yaml::from_str(&contents)?;
    Ok(round)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_read_valid_round() {
        let path = PathBuf::from("tests/flush.yml");
        let result = read_from_file(path);
        assert!(result.is_ok());
        
        let round = result.unwrap();
        dbg!(&round);
        assert!(!round.cards_played.is_empty());
    }

    #[test]
    fn test_read_nonexistent_file() {
        let path = PathBuf::from("nonexistent.yml");
        let result = read_from_file(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_invalid_yaml() {
        let path = PathBuf::from("tests/fixtures/invalid_round.yml");
        let result = read_from_file(path);
        assert!(result.is_err());
    }
}