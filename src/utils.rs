use regex::Regex;

pub const GMS2_VALID_PATH_REGEX: &str = r"^[A-z_]\w*$";
use std::borrow::Cow;

/// The regex used by the trailing comma utility.
pub const TRAILING_COMMA_REGEX: &str = r"(,)([\s\n]+)?([},\]])";

/// Provides validation services for Gms2 Resource Names by running the Regex
/// of `r"[A-z_]\w+"` over the input. For those who don't speak Regex, it passes
/// for any input which contains *only* `A-z`, `_`, or `0-9` and which does not
/// begin with `0-9`.
///
/// Passes:
/// - `spr_player`
/// - `_abc`
/// - `_a12bc`
///
/// Fails:
/// - `0123abc`
/// - `9_`
/// - `5`
#[derive(Debug, Clone)]
pub struct ResourceNameValidator(Regex);

impl ResourceNameValidator {
    /// Create a new `PathValidator`, and initialize its regex. Use this
    /// stateful struct if you are going to be validating multiple paths. If
    /// only validating one path, feel free to use
    /// `PathValidator::validate_path_once`.
    pub fn new() -> ResourceNameValidator {
        ResourceNameValidator(Regex::new(GMS2_VALID_PATH_REGEX).unwrap())
    }

    /// Checks if the path given is a valid name for Gms2 Resources.
    pub fn is_valid(&self, input: &str) -> bool {
        Self::validate_path_internal(input, &self.0)
    }

    /// Checks if path is a valid name for Gms2 Resources.
    pub fn validate_path_once(input: &str) -> bool {
        let re = Regex::new(GMS2_VALID_PATH_REGEX).unwrap();

        Self::validate_path_internal(input, &re)
    }

    fn validate_path_internal(input: &str, re: &Regex) -> bool {
        re.is_match(input)
    }
}

impl Default for ResourceNameValidator {
    fn default() -> Self {
        ResourceNameValidator::new()
    }
}

/// Clears trailing commas from JSON.
#[derive(Debug, Clone)]
pub struct TrailingCommaUtility(Regex);

impl TrailingCommaUtility {
    pub fn new() -> TrailingCommaUtility {
        TrailingCommaUtility(Regex::new(TRAILING_COMMA_REGEX).unwrap())
    }

    pub fn clear_trailing_comma<'a>(&self, input: &'a str) -> Cow<'a, str> {
        Self::clear_trailing_comma_internal(input, &self.0)
    }

    /// This function clears a trailing comma from a JSON. It is relatively
    /// inefficent, and does one allocations per call.
    pub fn clear_trailing_comma_once(input: &str) -> Cow<'_, str> {
        let re = Regex::new(TRAILING_COMMA_REGEX).unwrap();

        Self::clear_trailing_comma_internal(input, &re)
    }

    fn clear_trailing_comma_internal<'a>(input: &'a str, re: &Regex) -> Cow<'a, str> {
        re.replace_all(input, |caps: &regex::Captures<'_>| {
            format!(
                "{}{}",
                &caps
                    .get(2)
                    .map(|matches| matches.as_str())
                    .unwrap_or_else(|| ""),
                &caps[3]
            )
        })
    }
}

impl Default for TrailingCommaUtility {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VersionStamp<const N: u8>;

impl<const N: u8> serde::Serialize for VersionStamp<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if N == 0 {
            serializer.serialize_str("")
        } else {
            let output_bytes: [u8; 2] = [b"v"[0], char::from_digit(N as u32, 10).unwrap() as u8];
            serializer.serialize_str(std::str::from_utf8(&output_bytes).unwrap())
        }
    }
}

impl<'de, const N: u8> serde::Deserialize<'de> for VersionStamp<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<const N: u8>;
        impl<'de, const N: u8> serde::de::Visitor<'de> for Visitor<N> {
            type Value = VersionStamp<N>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a string starting with `v`, or nothing")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.is_empty() {
                    if N == 0 {
                        Ok(VersionStamp)
                    } else {
                        Err(E::custom("expected an empty string `\"\"`"))
                    }
                } else if let Some(sub) = v.strip_prefix('v') {
                    if sub.len() == 1 {
                        let version_stamp = sub.as_bytes()[0] - ('0' as u32 as u8);

                        if version_stamp == N {
                            Ok(VersionStamp)
                        } else {
                            Err(E::custom(format!(
                                "expected version `{}`, got version `{}`",
                                N, version_stamp
                            )))
                        }
                    } else {
                        Err(E::custom("expected a version between 0-9"))
                    }
                } else {
                    Err(E::custom("expected a string starting with `v`"))
                }
            }
            // Similar for other methods:
            //   - visit_i16
            //   - visit_u8
            //   - visit_u16
            //   - visit_u32
            //   - visit_u64
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn once_validation() {
        assert!(ResourceNameValidator::validate_path_once("abcAbc"));
        assert!(ResourceNameValidator::validate_path_once("Abcabc"));
        assert!(ResourceNameValidator::validate_path_once("_Abcabc"));
        assert!(ResourceNameValidator::validate_path_once("_abcAbc"));
        assert!(ResourceNameValidator::validate_path_once("___"));
        assert!(!ResourceNameValidator::validate_path_once("123"));
        assert!(!ResourceNameValidator::validate_path_once("123abc"));
        assert!(!ResourceNameValidator::validate_path_once("123__"));
        assert!(ResourceNameValidator::validate_path_once("__123"));
        assert!(ResourceNameValidator::validate_path_once("__123abc"));
    }

    #[test]
    fn path_validation() {
        let path_validator = ResourceNameValidator::new();

        assert!(path_validator.is_valid("abcAbc"));
        assert!(path_validator.is_valid("Abcabc"));
        assert!(path_validator.is_valid("_Abcabc"));
        assert!(path_validator.is_valid("_abcAbc"));
        assert!(path_validator.is_valid("___"));
        assert!(!path_validator.is_valid("123"));
        assert!(!path_validator.is_valid("123abc"));
        assert!(!path_validator.is_valid("123__"));
        assert!(path_validator.is_valid("__123"));
        assert!(path_validator.is_valid("__123abc"));
    }

    #[test]
    fn trivial_trailing_commas() {
        let input = TrailingCommaUtility::clear_trailing_comma_once("{member,}");
        assert_eq!(input, "{member}");
    }

    #[test]
    fn trailing_commas_test() {
        let mut tcu = TrailingCommaUtility::new();
        test_harness(&mut tcu, "{member,}", "{member}");
        test_harness(&mut tcu, "{member, }", "{member }");
        test_harness(&mut tcu, "{member ,}", "{member }");
        test_harness(&mut tcu, "{member\t\n,\n\t  \t}", "{member\t\n\n\t  \t}");
        test_harness(&mut tcu, "{{member},}", "{{member}}");
        test_harness(&mut tcu, "{member}", "{member}");

        test_harness(
            &mut tcu,
            include_str!("../data/trailing_comma/sprite_trailing.yy"),
            include_str!("../data/trailing_comma/sprite_no_trailing.yy"),
        );
    }

    fn test_harness(tcu: &mut TrailingCommaUtility, input: &str, output: &str) {
        assert_eq!(tcu.clear_trailing_comma(input), output);
    }
}
