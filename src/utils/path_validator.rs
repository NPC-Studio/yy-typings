use regex::Regex;

pub const GMS2_VALID_PATH_REGEX: &str = r"^[A-z_]\w+";

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
    /// feel free to use `PathValidator::validate_path_once`.
    pub fn new() -> PathValidator {
        PathValidator {
            regex: Regex::new(GMS2_VALID_PATH_REGEX).unwrap(),
        }
    }

    /// Checks if the path given is a valid name for Gms2 Resources.
    pub fn is_valid(&self, input: &str) -> bool {
        Self::validate_path_internal(input, &self.regex)
    }

    /// Checks if path is a valid name for Gms2 Resources. It is relatively inefficent, and
    /// does one allocations per call.
    pub fn validate_path_once(input: &str) -> bool {
        let re = Regex::new(GMS2_VALID_PATH_REGEX).unwrap();

        Self::validate_path_internal(input, &re)
    }

    fn validate_path_internal(input: &str, re: &Regex) -> bool {
        re.is_match(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn once_validation() {
        assert!(PathValidator::validate_path_once("abcAbc"));
        assert!(PathValidator::validate_path_once("Abcabc"));
        assert!(PathValidator::validate_path_once("_Abcabc"));
        assert!(PathValidator::validate_path_once("_abcAbc"));
        assert!(PathValidator::validate_path_once("___"));
        assert!(PathValidator::validate_path_once("123") == false);
        assert!(PathValidator::validate_path_once("123abc") == false);
        assert!(PathValidator::validate_path_once("123__") == false);
        assert!(PathValidator::validate_path_once("__123"));
        assert!(PathValidator::validate_path_once("__123abc"));
    }

    #[test]
    fn path_validation() {
        let path_validator = PathValidator::new();

        assert!(path_validator.is_valid("abcAbc"));
        assert!(path_validator.is_valid("Abcabc"));
        assert!(path_validator.is_valid("_Abcabc"));
        assert!(path_validator.is_valid("_abcAbc"));
        assert!(path_validator.is_valid("___"));
        assert!(path_validator.is_valid("123") == false);
        assert!(path_validator.is_valid("123abc") == false);
        assert!(path_validator.is_valid("123__") == false);
        assert!(path_validator.is_valid("__123"));
        assert!(path_validator.is_valid("__123abc"));
    }
}
