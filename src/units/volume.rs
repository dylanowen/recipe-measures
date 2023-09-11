use enum_iterator::Sequence;
use lazy_static::lazy_static;
use num_rational::Rational32;
use num_traits::One;
use serde::{Deserialize, Serialize};

use crate::utils::SelfSequence;
use crate::{unit_macro, CommonUnit, DimensionLike, Unit, UnitLike};

lazy_static! {
    pub static ref UNITS: Vec<Unit> = VolumeUnit::all().map(Into::into).collect();
    pub static ref COMMON_UNITS: Vec<CommonUnit> = vec![
        CommonUnit::new(VolumeUnit::Teaspoon, vec![Rational32::new_raw(1, 8)]),
        CommonUnit::new(VolumeUnit::Tablespoon, vec![Rational32::new_raw(1, 2)]),
        CommonUnit::new(
            VolumeUnit::Cup,
            vec![Rational32::new_raw(1, 4), Rational32::new_raw(1, 3)]
        ),
    ];
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct VolumeDimension;

impl DimensionLike for VolumeDimension {
    fn units(&self) -> &'static [Unit] {
        &UNITS
    }

    fn common_units(&self) -> &'static [CommonUnit] {
        &COMMON_UNITS
    }
}

unit_macro![VolumeUnit, VolumeDimension:
    (Drop:drop              @ Rational32::new_raw(1, 1); "drops", "dr", "gt", "gtt"),
    (Smidgen:smidgen        @ Rational32::new_raw(3, 1); "smidgens", "smdg", "smi"),
    (Pinch:pinch            @ Rational32::new_raw(6, 1); "pinches", "pn"),
    (Dash:dash              @ Rational32::new_raw(12, 1); "dashes", "ds"),
    (Teaspoon:teaspoon      @ Rational32::new_raw(96, 1); "teaspoons", "tsp", "t"),
    (Tablespoon:tablespoon  @ Rational32::new_raw(288, 1); "tablespoons", "tbsp", "Tb", "T"),
    (Cup:cup                @ Rational32::new_raw(4_608, 1); "cups", "C", "c"),
    (Pint:pint              @ Rational32::new_raw(9_216, 1); "pints", "pt"),
    (Quart:quart            @ Rational32::new_raw(18_432, 1); "quarts", "qt"),
    (Gallon:gallon          @ Rational32::new_raw(73_728, 1); "gallons", "gal"),
];

#[cfg(test)]
mod test {
    use crate::{Measure, MeasureLike};

    use super::*;

    #[test]
    fn test_magnitude_multiples() {
        assert_eq!(
            Measure::single(1, VolumeUnit::Drop).magnitude(),
            Measure::single((1, 96), VolumeUnit::Teaspoon).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Smidgen).magnitude(),
            Measure::single((1, 32), VolumeUnit::Teaspoon).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Pinch).magnitude(),
            Measure::single((1, 16), VolumeUnit::Teaspoon).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Dash).magnitude(),
            Measure::single((1, 8), VolumeUnit::Teaspoon).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Teaspoon).magnitude(),
            Measure::single((1, 3), VolumeUnit::Tablespoon).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Tablespoon).magnitude(),
            Measure::single((1, 16), VolumeUnit::Cup).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Cup).magnitude(),
            Measure::single((1, 2), VolumeUnit::Pint).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Pint).magnitude(),
            Measure::single((1, 2), VolumeUnit::Quart).magnitude()
        );
        assert_eq!(
            Measure::single(1, VolumeUnit::Quart).magnitude(),
            Measure::single((1, 4), VolumeUnit::Gallon).magnitude()
        );
    }
}
