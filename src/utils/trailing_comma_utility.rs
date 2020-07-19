use regex::Regex;
use std::borrow::Cow;

pub const TRAILING_COMMA_REGEX: &str = r"(,)([\s\n]+)?([},\]])";

#[derive(Debug, Clone)]
pub struct TrailingCommaUtility {
    regex: Regex,
}

impl Default for TrailingCommaUtility {
    fn default() -> Self {
        TrailingCommaUtility::new()
    }
}

impl TrailingCommaUtility {
    pub fn new() -> TrailingCommaUtility {
        TrailingCommaUtility {
            regex: Regex::new(TRAILING_COMMA_REGEX).unwrap(),
        }
    }

    pub fn clear_trailing_comma<'a>(&self, input: &'a str) -> Cow<'a, str> {
        Self::clear_trailing_comma_internal(input, &self.regex)
    }

    /// This function clears a trailing comma from a JSON. It is relatively inefficent, and
    /// does one allocations per call.
    pub fn clear_trailing_comma_once(input: &str) -> Cow<'_, str> {
        let re = Regex::new(TRAILING_COMMA_REGEX).unwrap();

        Self::clear_trailing_comma_internal(input, &re)
    }

    fn clear_trailing_comma_internal<'a, 'b>(input: &'a str, re: &'b Regex) -> Cow<'a, str> {
        re.replace_all(input, |caps: &regex::Captures| {
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
