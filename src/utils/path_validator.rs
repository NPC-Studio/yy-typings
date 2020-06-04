use regex::Regex;

pub const GMS2_VALID_PATH_REGEX: &'static str = r"[A-z]\w+";

#[derive(Debug, Clone)]
pub struct PathValidator {
    regex: Regex,
}

impl PathValidator {
    pub fn new() -> PathValidator {
        PathValidator {
            regex: Regex::new(GMS2_VALID_PATH_REGEX).unwrap(),
        }
    }

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
