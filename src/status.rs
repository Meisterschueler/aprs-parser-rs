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
        // check minimum length
        if s.len() < 7 {
            return Err(AprsError::InvalidStatus(s.to_owned()));
        }

        // parse timestamp
        let timestamp = Some(s[0..7].parse()?);

        let comment = &s[7..s.len()];

        Ok(AprsStatus {
            timestamp,
            comment: comment.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let result = r"194847hcomment".parse::<AprsStatus>().unwrap();
        assert_eq!(result.timestamp, Some(Timestamp::HHMMSS(19, 48, 47)));
        assert_eq!(result.comment, "comment");
    }
}
