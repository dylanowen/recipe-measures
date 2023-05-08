use std::fmt::{Debug, Formatter};

use num_rational::Rational32;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use crate::{Dimension, Unit};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Measure {
    Single(SingleMeasure),
    Multi(Vec<SingleMeasure>),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SingleMeasure {
    pub value: Rational32,
    pub unit: Unit,
}

impl Measure {
    pub fn single(value: Rational32, unit: Unit) -> Measure {
        Measure::Single(SingleMeasure::new(value, unit))
    }

    pub fn multi<Q: Into<Vec<SingleMeasure>>>(quantities: Q) -> Measure {
        Measure::Multi(quantities.into())
    }

    pub(crate) fn from_base(base_value: Rational32, unit: Unit) -> Measure {
        Measure::single(unit.from_base_value(base_value), unit)
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Measure::Single(single) => single.value.is_integer(),
            Measure::Multi(_) => false,
        }
    }

    pub fn dimension(&self) -> Dimension {
        self.main_unit().dimension()
    }

    pub fn main_unit(&self) -> &Unit {
        match self {
            Measure::Single(single) => &single.unit,
            Measure::Multi(multi) => &multi.first().unwrap().unit,
        }
    }
}

impl Debug for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Single(single) => single.fmt(f),
            Measure::Multi(multi) => write!(
                f,
                "{}",
                multi
                    .iter()
                    .map(|q| format!("{q:?}"))
                    .collect::<Vec<_>>()
                    .join(" and ")
            ),
        }
    }
}

impl From<SingleMeasure> for Measure {
    fn from(value: SingleMeasure) -> Self {
        Measure::Single(value)
    }
}

const COMMON_FRACTIONS: [Rational32; 2] = [Rational32::new_raw(1, 8), Rational32::new_raw(1, 3)];
impl SingleMeasure {
    pub fn new(value: Rational32, unit: Unit) -> SingleMeasure {
        SingleMeasure { value, unit }
    }

    pub(crate) fn from_base(base_value: Rational32, unit: Unit) -> SingleMeasure {
        SingleMeasure::new(base_value / unit.multiple(), unit)
    }

    pub(crate) fn base_trunc(&self) -> Rational32 {
        self.value.trunc() * self.unit.multiple()
    }

    pub(crate) fn base_fract(&self) -> Rational32 {
        self.value.fract() * self.unit.multiple()
    }

    pub fn is_good(&self) -> bool {
        self.value != Rational32::zero()
            && (self.value.is_integer() || {
                COMMON_FRACTIONS
                    .iter()
                    .any(|fraction| (self.value / fraction).is_integer())
            })
    }
}

impl Debug for SingleMeasure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value.is_integer() {
            write!(
                f,
                "{} {}",
                self.value.numer(),
                self.unit.description(self.value > Rational32::one())
            )
        } else if self.value > Rational32::one() {
            let fract = self.value.fract();
            write!(
                f,
                "{} {}/{} {}",
                self.value.to_integer(),
                fract.numer(),
                fract.denom(),
                self.unit.description(false)
            )
        } else {
            write!(
                f,
                "{}/{} {}",
                self.value.numer(),
                self.value.denom(),
                self.unit.description(false)
            )
        }
    }
}
