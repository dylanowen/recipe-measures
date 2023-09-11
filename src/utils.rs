use casey;
use enum_iterator::{All, Sequence};

pub trait SelfSequence: Sized {
    fn all() -> All<Self>;
}

impl<S> SelfSequence for S
where
    S: Sequence,
{
    #[inline]
    fn all() -> All<Self> {
        enum_iterator::all()
    }
}

#[macro_export]
macro_rules! unit_macro {
    [$unit:ident, $dimension:ident: $((
        $r_unit:ident:$r_unit_mod:ident @ $multiple:expr ;
        $description_plural:literal,
        $abbreviation:literal
        $(,$additional_aliases:literal)*
    ),)*] => {
        #[derive(Sequence, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
        pub enum $unit {
            $($r_unit,)*
        }

        $crate::unit_modules![
            $(($r_unit_mod @ $multiple; $description_plural, $abbreviation $(,$additional_aliases)*),)*
        ];

        impl $crate::UnitLike for $unit {
            fn dimension(&self) -> $crate::Dimension {
                $dimension.into()
            }

            fn aliases(&self) -> &'static [&'static str] {
                $crate::unit_match!(self, $unit @( $(($r_unit => &$r_unit_mod::ALIASES))* ) [] [] )
            }

            fn abbreviation(&self) -> &'static str {
               $crate::unit_match!(self, $unit @( $(($r_unit => $r_unit_mod::ABBREVIATION))* ) [] [] )
            }

            fn description(&self, plural: bool) -> &'static str {
                $crate::unit_match!(self, $unit @( $(($r_unit => {
                    if plural {
                        &$r_unit_mod::DESCRIPTION_PLURAL
                    } else {
                        &$r_unit_mod::DESCRIPTION
                    }
                }))* ) [] [] )
            }

            fn multiple(&self) -> num_rational::Rational32 {
               $crate::unit_match!(self, $unit @( $(($r_unit => $r_unit_mod::MULTIPLE))* ) [] [] )
            }
        }
    }
}

#[macro_export]
macro_rules! unit_modules {
    [
        (
            $unit:ident @ $multiple:expr ;
            $description_plural:literal,
            $abbreviation:literal
            $(,$additional_aliases:literal)*
        ),
    ] => {
        pub mod $unit {
            use lazy_static::lazy_static;
            use num_rational::Rational32;

            pub const MULTIPLE: Rational32 = $multiple;
            pub const DESCRIPTION: &str = stringify!($unit);
            pub const DESCRIPTION_PLURAL: &str = $description_plural;
            pub const ABBREVIATION: &str = $abbreviation;

            lazy_static! {
                pub static ref ALIASES: Vec<&'static str> = vec![
                    DESCRIPTION,
                    DESCRIPTION_PLURAL,
                    $($additional_aliases,)*
                    ABBREVIATION,
                ];
            }
        }
    };
    [
        (
            $unit:ident @ $multiple:expr ;
            $description_plural:literal,
            $abbreviation:literal
            $(,$additional_aliases:literal)*
        ),
        $((
            $unit_tail:ident @ $multiple_tail:expr ;
            $description_plural_tail:literal,
            $abbreviation_tail:literal
            $(,$additional_aliases_tail:literal)*
        ),)*
    ] => {
        $crate::unit_modules![
          ($unit @ $multiple; $description_plural, $abbreviation $(,$additional_aliases)*),
        ];
        $crate::unit_modules![
          $(($unit_tail @ $multiple_tail; $description_plural_tail, $abbreviation_tail $(,$additional_aliases_tail)*),)*
        ];
    };
}

#[macro_export]
macro_rules! unit_match {
    ($target:ident, $unit:ident @(
            ($r_unit:ident => $value_expression:expr)
        )
        [$($indexes:pat,)*] [$($arms:expr,)*]
    ) => {
        match $target {
            $unit::$r_unit => $value_expression,
            $($indexes => $arms,)*
        }
    };
    ($target:ident, $unit:ident @(
            ($r_unit:ident => $value_expression:expr)
            $(($r_unit_tail:ident => $value_expression_tail:expr))*
        )
        [$($indexes:pat,)*] [$($arms:expr,)*]
    ) => {
        $crate::unit_match!($target, $unit @(
                $(($r_unit_tail => $value_expression_tail))*
            )
           [$unit::$r_unit, $($indexes,)*] [$value_expression, $($arms,)*]
        );
    };
}
