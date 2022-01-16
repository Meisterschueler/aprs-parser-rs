use std::str::FromStr;

use crate::AprsError;
use crate::AprsPosition;
use crate::AprsStatus;
use crate::Callsign;

#[derive(PartialEq, Debug, Clone)]
pub struct AprsMessage {
    pub from: Callsign,
    pub to: Callsign,
    pub via: Vec<Callsign>,
    pub data: AprsData,
}

impl FromStr for AprsMessage {
    type Err = AprsError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        // split message into header and body
        let header_delimiter = s
            .find(':')
            .ok_or_else(|| AprsError::InvalidMessage(s.to_owned()))?;
        let (header, rest) = s.split_at(header_delimiter);
        let body = &rest[1..];

        // parse header
        let from_delimiter = header
            .find('>')
            .ok_or_else(|| AprsError::InvalidMessage(s.to_owned()))?;
        let (from, rest) = header.split_at(from_delimiter);
        let from = Callsign::from_str(from)?;

        let to_and_via = &rest[1..];
        let to_and_via: Vec<_> = to_and_via.split(',').collect();

        let to = to_and_via
            .first()
            .ok_or_else(|| AprsError::InvalidMessage(s.to_owned()))?;
        let to = Callsign::from_str(to)?;

        let mut via = vec![];
        for v in to_and_via.iter().skip(1) {
            via.push(Callsign::from_str(v)?);
        }

        // parse body
        if let Some(message_type) = body.chars().next() {
            let body = &body[1..];
            let data = match message_type {
                '/' => AprsData::Position(AprsPosition::from_str(body)?),
                '>' => AprsData::Status(AprsStatus::from_str(body)?),
                _ => AprsData::Unknown,
            };

            Ok(AprsMessage {
                from,
                to,
                via,
                data,
            })
        } else {
            Err(AprsError::InvalidMessage(s.to_owned()))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AprsData {
    Position(AprsPosition),
    Status(AprsStatus),
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataExtension;
    use crate::Timestamp;

    #[test]
    fn parse_position() {
        let result = r"ICA3D17F2>APRS,qAS,dl4mea:/074849h4821.61N\01224.49E^322/103/A=003054 !W09! id213D17F2 -039fpm +0.0rot 2.5dB 3e -0.0kHz gps1x1"
            .parse::<AprsMessage>()
            .unwrap();
        assert_eq!(result.from, Callsign::new("ICA3D17F2", None));
        assert_eq!(result.to, Callsign::new("APRS", None));
        assert_eq!(
            result.via,
            vec![Callsign::new("qAS", None), Callsign::new("dl4mea", None),]
        );

        match result.data {
            AprsData::Position(position) => {
                assert_eq!(position.timestamp, Some(Timestamp::HHMMSS(7, 48, 49)));
                assert_relative_eq!(*position.latitude, 48.360166);
                assert_relative_eq!(*position.longitude, 12.408166);
                assert_eq!(position.symbol_table, '\\');
                assert_eq!(position.symbol_code, '^');
                assert_eq!(
                    position.data_extension,
                    Some(DataExtension::CourseSpeed(322, 103))
                );
                assert_eq!(position.altitude, Some(3054));
                assert_eq!(
                    position.comment,
                    " !W09! id213D17F2 -039fpm +0.0rot 2.5dB 3e -0.0kHz gps1x1"
                );
            }
            _ => panic!("Unexpected data type"),
        }
    }

    #[test]
    fn parse_status() {
        let result = r"Cordoba>APRS,TCPIP*,qAC,GLIDERN3:>194847h v0.2.5.ARM CPU:0.4 RAM:755.4/970.8MB NTP:6.7ms/-0.1ppm +45.5C 0/0Acfts[1h] RF:+48+18.3ppm/+3.45dB/+0.4dB@10km[71]/+0.4dB@10km[1/1]"
            .parse::<AprsMessage>()
            .unwrap();
        assert_eq!(result.from, Callsign::new("Cordoba", None));
        assert_eq!(result.to, Callsign::new("APRS", None));
        assert_eq!(
            result.via,
            vec![
                Callsign::new("TCPIP*", None),
                Callsign::new("qAC", None),
                Callsign::new("GLIDERN3", None),
            ]
        );

        match result.data {
            AprsData::Status(status) => {
                assert_eq!(status.timestamp, Some(Timestamp::HHMMSS(19, 48, 47)));
                assert_eq!(
                    status.comment,
                    " v0.2.5.ARM CPU:0.4 RAM:755.4/970.8MB NTP:6.7ms/-0.1ppm +45.5C 0/0Acfts[1h] RF:+48+18.3ppm/+3.45dB/+0.4dB@10km[71]/+0.4dB@10km[1/1]"
                );
            }
            _ => panic!("Unexpected data type"),
        }
    }
}
