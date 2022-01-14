use std::str::FromStr;

use AprsError;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum DataExtension {
    CourseSpeed(u16, u16),
    StationPower(u8, u8, u8, u8),
    RadioRange(u16),
    SignalStrength(u8, u8, u8, u8),
}

impl FromStr for DataExtension {
    type Err = AprsError;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        // check for course and speed (CSE/SPD)
        if input[0..3].chars().all(|c| c.is_numeric())
            && input.chars().nth(3) == Some('/')
            && input[4..7].chars().all(|c| c.is_numeric())
        {
            let course = input[0..3].parse::<u16>().unwrap();
            if course > 360 {
                return Err(AprsError::InvalidCourse(input.to_owned()));
            }
            let speed = input[4..7].parse::<u16>().unwrap();
            Ok(DataExtension::CourseSpeed(course, speed))
        }
        // check for station power and effective antenna height/gain/directivity
        else if input.starts_with("PHG") && input[3..7].chars().all(|c| c.is_numeric()) {
            let phgd = input[3..7]
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<_>>();
            Ok(DataExtension::StationPower(
                phgd[0], phgd[1], phgd[2], phgd[3],
            ))
        }
        // check for pre-calculated radio range
        else if input.starts_with("RNG") && input[3..7].chars().all(|c| c.is_numeric()) {
            let rrrr = input[3..7].to_string().parse::<u16>().unwrap();
            Ok(DataExtension::RadioRange(rrrr))
        }
        // check for signal strength and effective antenna height/gain/directivity
        else if input.starts_with("DFS") && input[3..7].chars().all(|c| c.is_numeric()) {
            let shgd = input[3..7]
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<_>>();
            Ok(DataExtension::SignalStrength(
                shgd[0], shgd[1], shgd[2], shgd[3],
            ))
        } else {
            Err(AprsError::InvalidDataExtension(input.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_course_speed() {
        assert_eq!("012/345".parse(), Ok(DataExtension::CourseSpeed(12, 345)));
    }

    #[test]
    fn parse_station_power_and_effective_antenna_height_gain_directivity() {
        assert_eq!(
            "PHG0123".parse(),
            Ok(DataExtension::StationPower(0, 1, 2, 3))
        );
    }

    #[test]
    fn parse_pre_calculated_radio_range() {
        assert_eq!("RNG0050".parse(), Ok(DataExtension::RadioRange(50)));
    }

    #[test]
    fn parse_signal_strength_and_effective_antenna_height_gain_directivity() {
        assert_eq!(
            "DFS4567".parse(),
            Ok(DataExtension::SignalStrength(4, 5, 6, 7))
        );
    }

    #[test]
    fn parse_invalid_course() {
        assert_eq!(
            "361/010".parse::<DataExtension>(),
            Err(AprsError::InvalidCourse("361/010".to_owned()))
        );
    }

    #[test]
    fn parse_no_data_extension() {
        assert_eq!(
            "yoyobaa".parse::<DataExtension>(),
            Err(AprsError::InvalidDataExtension("yoyobaa".to_owned()))
        );
    }
}
