use crate::options::Options;
use chrono::NaiveDate;

#[derive(Clone, Copy)]
pub struct Round {
    pub date: NaiveDate,
    pub round_number: u32,
    pub holes: u32,
    pub rating: u32,
}

pub fn calculate_rating(_rounds: Vec<Round>, _options: Options, _date: Option<NaiveDate>) -> u32 {
    1000 // TODO
}

pub fn calculate_rating_on(rounds: Vec<Round>, options: Options, date: NaiveDate) -> u32 {
    calculate_rating(rounds, options, Some(date))
}

pub struct OptionsWithError {
    pub options: Options,
    pub error: f64,
}
