pub struct Options {
    pub rounds_to_double_weight_rounding_mode: RoundingMode,
    pub same_day_round_ordering: SameDayRoundOrdering,
    pub bad_round_exclusion_average: BadRoundExclusionAverage,
    pub bad_round_exclusion_standard_deviation: BadRoundExclusionStandardDeviation,
    pub final_average_rounding_mode: RoundingMode,
}

pub enum RoundingMode {
    Floor,
    Ceiling,
    MidpointUp,
    MidpointToEven,
}

pub enum SameDayRoundOrdering {
    Chronological,
    Reversed,
}

pub enum BadRoundExclusionAverage {
    Weighted,
    Even,
}

pub enum BadRoundExclusionStandardDeviation {
    Population,
    Sample,
}

pub fn get_all_options() -> Vec<Options> {
    todo!()
}
