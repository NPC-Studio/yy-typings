use regex::Regex;

pub const GMS2_VALID_PATH_REGEX: &str = r"[A-z_]\w+";

/// Provides validation services for Gms2 Resource Names by running the Regex
/// of `r"[A-z_]\w+"` over the input. For those who don't speak Regex, it passes for
/// any input which contains *only* `A-z`, `_`, or `0-9` and which does not begin with `0-9`.
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
pub struct PathValidator {
    regex: Regex,
}

impl Default for PathValidator {
    fn default() -> Self {
        PathValidator::new()
    }
}

impl PathValidator {
    /// Create a new `PathValidator`, and initialize its regex. Use this stateful
    /// struct if you are going to be validating multiple paths. If only validating one path,
    /// feel free to use `PathValidator::clear_trailing_comma_once`.
    pub fn new() -> PathValidator {
        PathValidator {
            regex: Regex::new(GMS2_VALID_PATH_REGEX).unwrap(),
        }
    }

    /// Checks if the path given is a valid name for Gms2 Resources.
    pub fn is_valid(&self, input: &str) -> bool {
        Self::clear_trailing_comma_internal(input, &self.regex)
    }

    /// This function clears a trailing comma from a JSON. It is relatively inefficent, and
    /// does one allocations per call.
    pub fn clear_trailing_comma_once(input: &str) -> bool {
        let re = Regex::new(GMS2_VALID_PATH_REGEX).unwrap();

        Self::clear_trailing_comma_internal(input, &re)
    }

    fn clear_trailing_comma_internal(input: &str, re: &Regex) -> bool {
        re.is_match(input)
    }
}
