use crate::core::{calculate_rating_on, OptionsWithError};
use options::get_all_options;

mod core;
mod io_handler;
mod options;

fn main() -> Result<(), io_handler::Error> {
    let (rounds, dates_with_ratings) = io_handler::read(std::io::stdin())?;
    let mut results = get_all_options()
        .into_iter()
        .map(|options| {
            let unscaled_error: u32 = dates_with_ratings
                .iter()
                .map(|(date, rating)| {
                    let projected_rating = calculate_rating_on(rounds.clone(), options, *date);
                    (projected_rating - rating).pow(2)
                })
                .sum();
            OptionsWithError {
                options,
                error: unscaled_error as f64 / rounds.len() as f64,
            }
        })
        .collect::<Vec<_>>();
    results.sort_unstable_by(|a, b| a.error.partial_cmp(&b.error).unwrap());
    io_handler::write(std::io::stdout(), results)?;
    Ok(())
}
