//! A `UncGas` type to represent a value of Gas.
//!
//! Each `UncGas` is composed of a whole number of Gases.
//! `UncGas` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u64`.
//!
//! # Examples
//! ```
//! use unc_gas::*;
//!
//! let one_tera_gas = UncGas::from_gas(10_u64.pow(12));
//! assert_eq!(one_tera_gas, UncGas::from_tgas(1));
//! assert_eq!(one_tera_gas, UncGas::from_ggas(1000));
//! ```
//!
//! # Crate features
//!
//! * **borsh** (optional) -
//!   When enabled allows `UncGas` to serialized and deserialized by `borsh`.
//!
//! * **serde** (optional) -
//!   When enabled allows `UncGas` to serialized and deserialized by `serde`.
//!
//! * **schemars** (optional) -
//!  Implements `schemars::JsonSchema` for `UncGas`.
//!
//! * **interactive-clap** (optional) -
//!  Implements `interactive_clap::ToCli` for `UncGas`.
mod error;
mod trait_impls;
mod utils;

pub use self::error::UncGasError;
pub use self::utils::DecimalNumberParsingError;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize)
)]
#[cfg_attr(feature = "abi", derive(borsh::BorshSchema))]
#[repr(transparent)]
pub struct UncGas {
    inner: u64,
}

const ONE_TERA_GAS: u64 = 10u64.pow(12);
const ONE_GIGA_GAS: u64 = 10u64.pow(9);

impl UncGas {
    /// Creates a new `UncGas` from the specified number of whole tera Gas.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    ///
    /// let tera_gas = UncGas::from_tgas(5);
    ///
    /// assert_eq!(tera_gas.as_gas(), 5 * 1_000_000_000_000);
    /// ```
    pub const fn from_tgas(mut inner: u64) -> Self {
        inner *= ONE_TERA_GAS;
        Self { inner }
    }

    /// Creates a new `UncGas` from the specified number of whole giga Gas.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    ///
    /// let giga_gas = UncGas::from_ggas(5);
    ///
    /// assert_eq!(giga_gas.as_gas(), 5 * 1_000_000_000);
    /// ```
    pub const fn from_ggas(mut inner: u64) -> Self {
        inner *= ONE_GIGA_GAS;
        Self { inner }
    }

    /// Creates a new `UncGas` from the specified number of whole Gas.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    ///
    /// let gas = UncGas::from_gas(5 * 1_000_000_000_000);
    ///
    /// assert_eq!(gas.as_tgas(), 5);
    /// ```
    pub const fn from_gas(inner: u64) -> Self {
        Self { inner }
    }

    /// Returns the total number of whole Gas contained by this `UncGas`.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// let UncGas = UncGas::from_gas(12345);
    /// assert_eq!(UncGas.as_gas(), 12345);
    /// ```
    pub const fn as_gas(self) -> u64 {
        self.inner
    }

    /// Returns the total number of a whole part of giga Gas contained by this `UncGas`.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// let UncGas = UncGas::from_gas(1 * 1_000_000_000);
    /// assert_eq!(UncGas.as_ggas(), 1);
    /// ```
    pub const fn as_ggas(self) -> u64 {
        self.inner / ONE_GIGA_GAS
    }

