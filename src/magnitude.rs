use num_rational::Rational32;

use crate::{Dimension, Measure, SingleMeasure, Unit};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Magnitude {
    base_value: Rational32,
    dimension: Dimension,
}

impl Magnitude {
    pub fn new<R: Into<Rational32>>(value: R, unit: Unit) -> Magnitude {
        Magnitude {
            base_value: value.into() * unit.multiple(),
            dimension: unit.dimension(),
        }
    }

    // fn unitless(value: Rational32) -> Magnitude {
    //     Magnitude {
    //         base_value: value,
    //         dimension: Dimension::Unitless,
    //     }
    // }

    pub fn measure(self, unit: Unit) -> Measure {
        Measure::from_base(self.base_value, unit)
    }

    pub fn best_measures(self) -> Vec<Measure> {
        // const COMMON_FRACTIONS: [Rational32; 2] =
        //     [Rational32::new_raw(1, 8), Rational32::new_raw(1, 3)];
        let units = self.dimension.units();
        let mut quantities = vec![];
        'unit_loop: for (i, unit) in units.iter().cloned().enumerate() {
            let quantity = SingleMeasure::from_base(self.base_value, unit.clone());

            if quantity.is_good() {
                quantities.push(quantity.into());
            } else {
                for j in (0..i).rev() {
                    let main_quantity =
                        SingleMeasure::from_base(quantity.base_trunc(), unit.clone());
                    let sub_quantity =
                        SingleMeasure::from_base(quantity.base_fract(), units[j].clone());

                    if main_quantity.is_good() && sub_quantity.is_good() {
                        quantities.push(Measure::Multi(vec![main_quantity, sub_quantity]));
                        continue 'unit_loop;
                    }
                }
            }
        }

        if let Some((smallest_common_unit_integer, _)) = quantities
            .iter()
            .enumerate()
            .rev()
            .find(|(_, q)| q.is_integer() && q.main_unit().is_common())
        {
            quantities.drain(0..smallest_common_unit_integer);
        }

        quantities
    }

    pub fn best_measure(self) -> Option<Measure> {
        let mut measures = self.best_measures().into_iter();
        let fallback = measures.next();
        for measure in measures.rev() {
            if measure.main_unit().is_common() {
                return Some(measure);
            }
        }

        fallback
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

    #[test]
    fn test_magnitude_multiples() {
        assert_eq!(
            Magnitude::new(1, Unit::Drop),
            Magnitude::new((1, 96), Unit::Teaspoon)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Smidgen),
            Magnitude::new((1, 32), Unit::Teaspoon)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Pinch),
            Magnitude::new((1, 16), Unit::Teaspoon)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Dash),
            Magnitude::new((1, 8), Unit::Teaspoon)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Teaspoon),
            Magnitude::new((1, 3), Unit::Tablespoon)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Tablespoon),
            Magnitude::new((1, 16), Unit::Cup)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Cup),
            Magnitude::new((1, 2), Unit::Pint)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Pint),
            Magnitude::new((1, 2), Unit::Quart)
        );
        assert_eq!(
            Magnitude::new(1, Unit::Quart),
            Magnitude::new((1, 4), Unit::Gallon)
        );
    }
}
