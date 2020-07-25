use regex::Regex;
use std::borrow::Cow;

/// The regex used by the trailing comma utility.
pub const TRAILING_COMMA_REGEX: &str = r"(,)([\s\n]+)?([},\]])";

/// here?
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn trivial_trailing_commas() {
        let input = TrailingCommaUtility::clear_trailing_comma_once(&"{member,}");
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
            include_str!("../../data/trailing_comma/sprite_trailing.yy"),
            include_str!("../../data/trailing_comma/sprite_no_trailing.yy"),
        );
    }

    fn test_harness(tcu: &mut TrailingCommaUtility, input: &str, output: &str) {
        assert_eq!(tcu.clear_trailing_comma(&input), output);
    }
}
