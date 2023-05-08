use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use lazy_static::lazy_static;
use num_rational::Rational32;
use num_traits::One;
use serde::{Deserialize, Serialize};

use crate::Dimension;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Unit {
    // Volume
    Drop,
    Smidgen,
    Pinch,
    Dash,
    Teaspoon,
    Tablespoon,
    Cup,
    Pint,
    Quart,
    Gallon,
    // Temp
    Fahrenheit,
    Celsius,
    // Time
    Second,
    Minute,
    Hour,
    //
    Unitless { unit: String },
}

const VOLUME_UNITS_COUNT: usize = 10;
pub static VOLUME_UNITS: [Unit; VOLUME_UNITS_COUNT] = [
    Unit::Drop,
    Unit::Smidgen,
    Unit::Pinch,
    Unit::Dash,
    Unit::Teaspoon,
    Unit::Tablespoon,
    Unit::Cup,
    Unit::Pint,
    Unit::Quart,
    Unit::Gallon,
];
const TEMPERATURE_UNITS_COUNT: usize = 2;
pub static TEMPERATURE_UNITS: [Unit; TEMPERATURE_UNITS_COUNT] = [Unit::Fahrenheit, Unit::Celsius];
const TIME_UNITS_COUNT: usize = 3;
pub static TIME_UNITS: [Unit; TIME_UNITS_COUNT] = [Unit::Second, Unit::Minute, Unit::Hour];
pub static UNITLESS_UNITS: [Unit; 1] = [Unit::unitless(String::new())];

lazy_static! {
    pub static ref UNITFUL_UNITS: [Unit; VOLUME_UNITS_COUNT + TEMPERATURE_UNITS_COUNT + TIME_UNITS_COUNT] = {
        let mut unitful: [Unit; VOLUME_UNITS_COUNT + TEMPERATURE_UNITS_COUNT + TIME_UNITS_COUNT] =
            Default::default();
        let (volume, remainder) = unitful.split_at_mut(VOLUME_UNITS_COUNT);
        let (temperature, time) = remainder.split_at_mut(TEMPERATURE_UNITS_COUNT);
        volume.clone_from_slice(&VOLUME_UNITS);
        temperature.clone_from_slice(&TEMPERATURE_UNITS);
        time.clone_from_slice(&TIME_UNITS);

        unitful
    };
}

impl Unit {
    pub fn aliases(&self) -> &[&str] {
        match self {
            // Volume
            Unit::Drop => &["drop", "drops", "gt", "gtt", "dr"],
            Unit::Smidgen => &["smidgen", "smidgens", "smi", "smdg"],
            Unit::Pinch => &["pinch", "pinches", "pn"],
            Unit::Dash => &["dash", "dashes", "ds"],
            Unit::Teaspoon => &["teaspoon", "teaspoons", "t", "tsp"],
            Unit::Tablespoon => &["tablespoon", "tablespoons", "Tb", "T", "tbsp"],
            Unit::Cup => &["cup", "cups", "c", "C"],
            Unit::Pint => &["pint", "pints", "pt"],
            Unit::Quart => &["quart", "quarts", "qt"],
            Unit::Gallon => &["gallon", "gallons", "gal"],
            //  Temp
            Unit::Fahrenheit => &[
                "fahrenheit",
                "fahrenheit",
                "degrees", // We're targeting the US so prioritize Fahrenheit
                "°F",
                "F",
            ],
            Unit::Celsius => &["celsius", "celsius", "°C", "C"],
            // Time
            Unit::Second => &["second", "seconds", "sec"],
            Unit::Minute => &["minute", "minutes", "min"],
            Unit::Hour => &["hour", "hours"],
            //
            Unit::Unitless { .. } => &[""],
        }
    }

    pub fn abbreviation(&self) -> &str {
        self.aliases().last().unwrap()
    }

    pub fn description(&self, plural: bool) -> &str {
        if plural {
            // TODO this is a gross hack
            self.aliases().get(1).unwrap_or(&"")
        } else {
            self.aliases().first().unwrap()
        }
    }

    pub fn is_common(&self) -> bool {
        matches!(self, Unit::Teaspoon | Unit::Tablespoon | Unit::Cup)
    }

    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_base_value(&self, base_value: Rational32) -> Rational32 {
        base_value / self.multiple()
    }

    pub(crate) fn multiple(&self) -> Rational32 {
        match self {
            // Volume
            Unit::Drop => Rational32::one(),
            Unit::Smidgen => Rational32::from_integer(3),
            Unit::Pinch => Rational32::from_integer(6),
            Unit::Dash => Rational32::from_integer(12),
            Unit::Teaspoon => Rational32::from_integer(96),
            Unit::Tablespoon => Rational32::from_integer(288),
            Unit::Cup => Rational32::from_integer(4_608),
            Unit::Pint => Rational32::from_integer(9_216),
            Unit::Quart => Rational32::from_integer(18_432),
            Unit::Gallon => Rational32::from_integer(73_728),
            // Temp
            Unit::Fahrenheit | Unit::Celsius => Rational32::one(),
            // Time
            Unit::Second => Rational32::one(),
            Unit::Minute => Rational32::from_integer(60),
            Unit::Hour => Rational32::from_integer(60 * 60),
            //
            Unit::Unitless { .. } => Rational32::one(),
        }
    }

    pub fn dimension(&self) -> Dimension {
        match self {
            Unit::Drop
            | Unit::Smidgen
            | Unit::Pinch
            | Unit::Dash
            | Unit::Teaspoon
            | Unit::Tablespoon
            | Unit::Cup
            | Unit::Pint
            | Unit::Quart
            | Unit::Gallon => Dimension::Volume,
            Unit::Second | Unit::Minute | Unit::Hour => Dimension::Time,
            Unit::Fahrenheit | Unit::Celsius => Dimension::Temperature,
            Unit::Unitless { .. } => Dimension::Unitless,
        }
    }

    pub const fn unitless(unit: String) -> Unit {
        Unit::Unitless { unit }
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit::unitless("".to_string())
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description(true))
    }
}
