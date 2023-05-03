use crate::{Unit, TEMPERATURE_UNITS, TIME_UNITS, UNITLESS_UNITS, VOLUME_UNITS};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Dimension {
    Volume,
    Temperature,
    Time,
    Unitless,
}

impl Dimension {
    pub fn units(self) -> &'static [Unit] {
        match self {
            Dimension::Volume => &VOLUME_UNITS,
            Dimension::Temperature => &TEMPERATURE_UNITS,
            Dimension::Time => &TIME_UNITS,
            Dimension::Unitless => &UNITLESS_UNITS,
        }
    }
}
