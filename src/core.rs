use crate::options::Options;
use chrono::NaiveDate;

pub struct Round {
    pub date: NaiveDate,
    pub round_number: u32,
    pub holes: u32,
    pub rating: u32,
}

pub fn calculate_rating(rounds: Vec<Round>, options: Options, date: Option<NaiveDate>) -> u32 {
    todo!()
}

pub fn calculate_rating_on(rounds: Vec<Round>, options: Options, date: NaiveDate) -> u32 {
    calculate_rating(rounds, options, Some(date))
}

pub fn calculate_rating_latest(rounds: Vec<Round>, options: Options) -> u32 {
    calculate_rating(rounds, options, None)
}
