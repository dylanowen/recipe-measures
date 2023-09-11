use enum_dispatch::enum_dispatch;
use num_rational::Rational32;
use std::ops;

use crate::dimension::DimensionLike;
use crate::measure::MeasureLike;
use crate::{Dimension, Measure, MultiMeasure, SingleMeasure, Unit};
use crate::{RangeMeasure, UnitLike};

#[enum_dispatch(Magnitude)]
pub trait MagnitudeLike {
    // todo playing around with magnitude like, maybe do a dispatch
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Magnitude {
    Single {
        base_value: Rational32,
        dimension: Dimension,
    },
    Range {
        from_base_value: Rational32,
        to_base_value: Rational32,
        dimension: Dimension,
    },
}

pub struct SingleMagnitude {}

impl Magnitude {
    pub fn single<R: Into<Rational32>, D: Into<Dimension>>(base_value: R, dimension: D) -> Self {
        Self::Single {
            base_value: base_value.into(),
            dimension: dimension.into(),
        }
    }

    pub fn range<R: Into<Rational32>, D: Into<Dimension>>(
        from_base_value: R,
        to_base_value: R,
        dimension: D,
    ) -> Self {
        Self::Range {
            from_base_value: from_base_value.into(),
            to_base_value: to_base_value.into(),
            dimension: dimension.into(),
        }
    }

    // fn unitless(value: Rational32) -> Magnitude {
    //     Magnitude {
    //         base_value: value,
    //         dimension: Dimension::Unitless,
    //     }
    // }

    pub fn measure(&self, unit: Unit) -> Measure {
        match self {
            Magnitude::Single { base_value, .. } => {
                SingleMeasure::from_base(*base_value, unit).into()
            }
            Magnitude::Range {
                from_base_value,
                to_base_value,
                ..
            } => {
                RangeMeasure::from_base(*from_base_value, unit.clone(), *to_base_value, unit).into()
            }
        }
    }

    pub fn best_measures(&self) -> Vec<Measure> {
        match self {
            Magnitude::Single {
                base_value,
                dimension,
            } => {
                // for unit in dimension.units() {
                //     let measure = self.measure(unit.clone());
                //
                //     if measure.v
                //
                // }

                todo!()
            }
            Magnitude::Range { .. } => {
                vec![]
            }
        }

        // const COMMON_FRACTIONS: [Rational32; 2] =
        //     [Rational32::new_raw(1, 8), Rational32::new_raw(1, 3)];
        // let units = self.dimension.units();
        // let mut quantities = vec![];
        // 'unit_loop: for (i, unit) in units.iter().cloned().enumerate() {
        //     let quantity = SingleMeasure::from_base(self.base_value, unit.clone());
        //
        //     if quantity.is_good() {
        //         quantities.push(quantity.into());
        //     } else {
        //         for j in (0..i).rev() {
        //             let main_quantity =
        //                 SingleMeasure::from_base(quantity.base_trunc(), unit.clone());
        //             let sub_quantity =
        //                 SingleMeasure::from_base(quantity.base_fract(), units[j].clone());
        //
        //             // TODO
        //             // if main_quantity.is_good() && sub_quantity.is_good() {
        //             //     quantities.push(Measure::Multi(vec![main_quantity, sub_quantity]));
        //             //     continue 'unit_loop;
        //             // }
        //         }
        //     }
        // }
        //
        // // TODO
        // // if let Some((smallest_common_unit_integer, _)) = quantities
        // //     .iter()
        // //     .enumerate()
        // //     .rev()
        // //     .find(|(_, q)| q.is_integer() && q.main_unit().is_common())
        // // {
        // //     quantities.drain(0..smallest_common_unit_integer);
        // // }
        //
        // quantities
        // todo!()
    }

    pub fn best_measure(self) -> Option<Measure> {
        let mut measures = self.best_measures().into_iter();
        let fallback = measures.next();
        // TODO
        // for measure in measures.rev() {
        //     if measure.main_unit().is_common() {
        //         return Some(measure);
        //     }
        // }

        fallback
    }
}

impl ops::Mul<Rational32> for Magnitude {
    type Output = Magnitude;

    fn mul(self, multiple: Rational32) -> Magnitude {
        match self {
            Magnitude::Single {
                base_value,
                dimension,
            } => Self::Single {
                base_value: base_value * multiple,
                dimension,
            },
            Magnitude::Range {
                from_base_value,
                to_base_value,
                dimension,
            } => Self::Range {
                from_base_value: from_base_value * multiple,
                to_base_value: to_base_value * multiple,
                dimension,
            },
        }
    }
}

impl From<Measure> for Magnitude {
    fn from(measure: Measure) -> Self {
        measure.magnitude()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best_quantities() {
        // for i in 0..400 {
        //     println!(
        //         "{:?} {:?}",
        //         Magnitude::new(i, Unit::Teaspoon).best_measure(),
        //         Magnitude::new(i, Unit::Teaspoon).best_measures()
        //     );
        // }

        // assert_eq!(
        //     Magnitude::new(384, Unit::Tablespoon).best_measures(),
        //     vec![]
        // )
    }
}
