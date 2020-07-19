use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone, Default)]
pub struct ResourceVersion {
    pub major: usize,
    pub minor: usize,
}

impl FromStr for ResourceVersion {
    type Err = ResourceVersionParseErr;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        let mut output = ResourceVersion::default();
        let mut two_ints = v.split('.');

        // parse the major
        if let Some(major_str) = two_ints.next() {
            output.major = major_str
                .parse()
                .map_err(ResourceVersionParseErr::ParseMajor)?;
        } else {
            panic!("is this even possible?")
        }

        // parse the minor
        if let Some(minor_str) = two_ints.next() {
            output.minor = minor_str
                .parse()
                .map_err(ResourceVersionParseErr::ParseMinor)?;
        } else {
            return Err(ResourceVersionParseErr::NoPeriodSeparators);
        }

        // confirm we're done
        if two_ints.next().is_some() {
            return Err(ResourceVersionParseErr::TooManyPeriodSeparators);
        }

        Ok(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceVersionParseErr {
    NoPeriodSeparators,
    TooManyPeriodSeparators,
    ParseMajor(std::num::ParseIntError),
    ParseMinor(std::num::ParseIntError),
}

impl std::error::Error for ResourceVersionParseErr {}
impl std::fmt::Display for ResourceVersionParseErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceVersionParseErr::NoPeriodSeparators => {
                write!(f, "invalid ResourceVersion. need exactly 1 '.', like '1.0'")?;
            }
            ResourceVersionParseErr::TooManyPeriodSeparators => {
                write!(
                    f,
                    "invalid ResourceVersion. too many '.' -- use exactly 1 like '1.0'"
                )?;
            }
            ResourceVersionParseErr::ParseMajor(e) => {
                write!(f, "major version error: {}", e)?;
            }
            ResourceVersionParseErr::ParseMinor(e) => {
                write!(f, "minor version error: {}", e)?;
            }
        }

        Ok(())
    }
}

impl Serialize for ResourceVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let val = format!("{}.{}", self.major, self.minor);
        serializer.serialize_str(&val)
    }
}

impl<'de> Deserialize<'de> for ResourceVersion {
    fn deserialize<D>(deserializer: D) -> Result<ResourceVersion, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, Visitor};
        struct ResourceVersionVisitor;

        impl<'de> Visitor<'de> for ResourceVersionVisitor {
            type Value = ResourceVersion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a string of positives integers "1.0" or "9.2""#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(ResourceVersionVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_parse() {
        let resource_version = ResourceVersion { major: 1, minor: 0 };
        let val: ResourceVersion = "1.0".parse().unwrap();

        assert_eq!(val, resource_version);
    }

    #[test]
    fn parse_errors() {
        assert!("1.0".parse::<ResourceVersion>().is_ok());
        assert!("1".parse::<ResourceVersion>().is_err());
        assert!("1.0.0".parse::<ResourceVersion>().is_err());
        assert!(".0".parse::<ResourceVersion>().is_err());
        assert!("-1.0".parse::<ResourceVersion>().is_err());
        assert!("18446744073709551615.0".parse::<ResourceVersion>().is_ok());
        assert!("18446744073709551616.0".parse::<ResourceVersion>().is_err());
    }

    #[test]
    fn basic_serialization() {
        let resource_version = ResourceVersion { major: 1, minor: 0 };
        let val = serde_json::to_string(&resource_version).unwrap();
        assert_eq!(val, "\"1.0\"");
    }

    #[test]
    fn basic_deserialization() {
        let resource_version = ResourceVersion { major: 1, minor: 0 };
        let val: ResourceVersion = serde_json::from_str("\"1.0\"").unwrap();

        assert_eq!(val, resource_version);
    }

    #[test]
    fn deserialization_errors() {
        assert!(serde_json::from_str::<ResourceVersion>("\"9.0.0\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\"-9.0\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\"1.-1\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\".0\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\"0.\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\".\"").is_err());
        assert!(serde_json::from_str::<ResourceVersion>("\"1\"").is_err());
    }
}
