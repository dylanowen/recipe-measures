use enum_iterator::Sequence;
use lazy_static::lazy_static;
use num_traits::One;
use serde::{Deserialize, Serialize};

use crate::utils::SelfSequence;
use crate::{unit_macro, CommonUnit, DimensionLike, Unit, UnitLike};

lazy_static! {
    pub static ref UNITS: Vec<Unit> = TemperatureUnit::all().map(Into::into).collect();
    pub static ref COMMON_UNITS: Vec<CommonUnit> = vec![];
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct TemperatureDimension;

impl DimensionLike for TemperatureDimension {
    fn units(&self) -> &'static [Unit] {
        &UNITS
    }

    fn common_units(&self) -> &'static [CommonUnit] {
        &COMMON_UNITS
    }
}

unit_macro![TemperatureUnit, TemperatureDimension:
    // We're targeting the US so prioritize Fahrenheit
    (Fahrenheit:fahrenheit  @ Rational32::new_raw(1, 1); "fahrenheit", "F", "degrees", "°F"),
    (Celsius:celsius        @ Rational32::new_raw(1, 1); "celsius", "C", "°C"),
];
