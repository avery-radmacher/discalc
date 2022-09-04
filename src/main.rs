mod core;
mod options;
mod reader;

fn main() {
    // reader/csv | read file from program args
    // reader     | read into internal format
    // options    | then for each combination of options:
    // self       |     for each valid date in the format:
    // self/core  |         calculate rating
    // self       |         calculate error from projected
    // self       |     aggregate errors
    // self       | order combinations by least error
    // self       | display results
}
