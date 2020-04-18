use regex::Regex;
use rocket::response::status::BadRequest;
use std::num::NonZeroU8;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum ParseError {
    InvalidDiceNumber,
    InvalidDiceSize,
    UnableToParse,
}

impl From<ParseError> for BadRequest<String> {
    fn from(p: ParseError) -> Self {
        match p {
            ParseError::InvalidDiceNumber => BadRequest(Some(String::from("Number of dice must be <= 255"))),
            ParseError::InvalidDiceSize => BadRequest(Some(String::from("Dice size must be 4, 6, 8, 10, 12, 20, or 100"))),
            ParseError::UnableToParse => BadRequest(Some(String::from("Unable to parse, must be of the form <number>d<size>")))
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum DiceSize {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

#[derive(Debug, PartialEq)]
pub(crate) struct RollCmd {
    pub num: NonZeroU8,
    pub size: DiceSize,
}

impl From<DiceSize> for NonZeroU8 {
    fn from(d: DiceSize) -> Self {
        match d {
            DiceSize::D4 => NonZeroU8::new(4).unwrap(),
            DiceSize::D6 => NonZeroU8::new(6).unwrap(),
            DiceSize::D8 => NonZeroU8::new(8).unwrap(),
            DiceSize::D10 => NonZeroU8::new(10).unwrap(),
            DiceSize::D12 => NonZeroU8::new(12).unwrap(),
            DiceSize::D20 => NonZeroU8::new(20).unwrap(),
            DiceSize::D100 => NonZeroU8::new(100).unwrap(),
        }
    }
}

impl From<DiceSize> for usize {
    fn from(d: DiceSize) -> Self {
        match d {
            DiceSize::D4 => 4,
            DiceSize::D6 => 6,
            DiceSize::D8 => 8,
            DiceSize::D10 => 10,
            DiceSize::D12 => 12,
            DiceSize::D20 => 20,
            DiceSize::D100 => 100,
        }
    }
}

impl FromStr for DiceSize {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4" => Ok(DiceSize::D4),
            "6" => Ok(DiceSize::D6),
            "8" => Ok(DiceSize::D8),
            "10" => Ok(DiceSize::D10),
            "12" => Ok(DiceSize::D12),
            "20" => Ok(DiceSize::D20),
            "100" => Ok(DiceSize::D100),
            _ => Err(ParseError::InvalidDiceSize)
        }
    }
}

pub(crate) fn parse_dice_str(dice_str: &str) -> Result<RollCmd, ParseError> {
    // Unwrapping here is fine since we'll know at compile time whether this regular expression compiles.
    let dice_regex = Regex::new(r"^([1-9]\d*)d(\d+)$").unwrap();
    let caps = dice_regex.captures(dice_str).ok_or(ParseError::UnableToParse)?;
    let dice_num = caps.get(1)
        .ok_or(ParseError::InvalidDiceNumber)?
        .as_str().parse::<NonZeroU8>()
        .map_err(|_| {ParseError::InvalidDiceNumber})?;
    let dice_size = caps.get(2)
        .ok_or(ParseError::InvalidDiceSize)?
        .as_str()
        .parse::<DiceSize>()?;
    Ok(RollCmd {
        num: dice_num,
        size: dice_size
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rejects_garbage_string() {
        assert_eq!(parse_dice_str("abcefg"), Err(ParseError::UnableToParse));
    }

    #[test]
    fn rejects_dice_number_starting_with_zero() {
        assert_eq!(parse_dice_str("010d12"), Err(ParseError::UnableToParse));
    }

    #[test]
    fn rejects_dice_number_too_large() {
        assert_eq!(parse_dice_str("256d12"), Err(ParseError::InvalidDiceNumber));
    }

    #[test]
    fn rejects_invalid_dice_size() {
        assert_eq!(parse_dice_str("4d23"), Err(ParseError::InvalidDiceSize));
    }

    #[test]
    fn parses_correct_dice_number() {
        let cmd = parse_dice_str("4d4").unwrap();
        assert_eq!(4, cmd.num.get());
    }

    #[test]
    fn parses_correct_dice_size() {
        let mut cmd = parse_dice_str("4d4").unwrap();
        assert_eq!(DiceSize::D4, cmd.size);
        cmd = parse_dice_str("4d6").unwrap();
        assert_eq!(DiceSize::D6, cmd.size);
        cmd = parse_dice_str("4d8").unwrap();
        assert_eq!(DiceSize::D8, cmd.size);
        cmd = parse_dice_str("4d10").unwrap();
        assert_eq!(DiceSize::D10, cmd.size);
        cmd = parse_dice_str("4d12").unwrap();
        assert_eq!(DiceSize::D12, cmd.size);
        cmd = parse_dice_str("4d20").unwrap();
        assert_eq!(DiceSize::D20, cmd.size);
        cmd = parse_dice_str("4d100").unwrap();
        assert_eq!(DiceSize::D100, cmd.size);
    }
}
