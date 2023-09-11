use crate::temperature::TemperatureDimension;
use crate::volume::VolumeDimension;

use crate::unitless::UnitlessDimension;
use crate::Unit;
use crate::{CommonUnit, UnitLike};
use enum_dispatch::enum_dispatch;
use std::fmt::Debug;

#[enum_dispatch(Dimension)]
pub trait DimensionLike {
    fn units(&self) -> &'static [Unit];

    fn common_units(&self) -> &'static [CommonUnit];
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
#[enum_dispatch]
pub enum Dimension {
    Volume(VolumeDimension),
    Temperature(TemperatureDimension),
    Unitless(UnitlessDimension),
}

impl<U: UnitLike> From<U> for Dimension {
    fn from(value: U) -> Self {
        value.dimension()
    }
}
