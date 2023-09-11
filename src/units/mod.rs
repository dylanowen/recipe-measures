pub mod temperature;
pub mod unitless;
pub mod volume;

use crate::volume::VolumeUnit;
use crate::{Dimension, RangeMeasure};
use enum_dispatch::enum_dispatch;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use lazy_static::lazy_static;
use num_rational::Rational32;
use num_traits::One;
use serde::{Deserialize, Serialize};

use crate::temperature::TemperatureUnit;
use crate::unitless::Unitless;

lazy_static! {
    pub static ref UNITFUL_UNITS: Vec<Unit> = vec![&*volume::UNITS, &*temperature::UNITS]
        .into_iter()
        .flatten()
        .cloned()
        .collect();
}

#[enum_dispatch(Unit)]
pub trait UnitLike {
    fn dimension(&self) -> Dimension;

    fn multiple(&self) -> Rational32;

    fn value_to_base(&self, value: Rational32) -> Rational32 {
        value * self.multiple()
    }

    fn value_from_base(&self, base_value: Rational32) -> Rational32 {
        base_value / self.multiple()
    }

    fn abbreviation(&self) -> &str;

    fn description(&self, plural: bool) -> &str;

    fn aliases(&self) -> &'static [&'static str];
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Unit {
    Volume(VolumeUnit),
    Temperature(TemperatureUnit),
    Unitless(Unitless),
}

impl Unit {
    pub fn unitful_units() -> &'static Vec<Unit> {
        &UNITFUL_UNITS
    }
}

impl Into<Dimension> for &dyn UnitLike {
    fn into(self) -> Dimension {
        self.dimension()
    }
}

impl Display for dyn UnitLike {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description(false))
    }
}

pub struct CommonUnit {
    unit: Unit,
    fractions: Vec<Rational32>,
}

impl CommonUnit {
    pub fn new<U, F>(unit: U, fractions: F) -> Self
    where
        U: Into<Unit>,
        F: Into<Vec<Rational32>>,
    {
        Self {
            unit: unit.into(),
            fractions: fractions.into(),
        }
    }
}
