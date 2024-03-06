use crate::{UncGas, UncGasError, ONE_GIGA_GAS};

/// UncGas Display implementation rounds up the gas usage to the relevant precision point.
/// There are 4 breakpoints:
/// 1. exactly 0 Tgas
/// 2. <0.001 Tgas
/// 3. 0.001 - 0.999 Tgas (uses 3 digits after the floating point)
/// 4. >1 Tgas (uses 1 digit after the floating point)
impl std::fmt::Display for UncGas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == UncGas::from_gas(0) {
            write!(f, "0 Tgas")
        } else if *self < UncGas::from_ggas(1) {
            write!(f, "<0.001 Tgas")
        } else if *self <= UncGas::from_ggas(999) {
            let gigagas_rounded_up = self.as_gas().saturating_add(ONE_GIGA_GAS - 1) / ONE_GIGA_GAS;
            write!(f, "0.{:03} Tgas", gigagas_rounded_up)
        } else {
            let terragas_rounded_up =
                self.as_gas().saturating_add(100 * ONE_GIGA_GAS - 1) / ONE_GIGA_GAS / 100;
            write!(
                f,
                "{}.{} Tgas",
                terragas_rounded_up / 10,
                terragas_rounded_up % 10
            )
        }
    }
}

impl std::fmt::Display for UncGasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UncGasError::IncorrectNumber(err) => write!(f, "Incorrect number: {:?}", err),
            UncGasError::IncorrectUnit(err) => write!(f, "Incorrect unit: {}", err),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::UncGas;

    #[test]
    fn test_display() {
        for (unc_gas, expected_display) in [
            (UncGas::from_gas(0), "0 Tgas"),
            (UncGas::from_gas(1), "<0.001 Tgas"),
            (UncGas::from_gas(999_999_999), "<0.001 Tgas"),
            (UncGas::from_gas(1_000_000_000), "0.001 Tgas"),
            (UncGas::from_gas(1_000_000_001), "0.002 Tgas"),
            (UncGas::from_gas(2_000_000_000), "0.002 Tgas"),
            (UncGas::from_gas(200_000_000_000), "0.200 Tgas"),
            (UncGas::from_gas(999_000_000_000), "0.999 Tgas"),
            (UncGas::from_gas(999_000_000_001), "1.0 Tgas"),
            (UncGas::from_gas(999_999_999_999), "1.0 Tgas"),
            (UncGas::from_gas(1_000_000_000_000), "1.0 Tgas"),
            (UncGas::from_gas(1_000_000_000_001), "1.1 Tgas"),
            (UncGas::from_gas(1_234_567_000_000), "1.3 Tgas"),
            (UncGas::from_gas(1_500_000_000_000), "1.5 Tgas"),
            (UncGas::from_gas(10_000_000_000_000), "10.0 Tgas"),
            (UncGas::from_gas(10_500_000_000_000), "10.5 Tgas"),
            (UncGas::from_gas(99_999_999_999_999), "100.0 Tgas"),
            (UncGas::from_gas(100_000_000_000_000), "100.0 Tgas"),
            (UncGas::from_gas(100_500_000_000_000), "100.5 Tgas"),
            (UncGas::from_gas(1_000_500_000_000_000), "1000.5 Tgas"),
            (
                UncGas::from_gas(1_000_000_500_000_000_000),
                "1000000.5 Tgas",
            ),
        ] {
            assert_eq!(
                unc_gas.to_string(),
                expected_display,
                "gas: {}",
                unc_gas.as_gas()
            );
        }
    }
}
