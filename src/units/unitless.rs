use crate::{CommonUnit, Dimension, DimensionLike, Unit, UnitLike};
use lazy_static::lazy_static;
use num_rational::Rational32;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref UNITS: Vec<Unit> = vec![];
    pub static ref COMMON_UNITS: Vec<CommonUnit> = vec![];
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct UnitlessDimension;

impl DimensionLike for UnitlessDimension {
    fn units(&self) -> &'static [Unit] {
        &UNITS
    }

    fn common_units(&self) -> &'static [CommonUnit] {
        &COMMON_UNITS
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Unitless {
    unit: String,
}

impl Unitless {
    pub fn new<S: ToString>(unit: S) -> Self {
        Self {
            unit: unit.to_string(),
        }
    }
}

impl UnitLike for Unitless {
    fn dimension(&self) -> Dimension {
        UnitlessDimension.into()
    }

    fn multiple(&self) -> Rational32 {
        Rational32::new_raw(1, 1)
    }

    fn abbreviation(&self) -> &str {
        &self.unit
    }

    fn description(&self, _plural: bool) -> &str {
        &self.unit
    }

    fn aliases(&self) -> &'static [&'static str] {
        &[]
    }
}
