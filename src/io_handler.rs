use crate::core::{OptionsWithError, Round};
use crate::options::Options;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    CsvError,
    DataError,
}
use Error::*;

pub fn read<R>(reader: R) -> Result<(Vec<Round>, Vec<(NaiveDate, u32)>), Error>
where
    R: std::io::Read,
{
    let mut reader = csv::Reader::from_reader(reader);
    let records = reader
        .deserialize()
        .map(to_data)
        .collect::<Result<Vec<_>, _>>()?;
    let (rounds, dated_ratings): (Vec<_>, Vec<_>) = records.into_iter().unzip();
    let dated_ratings = dated_ratings
        .into_iter()
        .filter_map(|i| i)
        .collect::<Vec<_>>();
    Ok((rounds, dated_ratings))
}

#[derive(Deserialize)]
struct Record {
    pub date: String,
    pub round_number: u32,
    pub holes: u32,
    pub rating: u32,
    pub player_rating: Option<u32>,
}

fn to_data(result: Result<Record, csv::Error>) -> Result<(Round, Option<(NaiveDate, u32)>), Error> {
    let record: Record = result.or(Err(CsvError))?;
    let date = NaiveDate::parse_from_str(&record.date, "%Y-%m-%d").or(Err(DataError))?;
    let round = Round {
        date,
        holes: record.holes,
        rating: record.rating,
        round_number: record.round_number,
    };
    let dated_rating = record.player_rating.map(|rating| (date, rating));
    Ok((round, dated_rating))
}

#[derive(Serialize)]
struct OutRecord {
    options: Options,
    error: f64,
}

pub fn write<W>(writer: W, data: Vec<OptionsWithError>) -> Result<(), Error>
where
    W: std::io::Write,
{
    let records = data
        .iter()
        .map(|&OptionsWithError { options, error }| OutRecord { options, error })
        .collect::<Vec<_>>();
    let mut writer = csv::Writer::from_writer(writer);
    for record in records {
        writer.serialize(record).or(Err(CsvError))?;
    }
    Ok(())
}