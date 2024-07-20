use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;

use crate::AprsError;

#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct Callsign {
    pub call: String,
    pub ssid: Option<u8>,
}

impl Callsign {
    pub fn new<T: Into<String>>(call: T, ssid: Option<u8>) -> Callsign {
        Callsign {
            call: call.into(),
            ssid,
        }
    }
}

impl FromStr for Callsign {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let delimiter = s.find('-'); //.ok_or_else(|| AprsError::EmptyCallsign(s.to_owned()))?;
        if delimiter.is_none() {
            return Ok(Callsign::new(s, None));
        }

        let delimiter = delimiter.unwrap();
        if delimiter == 0 {
            return Err(AprsError::EmptyCallsign(s.to_owned()));
        }

        let (call, rest) = s.split_at(delimiter);
        let part = &rest[1..rest.len()];

        if part.is_empty() {
            return Err(AprsError::EmptySSID(s.to_owned()));
        }

        let ssid = part.parse::<u8>().ok();

        if ssid.is_some() {
            Ok(Callsign::new(call, ssid))
        } else {
            Err(AprsError::InvalidSSID(s.to_owned()))
        }
    }
}

impl Display for Callsign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.call)?;

        if let Some(ssid) = &self.ssid {
            write!(f, "-{}", ssid)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_callsign() {
        assert_eq!("ABCDEF".parse(), Ok(Callsign::new("ABCDEF", None)));
    }

    #[test]
    fn parse_with_ssid() {
        assert_eq!("ABCDEF-42".parse(), Ok(Callsign::new("ABCDEF", Some(42))));
    }

    #[test]
    fn empty_callsign() {
        assert_eq!(
            "-42".parse::<Callsign>(),
            Err(AprsError::EmptyCallsign("-42".to_owned()))
        );
    }

    #[test]
    fn empty_ssid() {
        assert_eq!(
            "ABCDEF-".parse::<Callsign>(),
            Err(AprsError::EmptySSID("ABCDEF-".to_owned()))
        );
    }

    #[test]
    fn invalid_ssid() {
        assert_eq!(
            "D-EKDF".parse::<Callsign>(),
            Err(AprsError::InvalidSSID("D-EKDF".to_owned()))
        );
    }
}
