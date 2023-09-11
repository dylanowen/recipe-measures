use enum_dispatch::enum_dispatch;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Mul, Range};
use std::ptr::write;
use std::{fmt, ops};

use num_rational::Rational32;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use crate::UnitLike;
use crate::{Dimension, Magnitude, Unit};

#[enum_dispatch(Measure)]
pub trait MeasureLike {
    fn dimension(&self) -> Dimension;
    fn magnitude(&self) -> Magnitude;
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum Measure {
    Single(SingleMeasure),
    Multi(MultiMeasure),
    Range(RangeMeasure),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SingleMeasure {
    pub value: Rational32,
    pub unit: Unit,
}

const COMMON_FRACTIONS: [Rational32; 2] = [Rational32::new_raw(1, 8), Rational32::new_raw(1, 3)];
impl SingleMeasure {
    pub fn new<R: Into<Rational32>, U: Into<Unit>>(value: R, unit: U) -> Self {
        Self {
            value: value.into(),
            unit: unit.into(),
        }
    }

    pub(crate) fn from_base(base_value: Rational32, unit: Unit) -> SingleMeasure {
        SingleMeasure::new(unit.value_from_base(base_value), unit)
    }

    pub(crate) fn base_value(&self) -> Rational32 {
        self.unit.value_to_base(self.value)
    }

    // #[inline]
    // pub(crate) fn base(&self) -> Rational32 {
    //     self.value * self.unit.multiple()
    // }
    //
    // pub(crate) fn base_trunc(&self) -> Rational32 {
    //     self.base().trunc()
    // }
    //
    // pub(crate) fn base_fract(&self) -> Rational32 {
    //     self.base().fract()
    // }

    pub fn is_good(&self) -> bool {
        self.value != Rational32::zero()
            && (self.value.is_integer() || {
                COMMON_FRACTIONS
                    .iter()
                    .any(|fraction| (self.value / fraction).is_integer())
            })
    }
}

impl MeasureLike for SingleMeasure {
    fn dimension(&self) -> Dimension {
        self.unit.dimension()
    }

    fn magnitude(&self) -> Magnitude {
        Magnitude::single(self.base_value(), self.dimension())
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct MultiMeasure {
    pub measures: Vec<SingleMeasure>,
}

impl MultiMeasure {
    pub(crate) fn base_value(&self) -> Rational32 {
        self.measures
            .iter()
            .map(SingleMeasure::base_value)
            .fold(Rational32::zero(), ops::Add::add)
    }
}

impl MeasureLike for MultiMeasure {
    fn dimension(&self) -> Dimension {
        self.measures
            .first()
            .expect("Found empty MultiMeasure")
            .dimension()
    }

    fn magnitude(&self) -> Magnitude {
        Magnitude::single(self.base_value(), self.dimension())
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct RangeMeasure {
    pub from: SingleMeasure,
    pub to: SingleMeasure,
}

impl RangeMeasure {
    pub fn new<RF, RT, U>(from_value: RF, from_unit: U, to_value: RT, to_unit: U) -> Self
    where
        RF: Into<Rational32>,
        RT: Into<Rational32>,
        U: Into<Unit>,
    {
        Self {
            from: SingleMeasure::new(from_value, from_unit),
            to: SingleMeasure::new(to_value, to_unit),
        }
    }

    pub(crate) fn from_base<RF, RT, U>(
        from_base_value: RF,
        from_unit: U,
        to_base_value: RT,
        to_unit: U,
    ) -> Self
    where
        RF: Into<Rational32>,
        RT: Into<Rational32>,
        U: Into<Unit>,
    {
        let from_unit = from_unit.into();
        let to_unit = to_unit.into();
        Self::new(
            from_unit.value_from_base(from_base_value.into()),
            from_unit,
            to_unit.value_from_base(to_base_value.into()),
            to_unit,
        )
    }
}

impl MeasureLike for RangeMeasure {
    fn dimension(&self) -> Dimension {
        self.from.dimension()
    }

    fn magnitude(&self) -> Magnitude {
        Magnitude::range(
            self.from.base_value(),
            self.to.base_value(),
            self.dimension(),
        )
    }
}

impl Measure {
    pub fn single<R: Into<Rational32>, U: Into<Unit>>(value: R, unit: U) -> Self {
        Measure::Single(SingleMeasure::new(value, unit))
    }
    //
    // pub fn multi<Q: Into<MultiMeasure>>(quantities: Q) -> Self {
    //     Measure::Multi(quantities.into())
    // }

    // pub fn is_integer(&self) -> bool {
    //     match self {
    //         Measure::Single(single) => single.value.is_integer(),
    //         Measure::Multi(_) => false,
    //     }
    // }

    // pub fn dimension(&self) -> Dimension {
    //     self.main_unit().dimension()
    // }

    // pub fn main_unit(&self) -> &Unit {
    //     match self {
    //         Measure::Single(single) => &single.unit,
    //         Measure::Multi(multi) => &multi.first().unwrap().unit,
    //     }
    // }
}

impl Display for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // match self {
        //     Measure::Single(measure) => Display::fmt(measure, f),
        //     Measure::Multi(multi) => {
        //         let first = true;
        //         for (i, measure) in multi.iter().enumerate() {
        //             if !first && i < multi.len() - 1 {
        //                 write!(f, " and ")?;
        //             }
        //             Display::fmt(measure, f)?;
        //         }
        //         Ok(())
        //     }
        // }
        todo!()
    }
}

impl Into<Magnitude> for &Measure {
    fn into(self) -> Magnitude {
        self.magnitude()
    }
}

impl Debug for Measure {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#}")
    }
}
