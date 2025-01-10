// Define a new `SaturatingU16` type.
//   CHECK It should hold a `u16` value.
//   CHECK It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   CHECK It should support addition with a right-hand side of type
//          SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//          maximum value for `u16`.
//   CHECK It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   CHECK It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}

// Don't need to specify constructor!
// impl SaturatingU16 {
//     pub fn new (value: u16) -> Self {
//         SaturatingU16 {
//             value 
//         }
//     }
// }

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: Self::Output) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(other.value)
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: &Self::Output) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(other.value)
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(other)
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, other: &u16) -> Self::Output {
        self.add(*other)
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {
            value
        }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into()
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self::from(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self::from(*value)
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
