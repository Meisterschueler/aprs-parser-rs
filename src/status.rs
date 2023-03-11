//! A Status Report announces the station's current mission or any other single
//! line status to everyone. The report starts with the '>' APRS Data Type Identifier.
//! The report may optionally contain a timestamp.
//!
//! Examples:
//! - ">12.6V 0.2A 22degC"              (report without timestamp)
//! - ">120503hFatal error"             (report with timestamp in HMS format)
//! - ">281205zSystem will shutdown"    (report with timestamp in DHM format)

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::AprsError;
use crate::Timestamp;

#[derive(PartialEq, Debug, Clone)]
pub struct AprsStatus {
    pub timestamp: Option<Timestamp>,
    pub comment: String,
}

impl FromStr for AprsStatus {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        // Interpret the first 7 bytes as a timestamp, if valid.
        // Otherwise the whole field is the comment.
        let timestamp = if s.len() >= 7 {
            s[0..7].parse::<Timestamp>().ok()
        } else {
            None
        };
        let comment = if timestamp.is_some() { &s[7..] } else { s };

        Ok(AprsStatus {
            timestamp,
            comment: comment.to_owned(),
        })
    }
}

impl Display for AprsStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, ">")?;

        if let Some(ts) = &self.timestamp {
            write!(f, "{}", ts)?;
        }
        write!(f, "{}", self.comment)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_without_timestamp_or_comment() {
        let result = "".parse::<AprsStatus>().unwrap();
        assert_eq!(result.timestamp, None);
        assert_eq!(result.comment, "");
    }

    #[test]
    fn parse_with_timestamp_without_comment() {
        let result = "312359z".parse::<AprsStatus>().unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::DDHHMM(31, 23, 59)));
        assert_eq!(result.comment, "");
    }

    #[test]
    fn parse_without_timestamp_with_comment() {
        let result = "Hi there!".parse::<AprsStatus>().unwrap();
        assert_eq!(result.timestamp, None);
        assert_eq!(result.comment, "Hi there!");
    }

    #[test]
    fn parse_with_timestamp_and_comment() {
        let result = "235959hHi there!".parse::<AprsStatus>().unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::HHMMSS(23, 59, 59)));
        assert_eq!(result.comment, "Hi there!");
    }
}