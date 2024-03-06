use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::UncGas;

impl Serialize for UncGas {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::Error;
        let mut buf = [0u8; 20];
        let remainder = {
            use std::io::Write;
            let mut w: &mut [u8] = &mut buf;
            write!(w, "{}", self.inner)
                .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
            w.len()
        };
        let len = buf.len() - remainder;

        let s = std::str::from_utf8(&buf[..len])
            .map_err(|err| Error::custom(format!("Failed to serialize: {}", err)))?;
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for UncGas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse::<u64>()
            .map(UncGas::from_gas)
            .map_err(|err| de::Error::custom(err.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::UncGas;

    #[test]
    fn json_ser() {
        fn test_json_ser(val: u64) {
            let gas = UncGas::from_gas(val);
            let ser = serde_json::to_string(&gas).unwrap();
            assert_eq!(ser, format!("\"{}\"", val));
            let de: UncGas = serde_json::from_str(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_json_ser(u64::MAX);
        test_json_ser(8);
        test_json_ser(0);
    }
}
