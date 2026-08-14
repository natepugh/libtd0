pub use fastnum::D64 as Decimal;
use zerocopy::{I16, U16, U32};

pub trait HasMinAndMax {
    const MIN: Self;
    const MAX: Self;
}

impl HasMinAndMax for Decimal {
    const MIN: Self = Decimal::MIN;
    const MAX: Self = Decimal::MAX;
}

impl HasMinAndMax for &Decimal {
    const MIN: Self = &Decimal::MIN;
    const MAX: Self = &Decimal::MAX;
}

impl HasMinAndMax for i16 {
    const MIN: Self = i16::MIN;
    const MAX: Self = i16::MAX;
}

impl HasMinAndMax for &i16 {
    const MIN: Self = &i16::MIN;
    const MAX: Self = &i16::MAX;
}

impl HasMinAndMax for i8 {
    const MIN: Self = i8::MIN;
    const MAX: Self = i8::MAX;
}

impl HasMinAndMax for &i8 {
    const MIN: Self = &i8::MIN;
    const MAX: Self = &i8::MAX;
}

impl HasMinAndMax for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;
}

impl HasMinAndMax for &u16 {
    const MIN: Self = &u16::MIN;
    const MAX: Self = &u16::MAX;
}

impl HasMinAndMax for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;
}

impl HasMinAndMax for &u32 {
    const MIN: Self = &u32::MIN;
    const MAX: Self = &u32::MAX;
}

impl HasMinAndMax for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;
}

impl HasMinAndMax for &u8 {
    const MIN: Self = &u8::MIN;
    const MAX: Self = &u8::MAX;
}

impl<T> HasMinAndMax for U16<T> {
    const MIN: Self = U16::MIN;
    const MAX: Self = U16::MAX;
}

impl<T> HasMinAndMax for &U16<T> {
    const MIN: Self = &U16::MIN;
    const MAX: Self = &U16::MAX;
}

impl<T> HasMinAndMax for U32<T> {
    const MIN: Self = U32::MIN;
    const MAX: Self = U32::MAX;
}

impl<T> HasMinAndMax for &U32<T> {
    const MIN: Self = &U32::MIN;
    const MAX: Self = &U32::MAX;
}

impl<T> HasMinAndMax for I16<T> {
    const MIN: Self = I16::MIN;
    const MAX: Self = I16::MAX;
}

impl<T> HasMinAndMax for &I16<T> {
    const MIN: Self = &I16::MIN;
    const MAX: Self = &I16::MAX;
}
