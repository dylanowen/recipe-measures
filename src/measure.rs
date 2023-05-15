use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ptr::write;

use num_rational::Rational32;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use crate::{Dimension, Magnitude, Unit};

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

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Measure::Single(measure) => Display::fmt(measure, f),
            Measure::Multi(multi) => {
                let first = true;
                for (i, measure) in multi.iter().enumerate() {
                    if !first && i < multi.len() - 1 {
                        write!(f, " and ")?;
                    }
                    Display::fmt(measure, f)?;
                }
                Ok(())
            }
        }
    }
}

impl Debug for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#}")
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

    #[inline]
    pub(crate) fn base(&self) -> Rational32 {
        self.value * self.unit.multiple()
    }

    pub(crate) fn base_trunc(&self) -> Rational32 {
        self.base().trunc()
    }

    pub(crate) fn base_fract(&self) -> Rational32 {
        self.base().fract()
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

impl Display for SingleMeasure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let unit_text = if !f.alternate() {
            self.unit.abbreviation()
        } else {
            self.unit.description(self.value > Rational32::one())
        };
        if self.value.is_integer() {
            write!(f, "{} {unit_text}", self.value.numer(),)
        } else if self.value > Rational32::one() {
            let fract = self.value.fract();
            write!(
                f,
                "{} {}/{} {unit_text}",
                self.value.to_integer(),
                fract.numer(),
                fract.denom(),
            )
        } else {
            write!(
                f,
                "{}/{} {unit_text}",
                self.value.numer(),
                self.value.denom()
            )
        }
    }
}

impl Debug for SingleMeasure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#}")
    }
}
