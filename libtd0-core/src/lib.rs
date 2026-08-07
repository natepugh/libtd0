use fastnum::D64 as Decimal;
use zerocopy::{I16, U16};
pub mod result;

pub fn in_range_inclusive<T>(val: &T, min: Option<&T>, max: Option<&T>) -> bool
where
    T: Ord + HasMinAndMax,
{
    (min.unwrap_or(&T::MIN)..=max.unwrap_or(&T::MAX)).contains(&val)
}

pub trait HasMinAndMax {
    const MIN: Self;
    const MAX: Self;
}

impl HasMinAndMax for Decimal {
    const MIN: Self = Decimal::MIN;
    const MAX: Self = Decimal::MAX;
}

impl HasMinAndMax for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
}

impl HasMinAndMax for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
}

impl HasMinAndMax for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl HasMinAndMax for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl<T> HasMinAndMax for U16<T> {
    const MIN: Self = U16::MIN;
    const MAX: Self = U16::MAX;
}

impl<T> HasMinAndMax for I16<T> {
    const MIN: Self = I16::MIN;
    const MAX: Self = I16::MAX;
}

pub fn try_new_bounded<T>(val: T, min: T, max: T) -> Option<T>
where
    T: Ord + HasMinAndMax,
{
    if !in_range_inclusive(&val, Some(&min), Some(&max)) {
        None
    } else {
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastnum::dec64;

    #[test]
    fn test_in_range_inclusive_u8() {
        assert_eq!(in_range_inclusive(&u8::MIN, None, None), true);
        assert_eq!(in_range_inclusive(&u8::MAX, None, None), true);
        assert_eq!(in_range_inclusive(&1u8, Some(&1u8), Some(&1u8)), true);
        assert_eq!(in_range_inclusive(&2u8, Some(&1u8), Some(&1u8)), false);
        assert_eq!(in_range_inclusive(&0u8, Some(&1u8), Some(&1u8)), false);
    }

    #[test]
    fn test_in_range_inclusive_decimal() {
        assert_eq!(in_range_inclusive(&Decimal::MIN, None, None), true);
        assert_eq!(in_range_inclusive(&Decimal::MAX, None, None), true);
        assert_eq!(
            in_range_inclusive(&dec64!(23.0), Some(&dec64!(22.9)), Some(&dec64!(23.1))),
            true
        );
        assert_eq!(
            in_range_inclusive(&Decimal::NEG_INFINITY, None, None),
            false
        );
        assert_eq!(in_range_inclusive(&Decimal::INFINITY, None, None), false);
    }
}
