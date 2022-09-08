mod core;
mod options;
mod reader;

fn main() -> Result<(), reader::Error> {
    // reader & csv   | read file from program args
    // reader & chrono| read into internal format
    let (rounds, dates_with_ratings) = reader::read(std::io::stdin())?;
    // options        | then for each combination of options:
    // self           |     for each valid date in the format:
    // self/core      |         calculate rating
    // self           |         calculate error from projected
    // self           |     aggregate errors
    // self           | order combinations by least error
    // self           | display results
    Ok(())
}
