use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub struct Options {
    pub rounds_to_double_weight_rounding_mode: RoundingMode,
    pub same_day_round_ordering: SameDayRoundOrdering,
    pub bad_round_exclusion_average: BadRoundExclusionAverage,
    pub bad_round_exclusion_standard_deviation: BadRoundExclusionStandardDeviation,
    pub final_average_rounding_mode: RoundingMode,
}

#[derive(Clone, Copy, Serialize)]
pub enum RoundingMode {
    Floor,
    Ceiling,
    MidpointUp,
    MidpointToEven,
}

#[derive(Clone, Copy, Serialize)]
pub enum SameDayRoundOrdering {
    Chronological,
    Reversed,
}

#[derive(Clone, Copy, Serialize)]
pub enum BadRoundExclusionAverage {
    Weighted,
    Even,
}

#[derive(Clone, Copy, Serialize)]
pub enum BadRoundExclusionStandardDeviation {
    Population,
    Sample,
}

trait ValuesIter {
    fn iter_values() -> Vec<Self>
    where
        Self: Sized;
}

impl ValuesIter for RoundingMode {
    fn iter_values() -> Vec<Self>
    where
        Self: Sized,
    {
        use RoundingMode::*;
        vec![Floor, Ceiling, MidpointUp, MidpointToEven]
    }
}

impl ValuesIter for SameDayRoundOrdering {
    fn iter_values() -> Vec<Self>
    where
        Self: Sized,
    {
        use SameDayRoundOrdering::*;
        vec![Chronological, Reversed]
    }
}

impl ValuesIter for BadRoundExclusionAverage {
    fn iter_values() -> Vec<Self>
    where
        Self: Sized,
    {
        use BadRoundExclusionAverage::*;
        vec![Even, Weighted]
    }
}

impl ValuesIter for BadRoundExclusionStandardDeviation {
    fn iter_values() -> Vec<Self>
    where
        Self: Sized,
    {
        use BadRoundExclusionStandardDeviation::*;
        vec![Population, Sample]
    }
}

pub fn get_all_options() -> Vec<Options> {
    let mut ret = Vec::with_capacity(128);
    for rounds_to_double_weight_rounding_mode in RoundingMode::iter_values() {
        for same_day_round_ordering in SameDayRoundOrdering::iter_values() {
            for bad_round_exclusion_average in BadRoundExclusionAverage::iter_values() {
                for bad_round_exclusion_standard_deviation in
                    BadRoundExclusionStandardDeviation::iter_values()
                {
                    for final_average_rounding_mode in RoundingMode::iter_values() {
                        ret.push(Options {
                            bad_round_exclusion_average,
                            bad_round_exclusion_standard_deviation,
                            final_average_rounding_mode,
                            rounds_to_double_weight_rounding_mode,
                            same_day_round_ordering,
                        });
                    }
                }
            }
        }
    }
    ret
}