    /// Returns the total number of a whole part of tera Gas contained by this `UncGas`.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// let UncGas = UncGas::from_gas(1 * 1_000_000_000_000);
    /// assert_eq!(UncGas.as_tgas(), 1);
    /// ```
    pub const fn as_tgas(self) -> u64 {
        self.inner / ONE_TERA_GAS
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// use std::u64;
    /// assert_eq!(UncGas::from_gas(u64::MAX -2).checked_add(UncGas::from_gas(2)), Some(UncGas::from_gas(u64::MAX)));
    /// assert_eq!(UncGas::from_gas(u64::MAX -2).checked_add(UncGas::from_gas(3)), None);
    /// ```
    pub const fn checked_add(self, rhs: UncGas) -> Option<Self> {
        if let Some(gas) = self.as_gas().checked_add(rhs.as_gas()) {
            Some(Self::from_gas(gas))
        } else {
            None
        }
    }

    /// Checked integer subtraction. Computes self - rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// assert_eq!(UncGas::from_gas(2).checked_sub(UncGas::from_gas(2)), Some(UncGas::from_gas(0)));
    /// assert_eq!(UncGas::from_gas(2).checked_sub(UncGas::from_gas(3)), None);
    /// ```
    pub const fn checked_sub(self, rhs: UncGas) -> Option<Self> {
        if let Some(gas) = self.as_gas().checked_sub(rhs.as_gas()) {
            Some(Self::from_gas(gas))
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes self * rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// use std::u64;
    /// assert_eq!(UncGas::from_gas(2).checked_mul(2), Some(UncGas::from_gas(4)));
    /// assert_eq!(UncGas::from_gas(u64::MAX).checked_mul(2), None)
    pub const fn checked_mul(self, rhs: u64) -> Option<Self> {
        if let Some(gas) = self.as_gas().checked_mul(rhs) {
            Some(Self::from_gas(gas))
        } else {
            None
        }
    }

    /// Checked integer division. Computes self / rhs, returning None if rhs == 0.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// assert_eq!(UncGas::from_gas(10).checked_div(2), Some(UncGas::from_gas(5)));
    /// assert_eq!(UncGas::from_gas(2).checked_div(0), None);
    /// ```
    pub const fn checked_div(self, rhs: u64) -> Option<Self> {
        if let Some(gas) = self.as_gas().checked_div(rhs) {
            Some(Self::from_gas(gas))
        } else {
            None
        }
    }

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// assert_eq!(UncGas::from_gas(5).saturating_add(UncGas::from_gas(5)), UncGas::from_gas(10));
    /// assert_eq!(UncGas::from_gas(u64::MAX).saturating_add(UncGas::from_gas(1)), UncGas::from_gas(u64::MAX));
    /// ```
    pub const fn saturating_add(self, rhs: UncGas) -> UncGas {
        UncGas::from_gas(self.as_gas().saturating_add(rhs.as_gas()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// assert_eq!(UncGas::from_gas(5).saturating_sub(UncGas::from_gas(2)), UncGas::from_gas(3));
    /// assert_eq!(UncGas::from_gas(1).saturating_sub(UncGas::from_gas(2)), UncGas::from_gas(0));
    /// ```
    pub const fn saturating_sub(self, rhs: UncGas) -> UncGas {
        UncGas::from_gas(self.as_gas().saturating_sub(rhs.as_gas()))
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// use std::u64;
    /// assert_eq!(UncGas::from_gas(2).saturating_mul(5), UncGas::from_gas(10));
    /// assert_eq!(UncGas::from_gas(u64::MAX).saturating_mul(2), UncGas::from_gas(u64::MAX));
    /// ```
    pub const fn saturating_mul(self, rhs: u64) -> UncGas {
        UncGas::from_gas(self.as_gas().saturating_mul(rhs))
    }

    /// Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_gas::UncGas;
    /// assert_eq!(UncGas::from_gas(10).saturating_div(2), UncGas::from_gas(5));
    /// assert_eq!(UncGas::from_gas(10).saturating_div(0), UncGas::from_gas(0))
    /// ```
    pub const fn saturating_div(self, rhs: u64) -> UncGas {
        if rhs == 0 {
            return UncGas::from_gas(0);
        }
        UncGas::from_gas(self.as_gas().saturating_div(rhs))
    }
}

#[cfg(test)]
mod test {
    use crate::UncGas;

    #[test]
    fn checked_add_gas() {
        let gas = UncGas::from_gas(u64::MAX - 3);
        let any_gas = UncGas::from_gas(3);
        let more_gas = UncGas::from_gas(4);
        assert_eq!(gas.checked_add(any_gas), Some(UncGas::from_gas(u64::MAX)));
        assert_eq!(gas.checked_add(more_gas), None);
    }

    #[test]
    fn checked_sub_gas() {
        let gas = UncGas::from_gas(3);
        let any_gas = UncGas::from_gas(1);
        let more_gas = UncGas::from_gas(4);
        assert_eq!(gas.checked_sub(any_gas), Some(UncGas::from_gas(2)));
        assert_eq!(gas.checked_sub(more_gas), None);
    }

    #[test]
    fn checked_mul_gas() {
        let gas = UncGas::from_gas(u64::MAX / 10);
        assert_eq!(
            gas.checked_mul(10),
            Some(UncGas::from_gas(u64::MAX / 10 * 10))
        );
        assert_eq!(gas.checked_mul(11), None);
    }

    #[test]
    fn checked_div_gas() {
        let gas = UncGas::from_gas(10);
        assert_eq!(gas.checked_div(2), Some(UncGas::from_gas(5)));
        assert_eq!(gas.checked_div(11), Some(UncGas::from_gas(0)));
        assert_eq!(gas.checked_div(0), None);
    }

    #[test]
    fn saturating_add_gas() {
        let gas = UncGas::from_gas(100);
        let added_gas = UncGas::from_gas(1);
        let another_gas = UncGas::from_gas(u64::MAX);
        assert_eq!(
            gas.saturating_add(added_gas.clone()),
            UncGas::from_gas(101)
        );
        assert_eq!(
            another_gas.saturating_add(added_gas),
            UncGas::from_gas(u64::MAX)
        );
    }

    #[test]
    fn saturating_sub_gas() {
        let gas = UncGas::from_gas(100);
        let rhs_gas = UncGas::from_gas(1);
        let another_gas = UncGas::from_gas(u64::MIN);
        assert_eq!(gas.saturating_sub(rhs_gas.clone()), UncGas::from_gas(99));
        assert_eq!(
            another_gas.saturating_sub(rhs_gas),
            UncGas::from_gas(u64::MIN)
        );
    }

    #[test]
    fn saturating_mul_gas() {
        let gas = UncGas::from_gas(2);
        let rhs = 10;
        let another_gas = u64::MAX;
        assert_eq!(gas.saturating_mul(rhs), UncGas::from_gas(20));
        assert_eq!(gas.saturating_mul(another_gas), UncGas::from_gas(u64::MAX));
    }

    #[test]
    fn saturating_div_gas() {
        let gas = UncGas::from_gas(10);
        let rhs = 2;
        let another_gas = 20;
        assert_eq!(gas.saturating_div(rhs), UncGas::from_gas(5));
        assert_eq!(gas.saturating_div(another_gas), UncGas::from_gas(0));
    }
}
