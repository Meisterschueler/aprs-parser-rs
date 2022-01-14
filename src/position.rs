use std::str::FromStr;

use lonlat::{Latitude, Longitude};
use AprsError;
use Timestamp;

#[derive(PartialEq, Debug, Clone)]
pub struct AprsPosition {
    pub timestamp: Option<Timestamp>,
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub symbol_table: char,
    pub symbol_code: char,
    pub comment: String,
}

impl FromStr for AprsPosition {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        // check minimum length
        if s.len() < 26 {
            return Err(AprsError::InvalidPosition(s.to_owned()));
        }
        
        // parse timestamp
        let timestamp = Some(s[0..7].parse()?);

        // parse position
        let latitude = s[7..15].parse()?;
        let longitude = s[16..25].parse()?;

        // get symbol table ('/' = primary table, '\' = alternate table) and symbol code
        let symbol_table = s.chars().nth(15).unwrap();
        if !"/\\".contains(symbol_table) {
            return Err(AprsError::InvalidSymbolTable(s.to_owned()))
        }
        let symbol_code = s.chars().nth(25).unwrap();

        let comment = &s[26..s.len()];

        Ok(AprsPosition {
            timestamp,
            latitude,
            longitude,
            symbol_table,
            symbol_code,
            comment: comment.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let result = r"074849h4821.61N\01224.49E^322/103/A=003054"
            .parse::<AprsPosition>()
            .unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::HHMMSS(7, 48, 49)));
        assert_relative_eq!(*result.latitude, 48.360166);
        assert_relative_eq!(*result.longitude, 12.408166);
        assert_eq!(result.symbol_table, '\\');
        assert_eq!(result.symbol_code, '^');
        assert_eq!(result.comment, "322/103/A=003054");
    }

    #[test]
    fn parse_bad_symbol_table() {
        let result = r"074849h4821.61N'01224.49E^322/103/A=003054".parse::<AprsPosition>();
        assert_eq!(
            result,
            Err(AprsError::InvalidSymbolTable(
                r"074849h4821.61N'01224.49E^322/103/A=003054".to_owned()
            ))
        );
    }
}
